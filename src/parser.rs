use std::collections::HashMap;
use std::fmt;
use crate::Error;
use crate::lexer::{Token, TokenValue};

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Buffer(String),
    Variable(String),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::Integer(i) => write!(f, "{}", i),
            ValueType::Float(fl) => write!(f, "{}", fl),
            ValueType::String(s) => write!(f, "{}", s),
            ValueType::Boolean(b) => write!(f, "{}", b),
            ValueType::Buffer(b) => write!(f, "{}", b),
            ValueType::Variable(v) => write!(f, "{}", v),
        }
    }
}

impl ValueType {
    pub fn to_int(&self) -> Result<i32, String> {
        match self {
            ValueType::Integer(i) => Ok(*i),
            ValueType::Float(f) => Ok(*f as i32),
            ValueType::String(s) => s.parse::<i32>().map_err(|_| "Cannot convert string to int".to_string()),
            ValueType::Boolean(b) => Ok(*b as i32),
            ValueType::Buffer(_) => Err("Cannot convert buffer to int".to_string()),
            ValueType::Variable(_) => Err("Cannot convert variable to int".to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Cmp,
    Dup,
    Pop,
    Psh,
    Rot,
    Jmp,
    Jnz,
    Jzr,
    Type,
    Ret,
    Run,
    Sys,
    Len,
    Lbl,
    Fun,
    Dlc,
    Alc,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub params: Vec<ValueType>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct ParserRet {
    pub instrs: Vec<Instruction>,
    pub labels: HashMap<String, usize>,
    pub funcs: HashMap<String, usize>,
}

fn current(tokens: &Vec<Token>, i: usize) -> Option<&Token> {
    if i < tokens.len() {
        Some(&tokens[i])
    } else {
        None
    }
}

fn next(tokens: &Vec<Token>, i: usize) -> Option<&Token> {
    let tok = current(tokens, i + 1);
    Some(tok?)
}

fn expect<'a>(tokens: &'a Vec<Token>, i: usize, kind: &str) -> Result<&'a Token, Error> {
    let t = current(tokens, i).unwrap();
    if t.kind == kind {
        Ok(t)
    } else {
        Err(Error::new(format!("Expected token of kind {}, got {:?}", kind, t), t.line, t.col))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<ParserRet, Error> {
    let mut instrs: Vec<Instruction> = Vec::new();
    let mut i = 0;

    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut funcs: HashMap<String, usize> = HashMap::new();
    let mut bufs: Vec<String> = Vec::new();
    let mut vars: Vec<String> = Vec::new();

    while i < tokens.len() {
        let t = current(&tokens, i).unwrap();

        match t.kind {
            "identifier" => {
                if t.value == TokenValue::Identifier("psh".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let value = &next_token.value;
                    let value = match value {
                        TokenValue::Integer(i) => ValueType::Integer(*i),
                        TokenValue::Float(f) => ValueType::Float(*f),
                        TokenValue::String(s) => ValueType::String(s.clone()),
                        TokenValue::Identifier(s) if s == "true" => ValueType::Boolean(true),
                        TokenValue::Identifier(s) if s == "false" => ValueType::Boolean(false),
                        TokenValue::Buffer(s) => {
                            if bufs.iter().find(|&b| b == s).is_none() {
                                return Err(Error::new(format!("Buffer {} not found", s), t.line, t.col));
                            }
                            ValueType::Buffer(s.to_string())
                        },
                        TokenValue::Variable(s) => {
                            if vars.iter().find(|&b| b == s).is_none() {
                                return Err(Error::new(format!("Variable {} not found", s), t.line, t.col));
                            }
                            ValueType::Variable(s.to_string())
                        },
                        _ => {
                            return Err(Error::new(format!("Invalid value for psh: {:?}", value), t.line, t.col));
                        }
                    };

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Psh,
                        params: vec![value],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jmp".to_string()) {
                    i += 1;
                    let label = expect(&tokens, i, "label")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Jmp,
                        params: vec![ValueType::String(label.clone())],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jnz".to_string()) {
                    i += 1;
                    let label = expect(&tokens, i, "label")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Jnz,
                        params: vec![ValueType::String(label.clone())],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jzr".to_string()) {
                    i += 1;
                    let label = expect(&tokens, i, "label")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Jzr,
                        params: vec![ValueType::String(label.clone())],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("typ".to_string()) {
                    i += 1;
                    let ident = expect(&tokens, i, "identifier")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Type,
                        params: vec![ValueType::String(ident.clone())],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("run".to_string()) {
                    i += 1;
                    let func = expect(&tokens, i, "function")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Run,
                        params: vec![ValueType::String(func.clone())],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("alc".to_string()) {
                    i += 1;
                    let buffer_name = expect(&tokens, i, "buffer")?.value.to_string();
                    if bufs.iter().find(|b| &b == &&&buffer_name).is_some() {
                        return Err(Error::new(format!("Buffer {} already exists", buffer_name), t.line, t.col));
                    }

                    i += 1;
                    expect(&tokens, i, "punctuation")?;
                    i += 1;
                    let buffer_size = expect(&tokens, i, "integer")?.value.to_string().parse().unwrap();
                    i += 1;

                    bufs.push(buffer_name.clone());

                    let instruction = Instruction {
                        kind: InstructionKind::Alc,
                        params: vec![ValueType::Buffer(buffer_name.clone()), ValueType::Integer(buffer_size)],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("pop".to_string()) {
                    i += 1;
                    let var_name = expect(&tokens, i, "variable")?.value.to_string();

                    i += 1;
                    if vars.iter().find(|&b| b == &var_name).is_none() {
                        if var_name != "$_" && var_name != "$" {
                            vars.push(var_name.clone());
                        }
                    }

                    let instruction = Instruction {
                        kind: InstructionKind::Pop,
                        params: vec![ValueType::Variable(var_name.clone())],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("dlc".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let value = match next_token.kind.to_string().as_str() {
                        "variable" => {
                            let var_name = next_token.value.to_string();
                            if vars.iter().find(|&b| b == &var_name).is_none() {
                                return Err(Error::new(format!("Variable {} not found", var_name), t.line, t.col));
                            }
                            vars.remove(vars.iter().position(|x| x == &var_name).unwrap());
                            ValueType::Variable(var_name)
                        },
                        "buffer" => {
                            let buffer_name = next_token.value.to_string();
                            if bufs.iter().find(|&b| b == &buffer_name).is_none() {
                                return Err(Error::new(format!("Buffer {} not found", buffer_name), t.line, t.col));
                            }
                            bufs.remove(bufs.iter().position(|x| x == &buffer_name).unwrap());
                            ValueType::Buffer(buffer_name)
                        },
                        _ => return Err(Error::new("Expected variable or buffer".to_string(), t.line, t.col)),
                    };

                    i += 2;
                    let instruction = Instruction {
                        kind: InstructionKind::Dlc,
                        params: vec![value],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                } else {
                    let kind = match t.value {
                        TokenValue::Identifier(ref s) if s == "add" => InstructionKind::Add,
                        TokenValue::Identifier(ref s) if s == "sub" => InstructionKind::Sub,
                        TokenValue::Identifier(ref s) if s == "mul" => InstructionKind::Mul,
                        TokenValue::Identifier(ref s) if s == "div" => InstructionKind::Div,
                        TokenValue::Identifier(ref s) if s == "mod" => InstructionKind::Mod,
                        TokenValue::Identifier(ref s) if s == "cmp" => InstructionKind::Cmp,
                        TokenValue::Identifier(ref s) if s == "dup" => InstructionKind::Dup,
                        TokenValue::Identifier(ref s) if s == "rot" => InstructionKind::Rot,
                        TokenValue::Identifier(ref s) if s == "ret" => InstructionKind::Ret,
                        TokenValue::Identifier(ref s) if s == "sys" => InstructionKind::Sys,
                        TokenValue::Identifier(ref s) if s == "len" => InstructionKind::Len,
                        _ => return Err(Error::new(format!("Invalid instruction: {:?}", t), t.line, t.col)),
                    };

                    i += 1;

                    let instruction = Instruction {
                        kind,
                        params: vec![],
                        line: t.line,
                        col: t.col,
                    };

                    instrs.push(instruction);
                }
            },
            "label" => {
                i += 1;
                if expect(&tokens, i,"punctuation")?.value.to_string() != ":" {
                    return Err(Error::new("Parser: Expected ':' after label".to_string(), t.line, t.col));
                }
                i += 1;

                if labels.get(&t.value.to_string()).is_some() {
                    return Err(Error::new(format!("Parser: Label {} already exists", t.value.to_string()), t.line, t.col));
                }

                labels.insert(t.value.to_string(), instrs.len());
                instrs.push(Instruction {
                    kind: InstructionKind::Lbl,
                    params: vec![ValueType::String(t.value.to_string())],
                    line: t.line,
                    col: t.col,
                });
            },
            "function" => {
                i += 1;
                if expect(&tokens, i,"punctuation")?.value.to_string() != ":" {
                    return Err(Error::new("Parser: Expected ':' after function".to_string(), t.line, t.col));
                }
                i += 1;

                if funcs.get(&t.value.to_string()).is_some() {
                    return Err(Error::new(format!("Parser: Function {} already exists", t.value.to_string()), t.line, t.col));
                }

                funcs.insert(t.value.to_string(), instrs.len());
                instrs.push(Instruction {
                    kind: InstructionKind::Fun,
                    params: vec![ValueType::String(t.value.to_string())],
                    line: t.line,
                    col: t.col,
                });
            },
            _ => {
                Err(Error::new(format!("Unexpected token: {:?}", t), t.line, t.col))?;
            }
        }
    }

    if funcs.get("@entry").is_none() {
        return Err(Error::new("Parser: No @entry function found".to_string(), 0, 0));
    }

    Ok(ParserRet {
        instrs,
        labels,
        funcs,
    })
}