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

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Cmp,
    Dup,
    Pop(),
    Push(),
    Rot,
    Jump(),
    Jnz(),
    Jzr(),
    Type(),
    Ret,
    Label(),
    Function(),
    Run(),
    Sys,
    Len,
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

fn expect<'a>(tokens: &'a Vec<Token>, i: usize, kind: &str) -> Result<&'a Token, String> {    let t = current(tokens, i).unwrap();
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
            "keyword" => {
                if t.value == TokenValue::Keyword("psh".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let value = &next_token.value;
                    let value = match value {
                        TokenValue::Integer(i) => ValueType::Integer(*i),
                        TokenValue::Float(f) => ValueType::Float(*f),
                        TokenValue::String(s) => {
                            ValueType::String(s.to_string())
                        },
                        TokenValue::Keyword(s) if s == "true" => ValueType::Boolean(true),
                        TokenValue::Keyword(s) if s == "false" => ValueType::Boolean(false),
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
                        kind: InstructionKind::Push(),
                        params: vec![value],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Keyword("jmp".to_string()) {
                    let next_token = next(&tokens,  i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Jump(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Keyword("jnz".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Jnz(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Keyword("jzr".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Jzr(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Keyword("typ".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Type(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Keyword("run".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let func = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Run(),
                        params: vec![ValueType::String(func.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Keyword("alc".to_string()) {
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
                } else if t.value == TokenValue::Keyword("pop".to_string()) {
                    i += 1;
                    let var_name = expect(&tokens, i, "variable")?.value.to_string();

                    i += 1;
                    if vars.iter().find(|&b| b == &var_name).is_none() {
                        vars.push(var_name.clone());
                    }

                    let instruction = Instruction {
                        kind: InstructionKind::Pop(),
                        params: vec![ValueType::String(var_name.clone())],
                    };

                    instrs.push(instruction);
                } else {
                    let kind = match t.value {
                        TokenValue::Keyword(ref s) if s == "add" => InstructionKind::Add,
                        TokenValue::Keyword(ref s) if s == "sub" => InstructionKind::Sub,
                        TokenValue::Keyword(ref s) if s == "mul" => InstructionKind::Mul,
                        TokenValue::Keyword(ref s) if s == "div" => InstructionKind::Div,
                        TokenValue::Keyword(ref s) if s == "mod" => InstructionKind::Mod,
                        TokenValue::Keyword(ref s) if s == "cmp" => InstructionKind::Cmp,
                        TokenValue::Keyword(ref s) if s == "dup" => InstructionKind::Dup,
                        TokenValue::Keyword(ref s) if s == "rot" => InstructionKind::Rot,
                        TokenValue::Keyword(ref s) if s == "ret" => InstructionKind::Ret,
                        TokenValue::Keyword(ref s) if s == "sys" => InstructionKind::Sys,
                        TokenValue::Keyword(ref s) if s == "len" => InstructionKind::Len,
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
                if next(&tokens, i).unwrap().value != TokenValue::Punctuation(":".parse().unwrap()) {
                    return Err("Parser: Expected ':' after label".to_string());
                }

                if labels.get(&t.value.to_string()).is_some() {
                    return Err(format!("Parser: Label {} already exists", t.value.to_string()));
                }

                labels.insert(t.value.to_string(), instrs.len());
                i += 2;
                instrs.push(Instruction {
                    kind: InstructionKind::Label(),
                    params: vec![ValueType::String(t.value.to_string())],
                });
            },
            "function" => {
                if next(&tokens, i).unwrap().value != TokenValue::Punctuation(":".parse().unwrap()) {
                    return Err("Parser: Expected ':' after function".to_string());
                }

                if funcs.get(&t.value.to_string()).is_some() {
                    return Err(format!("Parser: Function {} already exists", t.value.to_string()));
                }

                funcs.insert(t.value.to_string(), instrs.len());
                i += 2;
                instrs.push(Instruction {
                    kind: InstructionKind::Function(),
                    params: vec![ValueType::String(t.value.to_string())],
                });
            },
            _ => {
                Err(format!("Unexpected token: {:?}", t))?;
            }
        }
    }

    if labels.get(".entry").is_none() {
        return Err("Parser: No .entry label found".to_string());
    }

    Ok(ParserRet {
        instrs,
        labels,
        funcs,
    })
}