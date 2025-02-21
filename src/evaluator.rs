use std::collections::HashMap;
use std::io::Write;
use std::io;
use crate::parser::{ValueType, Instruction, InstructionKind};

pub fn evaluate(instrs: Vec<Instruction>, labels: HashMap<String, usize>) -> Result<(Vec<ValueType>, i32), String> {
    let mut stack: Vec<ValueType> = Vec::new();
    let mut cur = *labels.get(".entry").ok_or("Entry label not found")?;
    println!("{:?}{}", labels, cur);

    while cur < instrs.len() {
        let instr = instrs.get(cur).ok_or("Instruction not found")?;
        match instr.kind {
            InstructionKind::Push() => {
                for param in &instr.params {
                    stack.insert(0, param.clone());
                }
            },
            InstructionKind::Rot => {
                let a = stack.pop().ok_or("Rot: Stack underflow")?.clone();
                stack.insert(0, a);
            },
            InstructionKind::Add => {
                let a = stack.pop().ok_or("Add: Stack underflow")?.clone();
                let b = stack.pop().ok_or("Add: Stack underflow")?.clone();
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
                let a = stack.pop().ok_or("Sub: Stack underflow")?.clone();
                let b = stack.pop().ok_or("Sub: Stack underflow")?.clone();
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
                let a = stack.pop().ok_or("Mul: Stack underflow")?.clone();
                let b = stack.pop().ok_or("Mul: Stack underflow")?.clone();
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
                let a = stack.pop().ok_or("Div: Stack underflow")?.clone();
                let b = stack.pop().ok_or("Div: Stack underflow")?.clone();
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
                let a = stack.pop().ok_or("Mod: Stack underflow")?.clone();
                let b = stack.pop().ok_or("Mod: Stack underflow")?.clone();
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
            InstructionKind::Cmp => {
                let a = stack.pop().ok_or("Equal: Stack underflow")?.clone();
                let b = stack.pop().ok_or("Equal: Stack underflow")?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Boolean(a == b));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Boolean(a == b));
                    },
                    (ValueType::String(a), ValueType::String(b)) => {
                        stack.push(ValueType::Boolean(a == b));
                    },
                    (ValueType::Boolean(a), ValueType::Boolean(b)) => {
                        stack.push(ValueType::Boolean(a == b));
                    },
                    _ => return Err(format!("Invalid types for equal {:?} {:?}", a_clone, b_clone)),
                }
            }
            InstructionKind::Pop => {
                stack.pop();
            },
            InstructionKind::Dup => {
                let a = stack.last().ok_or("Dup: Stack underflow")?.clone();
                stack.push(a);
            },
            InstructionKind::Print => {
                let a = stack.pop().ok_or("Print: Stack underflow")?.clone();
                let ln = match instr.params.get(0).cloned().unwrap_or(ValueType::Boolean(false)) {
                    ValueType::Boolean(true) => "\n",
                    ValueType::Boolean(false) => "",
                    _ => return Err("Invalid type for print".to_string()),
                };

                //let a_clone = a.clone();
                match a {
                    ValueType::Integer(i) => print!("{}{}", i, ln),
                    ValueType::Float(f) => print!("{}{}", f, ln),
                    ValueType::String(s) => print!("{}{}", s.replace("\\n", "\n"), ln),
                    ValueType::Boolean(b) => print!("{}{}", b, ln),
                    //_ => return Err(format!("Invalid type for print {:?}", a_clone)),
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
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or("Jump: Label not found")? - 1;
                cur = i;
            },
            InstructionKind::Jnz() => {
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or("Jnz: Label not found")? - 1;
                let a = stack.pop().ok_or("Jnz: Stack underflow")?.clone();
                match a {
                    ValueType::Integer(n) => if n != 0 { cur = i; },
                    ValueType::Float(n) => if n != 0.0 { cur = i; },
                    ValueType::String(n) => if n != "" { cur = i; },
                    ValueType::Boolean(n) => if n { cur = i; },
                }
            },InstructionKind::Jzr() => {
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or("Jzr: Label not found")? - 1;
                let a = stack.pop().ok_or("Jzr: Stack underflow")?.clone();
                match a {
                    ValueType::Integer(n) => if n == 0 { cur = i; },
                    ValueType::Float(n) => if n == 0.0 { cur = i; },
                    ValueType::String(n) => if n == "" { cur = i; },
                    ValueType::Boolean(n) => if !n { cur = i; },
                }
            },
            InstructionKind::Type() => {
                let label = match instr.params[0].clone() {
                    ValueType::String(s) => s,
                    _ => return Err("Type: Invalid type".to_string()),
                };
                let a = match stack.pop().ok_or("Type: Stack underflow")?.clone() {
                    ValueType::String(s) => s,
                    ValueType::Integer(i) => i.to_string(),
                    ValueType::Float(f) => f.to_string(),
                    ValueType::Boolean(b) => b.to_string(),
                    _ => return Err("Type: Invalid type".to_string()),
                };

                let res = match label {
                    s if s == "int" => {
                        match a.parse::<i32>() {
                            Ok(i) => ValueType::Integer(i),
                            Err(_) => return Err("Type: Invalid int".to_string()),
                        }
                    },
                    s if s == "float" => {
                        match a.parse::<f32>() {
                            Ok(f) => ValueType::Float(f),
                            Err(_) => return Err("Type: Invalid float".to_string()),
                        }
                    },
                    s if s == "str" => ValueType::String(a),
                    s if s == "bool" => {
                        match a.parse::<bool>() {
                            Ok(b) => ValueType::Boolean(b),
                            Err(_) => return Err("Type: Invalid bool".to_string()),
                        }
                    },
                    _ => return Err("Type: Invalid type".to_string()),
                };

                stack.push(res);
            },
            InstructionKind::Ret => {
                let a = stack.pop().ok_or("Ret: Stack underflow")?.clone();

                return Ok((stack, a.to_int().expect("Ret: Invalid return value")));
            },
        }

        cur += 1;
    }

    Ok((stack, 0))
}