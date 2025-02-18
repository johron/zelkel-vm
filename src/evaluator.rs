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
                            _ => return Err("Invalid types for mod".to_string()),
                        }
                    },
                    InstructionKind::Pop => {
                        stack.pop();
                    },
                }
            },
            NodeKind::Block(nodes) => {
                evaluate(nodes)?;
            },
        }
    }

    Ok(stack)
}