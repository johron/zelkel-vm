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

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    Push(),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pop,
    Dup,
    Rot,
    Print,
    Input,
    Jump(),
    JumpNZ(),
    Compare,
    Type(),
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
            kind: InstructionKind::JumpNZ(),
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
    } else {
        let kind = match token.value {
            TokenValue::Identifier(ref s) if s == "add" => InstructionKind::Add,
            TokenValue::Identifier(ref s) if s == "sub" => InstructionKind::Sub,
            TokenValue::Identifier(ref s) if s == "mul" => InstructionKind::Mul,
            TokenValue::Identifier(ref s) if s == "div" => InstructionKind::Div,
            TokenValue::Identifier(ref s) if s == "mod" => InstructionKind::Mod,
            TokenValue::Identifier(ref s) if s == "prt" => InstructionKind::Print,
            TokenValue::Identifier(ref s) if s == "inp" => InstructionKind::Input,
            TokenValue::Identifier(ref s) if s == "pop" => InstructionKind::Pop,
            TokenValue::Identifier(ref s) if s == "cmp" => InstructionKind::Compare,
            TokenValue::Identifier(ref s) if s == "dup" => InstructionKind::Dup,
            TokenValue::Identifier(ref s) if s == "rot" => InstructionKind::Rot,
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
            "punctuation" => {
                if t.value == TokenValue::Punctuation(".".parse().unwrap()) {
                    let ident = next(&tokens, &mut i).expect("Failed to get next token");
                    i = ident.1;
                    if next(&tokens, &mut i).unwrap().0.value != TokenValue::Punctuation(":".parse().unwrap()) {
                        return Err("Parser: Expected ':'".to_string());
                    }
                    i += 1;
                    labels.insert(ident.0.value.to_string(), instrs.len());
                } else {
                    return Err("Unexpected token".to_string());
                }
            },
            _ => {
                return Err("Unexpected token".to_string());
            }
        }
    }

    Ok((instrs, labels))
}