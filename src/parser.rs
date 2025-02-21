use std::collections::HashMap;
use std::fmt;
use crate::lexer::{Token, TokenValue};

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueType::Integer(i) => write!(f, "{}", i),
            ValueType::Float(fl) => write!(f, "{}", fl),
            ValueType::String(s) => write!(f, "{}", s),
            ValueType::Boolean(b) => write!(f, "{}", b),
        }
    }
}

impl ValueType {
    pub fn to_int(&self) -> Result<i32, String> {
        match self {
            ValueType::Integer(i) => Ok(*i),
            _ => Err("Cannot convert to int".to_string()),
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
    Print,
    Input,
    Jump(),
    Jnz(),
    Jzr(),
    Type(),
    Ret,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub params: Vec<ValueType>,
}

fn current<'a>(tokens: &'a Vec<Token>, i: &usize) -> Option<&'a Token> {
    if *i < tokens.len() {
        Some(&tokens[*i])
    } else {
        None
    }
}

fn next<'a>(tokens: &'a Vec<Token>, i: &mut usize) -> Option<(&'a Token, usize)> {
    *i += 1;
    let tok = current(tokens, i);
    Some((tok?, *i))
}

fn parse_identifier(tokens: &Vec<Token>, i: &mut usize) -> Result<(Instruction, usize), String> {
    let token = current(tokens, i).unwrap();

    if token.value == TokenValue::Identifier("psh".to_string()) {
        let next_token = next(tokens, i).unwrap();
        let value = &next_token.0.value;
        let value = match value {
            TokenValue::Integer(i) => ValueType::Integer(*i),
            TokenValue::Float(f) => ValueType::Float(*f),
            TokenValue::String(s) => ValueType::String(s.to_string()),
            TokenValue::Identifier(s) if s == "true" => ValueType::Boolean(true),
            TokenValue::Identifier(s) if s == "false" => ValueType::Boolean(false),
            _ => {
                return Err("Invalid value".to_string());
            }
        };

        let instruction = Instruction {
            kind: InstructionKind::Push(),
            params: vec![value],
        };

        Ok((instruction, next_token.1))
    } else if token.value == TokenValue::Identifier("jmp".to_string()) {
        let next_token = next(tokens, i).unwrap();
        let label = next_token.0.value.to_string();

        let instruction = Instruction {
            kind: InstructionKind::Jump(),
            params: vec![ValueType::String(label.clone())],
        };

        Ok((instruction, next_token.1))
    } else if token.value == TokenValue::Identifier("jnz".to_string()) {
        let next_token = next(tokens, i).unwrap();
        let label = next_token.0.value.to_string();

        let instruction = Instruction {
            kind: InstructionKind::Jnz(),
            params: vec![ValueType::String(label.clone())],
        };

        Ok((instruction, next_token.1))
    } else if token.value == TokenValue::Identifier("jzr".to_string()) {
        let next_token = next(tokens, i).unwrap();
        let label = next_token.0.value.to_string();

        let instruction = Instruction {
            kind: InstructionKind::Jzr(),
            params: vec![ValueType::String(label.clone())],
        };

        Ok((instruction, next_token.1))
    } else if token.value == TokenValue::Identifier("typ".to_string()) {
        let next_token = next(tokens, i).unwrap();
        let label = next_token.0.value.to_string();

        let instruction = Instruction {
            kind: InstructionKind::Type(),
            params: vec![ValueType::String(label.clone())],
        };

        Ok((instruction, *i))
    } else if token.value == TokenValue::Identifier("prt".to_string()) {
        if let Some((next_token, _)) = next(tokens, i) {
            if next_token.value == TokenValue::Punctuation(",".to_string().parse().unwrap()) {
                if let Some((next_token, _)) = next(tokens, i) {
                    if next_token.value == TokenValue::Identifier("ln".to_string()) {
                        let instruction = Instruction {
                            kind: InstructionKind::Print,
                            params: vec![ValueType::Boolean(true)],
                        };
                        return Ok((instruction, *i));
                    }
                }
            }
        }

        let instruction = Instruction {
            kind: InstructionKind::Print,
            params: vec![],
        };

        Ok((instruction, *i))
    } else {
        let kind = match token.value {
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
            _ => return Err(format!("Invalid instruction: {:?}", token)),
        };

        let instruction = Instruction {
            kind,
            params: vec![],
        };

        Ok((instruction, *i))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<(Vec<Instruction>, HashMap<String, usize>), String> {
    let mut instrs: Vec<Instruction> = Vec::new();
    let mut i = 0;

    let mut labels: HashMap<String, usize> = HashMap::new();

    while i < tokens.len() {
        let t = current(&tokens, &mut i).unwrap();

        match t.kind {
            "identifier" => {
                let parsed = parse_identifier(&tokens, &mut i).expect("Failed to parse identifier");
                instrs.push(parsed.0);
                i = parsed.1 + 1;
            },
            "label" => {
                if next(&tokens, &mut i).unwrap().0.value != TokenValue::Punctuation(":".parse().unwrap()) {
                    return Err("Parser: Expected ':' after label".to_string());
                }
                labels.insert(t.value.to_string(), instrs.len());
                i += 1;
            },
            _ => {
                Err(format!("Unexpected token: {:?}", t))?;
            }
        }
    }

    if labels.get(".entry").is_none() {
        return Err("Parser: No .entry label found".to_string());
    }

    Ok((instrs, labels))
}