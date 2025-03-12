use std::collections::HashMap;
use crate::parser::{ValueType, InstructionKind, ParserRet};
use syscalls;

#[derive(Debug, PartialEq, Clone)]
pub struct Buffer {
    pub data: Vec<u8>,
    pub size: usize,
    pub ptr: usize,
}

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
    let mut bufs: HashMap<String, Buffer> = HashMap::new();

    let mut stack: Vec<ValueType> = Vec::new();
    let mut ret_stack: Vec<usize> = Vec::new();

    let mut cur = *funcs.get("@entry").ok_or("Entry function not found")?;

    while cur < instrs.len() {
        let instr = instrs.get(cur).ok_or("Instruction not found")?;
        match instr.kind {
            InstructionKind::Psh => {
                for param in &instr.params {
                    if let ValueType::Variable(var_name) = param {
                        let var = vars.get(var_name).ok_or("Push: Variable not found")?;
                        stack.push(var.clone());
                } else {
                        stack.push(param.clone());
                    }
                }
            }
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
                    _ => return Err(format!("Invalid types for equal {:?} {:?}", a_clone, b_clone)),
                }
            }
            InstructionKind::Pop => {
                let a = stack.pop().ok_or("Pop: Stack underflow")?;
                let var_name = instr.params[0].clone().to_string();
                vars.insert(var_name, a).map(|old| old);
            },
            InstructionKind::Dup => {
                let a = stack.last().ok_or("Dup: Stack underflow")?.clone();
                stack.push(a);
            },
            InstructionKind::Jmp => {
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or("Jump: Label not found")?;
                cur = *i;
            }
            InstructionKind::Jnz => {
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
            },InstructionKind::Jzr => {
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
            InstructionKind::Type => {
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
                        let buf = bufs.get(&b).ok_or("Type: Buffer not found")?.clone();
                        let vec = ptr_to_vec(buf);
                        let trimmed_vec = trim_vec(vec);
                        String::from_utf8(trimmed_vec).unwrap()
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
                if let Some(i) = ret_stack.pop() {
                    cur = i;
                } else {
                    let a = stack.pop().ok_or("Ret: Stack underflow")?;
                    return Ok((stack, a.to_int()?));
                }
            },
            InstructionKind::Run => {
                let func = instr.params[0].clone();
                let i = funcs.get(&func.to_string()).ok_or("Run: Function not found")?;
                ret_stack.push(cur);
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
                                let buf = bufs.get(b).ok_or("Sys: Buffer not found").unwrap().clone();
                                buf.ptr
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
                    ValueType::Buffer(b) => {
                        println!("{}, {:?}", b, bufs);
                        let buf = bufs.get(&b).ok_or("Len: Buffer not found")?.clone();
                        buf.size
                    },
                    _ => return Err("Len: Invalid type".to_string()),
                };
                stack.push(ValueType::Integer(len as i32));
            },
            InstructionKind::Dlc => {
                let a = instr.params[0].clone();
                match a {
                    ValueType::Buffer(b) => {
                        bufs.remove(&b);
                    },
                    ValueType::Variable(v) => {
                        vars.remove(&v);
                    },
                    _ => return Err("Dlc: Invalid type".to_string()),
                };
            }
            InstructionKind::Lbl => {}
            InstructionKind::Fun => {}
            InstructionKind::Alc => {
                let name = instr.params[0].clone().to_string().replace('"', "");
                let size = instr.params[1].to_int()? as usize;
                let data = vec![0u8; size];
                let ptr = data.clone().as_mut_ptr() as usize;

                let buffer = Buffer {
                    data,
                    size,
                    ptr,
                };

                println!("{}", name);
                bufs.insert(name, buffer);
            },
        }

        cur += 1;
    }

    Ok((stack, 0))
}