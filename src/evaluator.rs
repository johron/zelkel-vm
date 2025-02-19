use std::io::Write;
use std::io;
use crate::parser::{Node, NodeKind, ValueType, InstructionKind};

pub fn evaluate(nodes: Vec<Node>) -> Result<Vec<ValueType>, String> {
    let mut stack: Vec<ValueType> = Vec::new();

    for node in nodes {
        match node.kind {
            NodeKind::Instruction(instruction) => {
                match instruction.kind {
                    InstructionKind::Push() => {
                        for param in instruction.params {
                            stack.push(param);
                        }
                    },
                    InstructionKind::Add => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        match (a, b) {
                            (ValueType::Integer(a), ValueType::Integer(b)) => {
                                stack.push(ValueType::Integer(a + b));
                            },
                            (ValueType::Float(a), ValueType::Float(b)) => {
                                stack.push(ValueType::Float(a + b));
                            },
                            (ValueType::String(a), ValueType::String(b)) => {
                                stack.push(ValueType::String(format!("{}{}", b, a)));
                            },
                            _ => return Err("Invalid types for add".to_string()),
                        }
                    },
                    InstructionKind::Sub => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        match (a, b) {
                            (ValueType::Integer(a), ValueType::Integer(b)) => {
                                stack.push(ValueType::Integer(b - a));
                            },
                            (ValueType::Float(a), ValueType::Float(b)) => {
                                stack.push(ValueType::Float(a - b));
                            },
                            _ => return Err("Invalid types for sub".to_string()),
                        }
                    },
                    InstructionKind::Mul => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        match (a, b) {
                            (ValueType::Integer(a), ValueType::Integer(b)) => {
                                stack.push(ValueType::Integer(a * b));
                            },
                            (ValueType::Float(a), ValueType::Float(b)) => {
                                stack.push(ValueType::Float(a * b));
                            },
                            (ValueType::String(a), ValueType::Integer(b)) => {
                                stack.push(ValueType::String(a.repeat(b as usize)));
                            },
                            (ValueType::Integer(a), ValueType::String(b)) => {
                                stack.push(ValueType::String(b.repeat(a as usize)));
                            },
                            _ => return Err("Invalid types for mul".to_string()),
                        }
                    },
                    InstructionKind::Div => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        match (a, b) {
                            (ValueType::Integer(a), ValueType::Integer(b)) => {
                                stack.push(ValueType::Integer(b / a));
                            },
                            (ValueType::Float(a), ValueType::Float(b)) => {
                                stack.push(ValueType::Float(a / b));
                            },
                            _ => return Err("Invalid types for div".to_string()),
                        }
                    },
                    InstructionKind::Mod => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        match (a, b) {
                            (ValueType::Integer(a), ValueType::Integer(b)) => {
                                stack.push(ValueType::Integer(b % a));
                            },
                            (ValueType::Float(a), ValueType::Float(b)) => {
                                stack.push(ValueType::Float(a % b));
                            },
                            _ => return Err("Invalid types for mod".to_string()),
                        }
                    },
                    InstructionKind::Pop => {
                        stack.pop();
                    },
                    InstructionKind::Print => {
                        let a = stack.pop().unwrap();
                        match a {
                            ValueType::Integer(i) => print!("{}", i),
                            ValueType::Float(f) => print!("{}", f),
                            ValueType::String(s) => print!("{}", s.replace("\\n", "\n")),
                            ValueType::Boolean(b) => print!("{}", b),
                            _ => {
                                return Err("Invalid type for print".to_string());
                            }
                        }
                    },
                    InstructionKind::Input => {
                        io::stdout().flush().expect("Failed to flush stdout");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let input = input.trim().to_string();
                        stack.push(ValueType::String(input));
                    }
                }
            },
            NodeKind::Block(nodes) => {
                evaluate(nodes)?;
            },
        }
    }

    Ok(stack)
}