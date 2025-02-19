use std::collections::HashMap;
use std::io::Write;
use std::io;
use crate::parser::{ValueType, Instruction, InstructionKind};

pub fn evaluate(instrs: Vec<Instruction>, labels: HashMap<String, usize>) -> Result<Vec<ValueType>, String> {
    let mut stack: Vec<ValueType> = Vec::new();

    for instr in instrs {
        match instr.kind {
            InstructionKind::Push() => {
                for param in instr.params {
                    stack.push(param);
                }
            },
            InstructionKind::Add => {
                let a = stack.pop().ok_or("Stack underflow")?.clone();
                let b = stack.pop().ok_or("Stack underflow")?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

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

                    (ValueType::Integer(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a as f32 + b));
                    },
                    (ValueType::Float(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Float(a + b as f32));
                    },

                    (ValueType::String(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::String(format!("{}{}", b, a)));
                    },
                    (ValueType::Integer(a), ValueType::String(b)) => {
                        stack.push(ValueType::String(format!("{}{}", b, a)));
                    },

                    (ValueType::String(a), ValueType::Float(b)) => {
                        stack.push(ValueType::String(format!("{}{}", b, a)));
                    },
                    (ValueType::Float(a), ValueType::String(b)) => {
                        stack.push(ValueType::String(format!("{}{}", b, a)));
                    },

                    _ => return Err(format!("Invalid types for add {:?} {:?}", a_clone, b_clone)),                        }
            },
            InstructionKind::Sub => {
                let a = stack.pop().ok_or("Stack underflow")?.clone();
                let b = stack.pop().ok_or("Stack underflow")?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Integer(b - a));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a - b));
                    },

                    (ValueType::Integer(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a as f32 - b));
                    },
                    (ValueType::Float(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Float(a - b as f32));
                    },

                    _ => return Err(format!("Invalid types for sub {:?} {:?}", a_clone, b_clone)),
                }
            },
            InstructionKind::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Integer(a * b));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a * b));
                    },

                    (ValueType::Integer(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a as f32 * b));
                    },
                    (ValueType::Float(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Float(a * b as f32));
                    },

                    (ValueType::String(a), ValueType::Integer(b)) | (ValueType::Integer(b), ValueType::String(a)) => {
                        stack.push(ValueType::String(a.repeat(b as usize)));
                    },
                    _ => return Err(format!("Invalid types for mul {:?} {:?}", a_clone, b_clone)),
                }
            },
            InstructionKind::Div => {
                let a = stack.pop().ok_or("Stack underflow")?.clone();
                let b = stack.pop().ok_or("Stack underflow")?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Integer(b / a));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a / b));
                    },

                    (ValueType::Integer(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(b / a as f32));
                    },
                    (ValueType::Float(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Float(a / b as f32));
                    },

                    _ => return Err(format!("Invalid types for div {:?} {:?}", a_clone, b_clone)),
                }
            },
            InstructionKind::Mod => {
                let a = stack.pop().ok_or("Stack underflow")?.clone();
                let b = stack.pop().ok_or("Stack underflow")?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Integer(b % a));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a % b));
                    },

                    (ValueType::Integer(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(b % a as f32));
                    },
                    (ValueType::Float(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Float(a % b as f32));
                    },

                    _ => return Err(format!("Invalid types for mod {:?} {:?}", a_clone, b_clone)),
                }
            },
            InstructionKind::Pop => {
                stack.pop();
            },
            InstructionKind::Print => {
                let a = stack.pop().ok_or("Stack underflow")?.clone();
                let a_clone = a.clone();
                match a {
                    ValueType::Integer(i) => print!("{}", i),
                    ValueType::Float(f) => print!("{}", f),
                    ValueType::String(s) => print!("{}", s.replace("\\n", "\n")),
                    ValueType::Boolean(b) => print!("{}", b),
                    _ => return Err(format!("Invalid type for print {:?}", a_clone)),
                }
            },
            InstructionKind::Input => {
                io::stdout().flush().expect("Failed to flush stdout");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input = input.trim().to_string();
                stack.push(ValueType::String(input));
            },
            InstructionKind::Jump() => {
                println!("{:?}", instr.params);
            },
        }
    }

    Ok(stack)
}