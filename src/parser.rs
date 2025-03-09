use std::collections::HashMap;
use std::fmt;
use crate::lexer::{Token, TokenValue};

#[derive(Debug, PartialEq, Clone)]
pub struct Buffer {
    pub name: String,
    pub size: usize,
    pub buffer: Vec<u8>,
    pub ptr: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Buffer(Buffer),
    Variable(String),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::Integer(i) => write!(f, "{}", i),
            ValueType::Float(fl) => write!(f, "{}", fl),
            ValueType::String(s) => write!(f, "{}", s),
            ValueType::Boolean(b) => write!(f, "{}", b),
            ValueType::Buffer(b) => write!(f, "{:?}", b),
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

fn expect<'a>(tokens: &'a Vec<Token>, i: usize, kind: &str) -> Result<&'a Token, String> {
    let t = current(tokens, i).unwrap();
    if t.kind == kind {
        Ok(t)
    } else {
        Err(format!("Expected token of kind {}, got {:?}", kind, t))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<ParserRet, String> {
    let mut instrs: Vec<Instruction> = Vec::new();
    let mut i = 0;

    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut funcs: HashMap<String, usize> = HashMap::new();
    let mut buffers: Vec<Buffer> = Vec::new();
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
                            if let Some(buffer) = buffers.iter().find(|b| &b.name == s) {
                                ValueType::Buffer(buffer.clone())
                            } else {
                                return Err(format!("Buffer {} not found", s));
                            }
                        },
                        TokenValue::Variable(s) => {
                            if vars.iter().find(|&b| b == s).is_none() {
                                return Err(format!("Variable {} not found", s));
                            }

                            ValueType::Variable(s.to_string())
                        },
                        _ => {
                            return Err("Invalid value for psh".to_string());
                        }
                    };

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Psh,
                        params: vec![value],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jmp".to_string()) {
                    i += 1;
                    let label = expect(&tokens, i, "label")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Jmp,
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jnz".to_string()) {
                    i += 1;
                    let label = expect(&tokens, i, "label")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Jnz,
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jzr".to_string()) {
                    i += 1;
                    let label = expect(&tokens, i, "label")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Jzr,
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("typ".to_string()) {
                    i += 1;
                    let ident = expect(&tokens, i, "identifier")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Type,
                        params: vec![ValueType::String(ident.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("run".to_string()) {
                    i += 1;
                    let func = expect(&tokens, i, "function")?.value.to_string();
                    i += 1;

                    let instruction = Instruction {
                        kind: InstructionKind::Run,
                        params: vec![ValueType::String(func.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("alc".to_string()) {
                    i += 1;
                    let buffer_name = expect(&tokens, i, "buffer")?.value.to_string();
                    if buffers.iter().find(|b| &b.name == &buffer_name).is_some() {
                        return Err(format!("Buffer {} already exists", buffer_name));
                    }

                    i += 1;
                    expect(&tokens, i, "punctuation")?;
                    i += 1;
                    let buffer_size = expect(&tokens, i, "integer")?.value.to_string().parse().unwrap();
                    i += 1;

                    let mut buffer = vec![0u8; buffer_size];

                    buffers.push(Buffer {
                        name: buffer_name,
                        size: buffer_size,
                        buffer: buffer.clone(),
                        ptr: buffer.as_mut_ptr() as usize,
                    });

                    let instruction = Instruction {
                        kind: InstructionKind::Alc,
                        params: vec![],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("pop".to_string()) {
                    i += 1;
                    let var_name = expect(&tokens, i, "variable")?.value.to_string();

                    i += 1;
                    if vars.iter().find(|&b| b == &var_name).is_none() {
                        vars.push(var_name.clone());
                    }

                    let instruction = Instruction {
                        kind: InstructionKind::Pop,
                        params: vec![ValueType::String(var_name.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("dlc".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let value = match next_token.kind.to_string().as_str() {
                        "variable" => {
                            let var_name = next_token.value.to_string();
                            if vars.iter().find(|&b| b == &var_name).is_none() {
                                return Err(format!("Variable {} not found", var_name));
                            }
                            ValueType::Variable(var_name)
                        },
                        "buffer" => {
                            let buffer_name = next_token.value.to_string();
                            if let Some(buffer) = buffers.iter().find(|b| &b.name == &buffer_name) {
                                ValueType::Buffer(buffer.clone())
                            } else {
                                return Err(format!("Buffer {} not found", buffer_name));
                            }
                        },
                        _ => return Err("Expected variable or buffer".to_string()),
                    };

                    i += 2;
                    let instruction = Instruction {
                        kind: InstructionKind::Dlc,
                        params: vec![value],
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
                        _ => return Err(format!("Invalid instruction: {:?}", t)),
                    };

                    i += 1;

                    let instruction = Instruction {
                        kind,
                        params: vec![],
                    };

                    instrs.push(instruction);
                }
            },
            "label" => {
                i += 1;
                if expect(&tokens, i,"punctuation")?.value.to_string() != ":" {
                    return Err("Parser: Expected ':' after label".to_string());
                }
                i += 1;

                if labels.get(&t.value.to_string()).is_some() {
                    return Err(format!("Parser: Label {} already exists", t.value.to_string()));
                }

                labels.insert(t.value.to_string(), instrs.len());
                instrs.push(Instruction {
                    kind: InstructionKind::Lbl,
                    params: vec![ValueType::String(t.value.to_string())],
                });
            },
            "function" => {
                i += 1;
                if expect(&tokens, i,"punctuation")?.value.to_string() != ":" {
                    return Err("Parser: Expected ':' after function".to_string());
                }
                i += 1;

                if funcs.get(&t.value.to_string()).is_some() {
                    return Err(format!("Parser: Function {} already exists", t.value.to_string()));
                }

                funcs.insert(t.value.to_string(), instrs.len());
                instrs.push(Instruction {
                    kind: InstructionKind::Fun,
                    params: vec![ValueType::String(t.value.to_string())],
                });
            },
            _ => {
                Err(format!("Unexpected token: {:?}", t))?;
            }
        }
    }

    if funcs.get("@entry").is_none() {
        return Err("Parser: No @entry function found".to_string());
    }

    Ok(ParserRet {
        instrs,
        labels,
        funcs,
    })
}