use std::collections::HashMap;
use crate::parser::{ValueType, InstructionKind, ParserRet};
use syscalls;
use crate::Error;

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

pub fn evaluate(parsed: ParserRet) -> Result<(Vec<ValueType>, i32), Error> {
    let instrs = parsed.instrs;
    let labels = parsed.labels;
    let funcs = parsed.funcs;

    let mut vars: HashMap<String, ValueType> = HashMap::new();
    let mut bufs: HashMap<String, Buffer> = HashMap::new();

    let mut stack: Vec<ValueType> = Vec::new();
    let mut ret_stack: Vec<usize> = Vec::new();

    let mut cur = *funcs.get("@entry").ok_or(Error::new("Entry function not found", 0, 0))?;

    while cur < instrs.len() {
        let instr = instrs.get(cur).ok_or(Error::new("Instruction not found", 0, 0))?;
        match instr.kind {
            InstructionKind::Psh => {
                for param in &instr.params {
                    if let ValueType::Variable(var_name) = param {
                        let var = vars.get(var_name).ok_or(Error::new("Push: Variable not found", instr.line, instr.col))?;
                        stack.push(var.clone());
                } else {
                        stack.push(param.clone());
                    }
                }
            }
            InstructionKind::Rot => {
                let a = stack.pop().ok_or(Error::new("Rot: Stack underflow", instr.line, instr.col))?.clone();
                let b = stack.pop().ok_or(Error::new("Rot: Stack underflow", instr.line, instr.col))?.clone();
                stack.push(a);
                stack.push(b);
            },
            InstructionKind::Add => {
                let a = stack.pop().ok_or(Error::new("Add: Stack underflow", instr.line, instr.col))?.clone();
                let b = stack.pop().ok_or(Error::new("Add: Stack underflow", instr.line, instr.col))?.clone();
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

                    _ => return Err(Error::new(format!("Invalid types for add {:?} {:?}", a_clone, b_clone), instr.line, instr.col)),
                }
            },
            InstructionKind::Sub => {
                let a = stack.pop().ok_or(Error::new("Sub: Stack underflow", instr.line, instr.col))?.clone();
                let b = stack.pop().ok_or(Error::new("Sub: Stack underflow", instr.line, instr.col))?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Integer(b - a));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a - b));
                    },
                    (ValueType::String(a), ValueType::String(b)) => {
                        stack.push(ValueType::String(b.replace(&a, "")));
                    },

                    _ => return Err(Error::new(format!("Invalid types for sub {:?} {:?}", a_clone, b_clone), instr.line, instr.col)),
                }
            },
            InstructionKind::Mul => {
                let a = stack.pop().ok_or(Error::new("Mul: Stack underflow", instr.line, instr.col))?.clone();
                let b = stack.pop().ok_or(Error::new("Mul: Stack underflow", instr.line, instr.col))?.clone();
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
                    _ => return Err(Error::new(format!("Invalid types for mul {:?} {:?}", a_clone, b_clone), instr.line, instr.col)),
                }
            },
            InstructionKind::Div => {
                let a = stack.pop().ok_or(Error::new("Div: Stack underflow", instr.line, instr.col))?.clone();
                let b = stack.pop().ok_or(Error::new("Div: Stack underflow", instr.line, instr.col))?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Integer(b / a));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a / b));
                    },

                    _ => return Err(Error::new(format!("Invalid types for div {:?} {:?}", a_clone, b_clone), instr.line, instr.col)),
                }
            },
            InstructionKind::Mod => {
                let a = stack.pop().ok_or(Error::new("Mod: Stack underflow", instr.line, instr.col))?.clone();
                let b = stack.pop().ok_or(Error::new("Mod: Stack underflow", instr.line, instr.col))?.clone();
                let a_clone = a.clone();
                let b_clone = b.clone();

                match (a, b) {
                    (ValueType::Integer(a), ValueType::Integer(b)) => {
                        stack.push(ValueType::Integer(b % a));
                    },
                    (ValueType::Float(a), ValueType::Float(b)) => {
                        stack.push(ValueType::Float(a % b));
                    },

                    _ => return Err(Error::new(format!("Invalid types for mod {:?} {:?}", a_clone, b_clone), instr.line, instr.col)),
                }
            },
            InstructionKind::Cmp => {
                let a = stack.pop().ok_or(Error::new("Equal: Stack underflow", instr.line, instr.col))?.clone();
                let b = stack.pop().ok_or(Error::new("Equal: Stack underflow", instr.line, instr.col))?.clone();
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
                    _ => return Err(Error::new(format!("Invalid types for equal {:?} {:?}", a_clone, b_clone), instr.line, instr.col)),
                }
            }
            InstructionKind::Pop => {
                let a = stack.pop().ok_or(Error::new("Pop: Stack underflow", instr.line, instr.col))?;
                let var_name = instr.params[0].clone().to_string();
                if var_name != "$_" && var_name != "$" {
                    vars.insert(var_name, a).map(|old| old);
                }
            },
            InstructionKind::Dup => {
                let a = stack.last().ok_or(Error::new("Dup: Stack underflow", instr.line, instr.col))?.clone();
                stack.push(a);
            },
            InstructionKind::Jmp => {
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or(Error::new("Jump: Label not found", instr.line, instr.col))?;
                cur = *i;
            }
            InstructionKind::Jnz => {
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or(Error::new("Jnz: Label not found", instr.line, instr.col))? - 1;
                let a = stack.pop().ok_or(Error::new("Jnz: Stack underflow", instr.line, instr.col))?.clone();
                match a {
                    ValueType::Integer(n) => if n != 0 { cur = i; },
                    ValueType::Float(n) => if n != 0.0 { cur = i; },
                    ValueType::String(n) => if n != "" { cur = i; },
                    ValueType::Boolean(n) => if n { cur = i; },
                    _ => return Err(Error::new("Jnz: Invalid type".to_string(), instr.line, instr.col)),
                }
            },InstructionKind::Jzr => {
                let label = instr.params[0].clone();
                let i = labels.get(&label.to_string()).ok_or(Error::new("Jzr: Label not found", instr.line, instr.col))? - 1;
                let a = stack.pop().ok_or(Error::new("Jzr: Stack underflow", instr.line, instr.col))?.clone();
                match a {
                    ValueType::Integer(n) => if n == 0 { cur = i; },
                    ValueType::Float(n) => if n == 0.0 { cur = i; },
                    ValueType::String(n) => if n == "" { cur = i; },
                    ValueType::Boolean(n) => if !n { cur = i; },
                    _ => return Err(Error::new("Jzr: Invalid type".to_string(), instr.line, instr.col)),
                }
            },
            InstructionKind::Type => {
                let label = match instr.params[0].clone() {
                    ValueType::String(s) => s,
                    _ => return Err(Error::new("Type: Invalid type".to_string(), instr.line, instr.col)),
                };
                let a = match stack.pop().ok_or(Error::new("Type: Stack underflow", instr.line, instr.col))?.clone() {
                    ValueType::String(s) => s,
                    ValueType::Integer(i) => i.to_string(),
                    ValueType::Float(f) => f.to_string(),
                    ValueType::Boolean(b) => b.to_string(),
                    ValueType::Buffer(b) => {
                        let buf = bufs.get(&b).ok_or(Error::new("Type: Buffer not found", instr.line, instr.col))?.clone();
                        let vec = ptr_to_vec(buf);
                        let trimmed_vec = trim_vec(vec);
                        String::from_utf8(trimmed_vec).unwrap()
                    },
                    _ => return Err(Error::new("Type: Invalid type".to_string(), instr.line, instr.col)),
                };

                let res = match label {
                    s if s == "int" => {
                        match a.parse::<i32>() {
                            Ok(i) => ValueType::Integer(i),
                            Err(_) => match a.parse::<bool>() {
                                Ok(b) => ValueType::Integer(b as i32),
                                Err(_) => return Err(Error::new("Type: Invalid int or bool".to_string(), instr.line, instr.col)),
                            },
                        }
                    },
                    s if s == "float" => {
                        match a.parse::<f32>() {
                            Ok(f) => ValueType::Float(f),
                            Err(_) => return Err(Error::new("Type: Invalid float".to_string(), instr.line, instr.col)),
                        }
                    },
                    s if s == "str" => ValueType::String(a),
                    s if s == "bool" => {
                        match a.parse::<bool>() {
                            Ok(b) => ValueType::Boolean(b),
                            Err(_) => return Err(Error::new("Type: Invalid bool".to_string(), instr.line, instr.col)),
                        }
                    },
                    _ => return Err(Error::new("Type: Invalid type".to_string(), instr.line, instr.col)),
                };

                stack.push(res);
            },
            InstructionKind::Ret => {
                if let Some(i) = ret_stack.pop() {
                    cur = i;
                } else {
                    let a = stack.pop().ok_or(Error::new("Ret: Stack underflow", instr.line, instr.col))?;
                    return Ok((stack, a.to_int().map_err(|e| Error::new(e, instr.line, instr.col))?));
                }
            },
            InstructionKind::Run => {
                let func = instr.params[0].clone();
                let i = funcs.get(&func.to_string()).ok_or(Error::new("Run: Function not found", instr.line, instr.col))?;
                ret_stack.push(cur);
                cur = *i;

            },
            InstructionKind::Sys => {
                let syscall_number = stack.pop().ok_or(Error::new("Sys: Stack underflow", instr.line, instr.col))?.clone();
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
                                let buf = bufs.get(b).ok_or(Error::new("Sys: Buffer not found", instr.line, instr.col)).unwrap().clone();
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
                    _ => return Err(Error::new("Sys: Invalid syscall number type".to_string(), instr.line, instr.col)),
                };

                stack.push(ValueType::Integer(result.unwrap() as i32));
            },
            InstructionKind::Len => {
                let a = stack.last().ok_or(Error::new("Len: Stack underflow", instr.line, instr.col))?.clone();
                let len = match a {
                    ValueType::String(s) => s.len(),
                    ValueType::Buffer(b) => {
                        let buf = bufs.get(&b).ok_or(Error::new("Len: Buffer not found", instr.line, instr.col))?.clone();
                        buf.size
                    },
                    _ => return Err(Error::new("Len: Invalid type".to_string(), instr.line, instr.col)),
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
                    _ => return Err(Error::new("Dlc: Invalid type".to_string(), instr.line, instr.col)),
                };
            }
            InstructionKind::Lbl => {}
            InstructionKind::Fun => {}
            InstructionKind::Alc => {
                let name = instr.params[0].clone().to_string();
                let size = instr.params[1].to_int().map_err(|e| Error::new(e, instr.line, instr.col))? as usize;
                let mut data = vec![0u8; size];
                let ptr = data.as_mut_ptr() as usize;

                let buffer = Buffer {
                    data,
                    size,
                    ptr,
                };

                bufs.insert(name, buffer);
            },
        }

        cur += 1;
    }

    Ok((stack, 0))
}