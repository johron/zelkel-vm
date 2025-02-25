use std::collections::HashMap;
use std::fmt;
use crate::lexer::{Token, TokenValue};

#[derive(Debug, PartialEq, Clone)]
pub struct Buffer {
    pub name: String,
    pub size: usize,
    pub ptr: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Buffer(Buffer),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::Integer(i) => write!(f, "{}", i),
            ValueType::Float(fl) => write!(f, "{}", fl),
            ValueType::String(s) => write!(f, "{}", s),
            ValueType::Boolean(b) => write!(f, "{}", b),
            ValueType::Buffer(b) => write!(f, "{:?}", b),
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
    Push(),
    Rot,
    Input,
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
    Alloc,
    Print,
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

fn next(tokens: &Vec<Token>, mut i: usize) -> Option<(&Token)> {
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
    let mut buffers: HashMap<String, i32> = HashMap::new();

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
                        TokenValue::String(s) => ValueType::String(s.to_string().replace("\\n", "\n")),
                        TokenValue::Identifier(s) if s == "true" => ValueType::Boolean(true),
                        TokenValue::Identifier(s) if s == "false" => ValueType::Boolean(false),
                        TokenValue::Buffer(s) => {
                            let buffer_size = buffers.get(s).unwrap();
                            ValueType::Buffer(Buffer {
                                name: s.to_string(),
                                size: *buffer_size as usize,
                                ptr: vec![0u8, *buffer_size as u8].as_mut_ptr() as usize,
                            })
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
                } else if t.value == TokenValue::Identifier("jmp".to_string()) {
                    let next_token = next(&tokens,  i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Jump(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jnz".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Jnz(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("jzr".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Jzr(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("typ".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let label = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Type(),
                        params: vec![ValueType::String(label.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("run".to_string()) {
                    let next_token = next(&tokens, i).unwrap();
                    let func = next_token.value.to_string();

                    i += 2;

                    let instruction = Instruction {
                        kind: InstructionKind::Run(),
                        params: vec![ValueType::String(func.clone())],
                    };

                    instrs.push(instruction);
                } else if t.value == TokenValue::Identifier("alc".to_string()) {
                    i += 1;
                    let buffer_name = expect(&tokens, i, "buffer")?.value.to_string();
                    i += 1;
                    expect(&tokens, i, "punctuation")?;
                    i += 1;
                    let buffer_size = expect(&tokens, i, "integer")?.value.to_string().parse().unwrap();
                    i += 1;

                    buffers.insert(buffer_name, buffer_size);
                } else {
                    let kind = match t.value {
                        TokenValue::Identifier(ref s) if s == "add" => InstructionKind::Add,
                        TokenValue::Identifier(ref s) if s == "sub" => InstructionKind::Sub,
                        TokenValue::Identifier(ref s) if s == "mul" => InstructionKind::Mul,
                        TokenValue::Identifier(ref s) if s == "div" => InstructionKind::Div,
                        TokenValue::Identifier(ref s) if s == "mod" => InstructionKind::Mod,
                        TokenValue::Identifier(ref s) if s == "inp" => InstructionKind::Input,
                        TokenValue::Identifier(ref s) if s == "pop" => InstructionKind::Pop,
                        TokenValue::Identifier(ref s) if s == "cmp" => InstructionKind::Cmp,
                        TokenValue::Identifier(ref s) if s == "dup" => InstructionKind::Dup,
                        TokenValue::Identifier(ref s) if s == "rot" => InstructionKind::Rot,
                        TokenValue::Identifier(ref s) if s == "ret" => InstructionKind::Ret,
                        TokenValue::Identifier(ref s) if s == "sys" => InstructionKind::Sys,
                        TokenValue::Identifier(ref s) if s == "len" => InstructionKind::Len,
                        TokenValue::Identifier(ref s) if s == "prt" => InstructionKind::Print,
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