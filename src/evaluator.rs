use std::collections::HashMap;
use crate::parser::{ValueType, InstructionKind, ParserRet, Buffer};
use syscalls;

fn ptr_to_vec(buf: Buffer) -> Vec<u8> {
    let data = unsafe { std::slice::from_raw_parts(buf.ptr as *const u8, buf.size) };
    data.to_vec()
}

fn trim_vec(buf: Vec<u8>) -> Vec<u8> {
    let mut trimmed = buf.clone();
    trimmed.retain(|&x| x != 0);
    trimmed
}

pub fn evaluate(parsed: ParserRet) -> Result<(Vec<ValueType>, i32), String> {
    let instrs = parsed.instrs;
    let labels = parsed.labels;
    let funcs = parsed.funcs;
    let mut vars: HashMap<String, ValueType> = HashMap::new();

    let mut stack: Vec<ValueType> = Vec::new();
    let mut ret_stack: Vec<i32> = Vec::new();

    let mut cur = *funcs.get("@entry").ok_or("Entry function not found")?;

    while cur < instrs.len() {
        let instr = instrs.get(cur).ok_or("Instruction not found")?;
        match instr.kind {
            InstructionKind::Push() => {
                for param in &instr.params {
                    if let ValueType::Variable(var_name) = param {
                        let var = vars.get(var_name).ok_or("Push: Variable not found")?;
                        stack.push(var.clone());
                    } else {
                        stack.push(param.clone());
                    }
                }
            },
            InstructionKind::Rot => {
                let a = stack.pop().ok_or("Rot: Stack underflow")?.clone();
                let b = stack.pop().ok_or("Rot: Stack underflow")?.clone();
                stack.push(a);
                stack.push(b);
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
                    (ValueType::Buffer(a), ValueType::Buffer(b)) => {
                        stack.push(ValueType::Boolean(a.buffer == b.buffer));
                    },
                    _ => return Err(format!("Invalid types for equal {:?} {:?}", a_clone, b_clone)),
                }
            }
            InstructionKind::Pop() => {
                let a = stack.pop().ok_or("Pop: Stack underflow")?;
                let var_name = instr.params[0].clone().to_string();
                vars.insert(var_name, a).map(|old| old);
            },
            InstructionKind::Dup => {
                let a = stack.last().ok_or("Dup: Stack underflow")?.clone();
                stack.push(a);
            },
            InstructionKind::Jump() => {
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or("Jump: Label not found")?;
                cur = *i;
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
                    _ => return Err("Jnz: Invalid type".to_string()),
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
                    _ => return Err("Jzr: Invalid type".to_string()),
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
                    ValueType::Buffer(b) => {
                        let buf = ptr_to_vec(b);
                        let buf = trim_vec(buf);
                        String::from_utf8(buf).unwrap()
                    },
                    _ => return Err("Type: Invalid type".to_string()),
                };

                let res = match label {
                    s if s == "int" => {
                        match a.parse::<i32>() {
                            Ok(i) => ValueType::Integer(i),
                            Err(_) => match a.parse::<bool>() {
                                Ok(b) => ValueType::Integer(b as i32),
                                Err(_) => return Err("Type: Invalid int or bool".to_string()),
                            },
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
                let i = ret_stack.pop().ok_or("Ret: Stack underflow")?;
                cur = i as usize;
            },
            InstructionKind::Run() => {
                let func = instr.params[0].clone();
                let i = funcs.get(&func.to_string()).ok_or("Run: Function not found")?;
                ret_stack.push(cur as i32);
                cur = *i;

            },
            InstructionKind::Sys => {
                let syscall_number = stack.pop().ok_or("Sys: Stack underflow")?.clone();
                let mut args = Vec::new();

                for _ in 0..6 {
                    if let Some(arg) = stack.pop() {
                        args.push(arg);
                    } else {
                        args.push(ValueType::Integer(0)); // Default to 0 if not enough arguments
                    }
                }

                let result = match syscall_number {
                    ValueType::Integer(num) => {
                        let syscall_args: Vec<usize> = args.iter().map(|arg| match arg {
                            ValueType::Integer(i) => *i as usize,
                            ValueType::Float(f) => *f as usize,
                            ValueType::Boolean(b) => *b as usize,
                            ValueType::String(s) => {
                                s.as_ptr() as usize
                            },
                            ValueType::Buffer(b) => {
                                b.ptr
                            },
                            ValueType::Variable(v) => {
                                v.len()
                            }
                        }).collect();

                        let syscall_args = syscalls::SyscallArgs::new(syscall_args[0], syscall_args[1], syscall_args[2], syscall_args[3], syscall_args[4], syscall_args[5]);
                        let result = unsafe { syscalls::syscall(syscalls::Sysno::from(num as u32), &syscall_args) };
                        result
                    },
                    _ => return Err("Sys: Invalid syscall number type".to_string()),
                };

                stack.push(ValueType::Integer(result.unwrap() as i32));
            },
            InstructionKind::Len => {
                let a = stack.last().ok_or("Len: Stack underflow")?.clone();
                let len = match a {
                    ValueType::String(s) => s.len(),
                    ValueType::Buffer(b) => b.size,
                    _ => return Err("Len: Invalid type".to_string()),
                };
                stack.push(ValueType::Integer(len as i32));
            },
            InstructionKind::Label() => {},
            InstructionKind::Function() => {},
        }

        cur += 1;
    }

    Ok((stack, 0))
}