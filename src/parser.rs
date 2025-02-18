use crate::lexer::{Token, TokenValue};

#[derive(Debug)]
pub enum ValueType {
    Integer(i32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub enum InstructionKind {
    Push(),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pop,
}

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub params: Vec<ValueType>,
}

#[derive(Debug)]
pub enum NodeKind {
    Instruction(Instruction),
    Block(Vec<Node>),
}

#[derive(Debug)]
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
    if *i < tokens.len() {
        *i += 1;
        let tok = &tokens[*i];
        Some((tok, *i))
    } else {
        None
    }
}

fn parse_identifier(tokens: &Vec<Token>, i: &mut usize) -> Result<(Node, usize), String> {
    let token = current(tokens, i).unwrap();

    match token.value {
        TokenValue::Identifier(ref s) => match s.as_str() {
            "push" => {
                let next = next(tokens, i).unwrap();
                *i = next.1;
                let value = match &next.0.value {
                    TokenValue::Integer(i) => ValueType::Integer(*i),
                    TokenValue::Float(f) => ValueType::Float(*f),
                    TokenValue::String(s) => ValueType::String(s.to_string()),
                    _ => {
                        return Err(format!("Unexpected value type: {:?}", next.0.value));
                    }
                };

                Ok(
                    (Node {
                        kind: NodeKind::Instruction(Instruction {
                            kind: InstructionKind::Push(),
                            params: vec![value],
                        }),
                    }, *i)
                )
            },
            _ => {
                Err(format!("Unexpected identifier: {}", s))
            }
        }
        _ => {
            Err(format!("Expected identifier: {:?}", token))
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut i = 0;

    loop {
        println!("{}", i);
        if i >= tokens.len() {
            break;
        }

        let t = current(&tokens, &mut i).unwrap();

        match t.kind {
            "identifier" => {
                let parsed = parse_identifier(&tokens, &mut i).expect("Failed to parse identifier");
                nodes.push(parsed.0);
                i = parsed.1;
            }
            _ => {
                eprintln!("Unexpected token1: {:?}", t);
            }
        }

    }

    nodes
}