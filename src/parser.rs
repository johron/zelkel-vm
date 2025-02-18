use crate::lexer::{Token, TokenValue};

#[derive(Debug, PartialEq)]
pub enum ValueType {
    Integer(i32),
    Float(f32),
    String(String),
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
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub params: Vec<ValueType>,
}

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Instruction(Instruction),
    Block(Vec<Node>),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
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

fn parse_identifier(tokens: &Vec<Token>, i: &mut usize) -> Result<(Node, usize), String> {
    let token = current(tokens, i).unwrap();

    if token.value == TokenValue::Identifier("push".to_string()) {
        let next_token = next(tokens, i).unwrap();
        let value = &next_token.0.value;
        let value = match value {
            TokenValue::Integer(i) => ValueType::Integer(*i),
            TokenValue::Float(f) => ValueType::Float(*f),
            TokenValue::String(s) => ValueType::String(s.to_string()),
            _ => {
                return Err("Invalid value".to_string());
            }
        };

        let instruction = Instruction {
            kind: InstructionKind::Push(),
            params: vec![value],
        };

        let node = Node {
            kind: NodeKind::Instruction(instruction),
        };

        Ok((node, next_token.1))
    } else {
        let kind = match token.value {
            TokenValue::Identifier(ref s) if s == "add" => InstructionKind::Add,
            TokenValue::Identifier(ref s) if s == "sub" => InstructionKind::Sub,
            TokenValue::Identifier(ref s) if s == "mul" => InstructionKind::Mul,
            TokenValue::Identifier(ref s) if s == "div" => InstructionKind::Div,
            TokenValue::Identifier(ref s) if s == "mod" => InstructionKind::Mod,
            _ => return Err("Invalid identifier".to_string()),
        };

        let instruction = Instruction {
            kind,
            params: vec![],
        };

        let node = Node {
            kind: NodeKind::Instruction(instruction),
        };

        Ok((node, *i))
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, String> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let t = current(&tokens, &mut i).unwrap();

        match t.kind {
            "identifier" => {
                let parsed = parse_identifier(&tokens, &mut i).expect("Failed to parse identifier");
                nodes.push(parsed.0);
                i = parsed.1 + 1;
            }
            _ => {
                return Err("Unexpected token".to_string());
            }
        }
    }

    Ok(nodes)
}