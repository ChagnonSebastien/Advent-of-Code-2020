use crate::utils::Part;


#[derive(Copy, Clone)]
enum Value {
  Integer(isize), Register(usize)
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Value::Integer(i1) => match other {
        Value::Integer(i2) => *i1 == *i2,
        _ => false,
      },
      Value::Register(r1) => match other {
        Value::Register(r2) => *r1 == *r2,
        _ => false,
      },
    }
  }
}
impl Eq for Value {}

#[derive(Copy, Clone)]
enum Instruction {
  CopyTo(Value, Value),
  Increment(Value),
  Decrement(Value),
  JumpIfNotZero(Value, Value),
  Toggle(Value),
  Out(Value),
}

impl Instruction {
  fn register_index(c: &str) -> usize {
    match c {
      "a" => 0,
      "b" => 1,
      "c" => 2,
      "d" => 3,
      _ => panic!("Invalid register value: {}", c),
    }
  }

  fn from_string(s: &str) -> Self {
    let mut parts = s.split(" ");
    let verb = parts.next().expect(format!("Invalid instruction (No verb): {}", s).as_str());
    let first_arg = parts.next().expect(format!("Invalid instruction (No args): {}", s).as_str());
    let first_value = match first_arg.parse::<isize>() {
      Ok(v) => Value::Integer(v),
      Err(_) => Value::Register(Self::register_index(first_arg)),
    };

    match verb {
      "cpy" => {
        let second_arg = parts.next().expect(format!("Invalid instruction (No second arg): {}", s).as_str());
        Self::CopyTo(
          first_value,
          match second_arg.parse::<isize>() {
            Ok(v) => Value::Integer(v),
            Err(_) => Value::Register(Self::register_index(second_arg)),
          },
        )
      }
      "jnz" => {
        let second_arg = parts.next().expect(format!("Invalid instruction (No second arg): {}", s).as_str());
        Self::JumpIfNotZero(
          first_value,
          match second_arg.parse::<isize>() {
            Ok(v) => Value::Integer(v),
            Err(_) => Value::Register(Self::register_index(second_arg)),
          },
        )
      }
      "inc" => Self::Increment(first_value),
      "dec" => Self::Decrement(first_value),
      "tgl" => Self::Toggle(first_value),
      "out" => Self::Out(first_value),
      _ => panic!("Unknown verb: {}", s),
    }
  }
}

struct Computer {
  instructions: Vec<Instruction>,
  registers: [isize; 4],
  offset: isize,
  last_print: Option<isize>,
  valid: bool,
}

impl Computer {
  fn compute(&mut self) {
    let current_instruction = self.instructions.get(self.offset as usize).expect("Offset points to an invalid instruction");
    match current_instruction {
      Instruction::CopyTo(value, to) => {
        match to {
          Value::Integer(_) => {},
          Value::Register(register) => {
            match value {
              Value::Integer(i) => self.registers[*register] = *i,
              Value::Register(r) => self.registers[*register] = self.registers[*r],
            };
          }
        }
        self.offset += 1;
      },
      Instruction::JumpIfNotZero(value, amount) => {
        let condition = match value {
          Value::Integer(i) => *i,
          Value::Register(r) => self.registers[*r],
        };
        match condition {
          0 => self.offset += 1,
          _ => self.offset += match amount {
            Value::Integer(i) => match *i == -2 {
              true => {
                match self.instructions.get((self.offset - 2) as usize).unwrap() {
                  Instruction::Increment(v1) => {
                    match *v1 == *value && condition < 0 {
                      true => {
                        match v1 {
                          Value::Integer(_i1) => {},
                          Value::Register(r1) => self.registers[*r1] = 0,
                        };
                        match self.instructions.get((self.offset - 1) as usize).unwrap() {
                          Instruction::Decrement(v2) => {
                            match v2 {
                              Value::Integer(_i2) => {},
                              Value::Register(r2) => self.registers[*r2] += condition,
                            };
                            1
                          },
                          Instruction::Increment(v2) => {
                            match v2 {
                              Value::Integer(_i2) => {},
                              Value::Register(r2) => self.registers[*r2] -= condition,
                            };
                            1
                          },
                          _ => *i,
                        }
                      },
                      false => {
                        match self.instructions.get((self.offset - 1) as usize).unwrap() {
                          Instruction::Decrement(v2) => {
                            match *v2 == *value && condition > 0 {
                              true => {
                                match v1 {
                                  Value::Integer(_i1) => {},
                                  Value::Register(r1) => self.registers[*r1] += condition,
                                };
                                match v2 {
                                  Value::Integer(_i2) => {},
                                  Value::Register(r2) => self.registers[*r2] = 0,
                                };
                                1
                              },
                              false => *i,
                            }
                          },
                          Instruction::Increment(v2) => {
                            match *v2 == *value && condition < 0 {
                              true => {
                                match v1 {
                                  Value::Integer(_i1) => {},
                                  Value::Register(r1) => self.registers[*r1] -= condition,
                                };
                                match v2 {
                                  Value::Integer(_i2) => {},
                                  Value::Register(r2) => self.registers[*r2] = 0,
                                };
                                1
                              },
                              false => *i,
                            }
                          },
                          _ => *i,
                        }
                      },
                    }
                  },
                  Instruction::Decrement(v1) => {
                    match *v1 == *value && condition > 0 {
                      true => {
                        match v1 {
                          Value::Integer(_i1) => {},
                          Value::Register(r1) => self.registers[*r1] = 0,
                        };
                        match self.instructions.get((self.offset - 1) as usize).unwrap() {
                          Instruction::Increment(v2) => {
                            match v2 {
                              Value::Integer(_i2) => {},
                              Value::Register(r2) => self.registers[*r2] += condition,
                            };
                            1
                          },
                          Instruction::Decrement(v2) => {
                            match v2 {
                              Value::Integer(_i2) => {},
                              Value::Register(r2) => self.registers[*r2] -= condition,
                            };
                            1
                          },
                          _ => *i,
                        }
                      },
                      false => {
                        match self.instructions.get((self.offset - 1) as usize).unwrap() {
                          Instruction::Decrement(v2) => {
                            match *v2 == *value && condition > 0 {
                              true => {
                                match v1 {
                                  Value::Integer(_i1) => {},
                                  Value::Register(r1) => self.registers[*r1] -= condition,
                                };
                                match v2 {
                                  Value::Integer(_i2) => {},
                                  Value::Register(r2) => self.registers[*r2] = 0,
                                };
                                1
                              },
                              false => *i,
                            }
                          },
                          Instruction::Increment(v2) => {
                            match *v2 == *value && condition < 0 {
                              true => {
                                match v1 {
                                  Value::Integer(_i1) => {},
                                  Value::Register(r1) => self.registers[*r1] += condition,
                                };
                                match v2 {
                                  Value::Integer(_i2) => {},
                                  Value::Register(r2) => self.registers[*r2] = 0,
                                };
                                1
                              },
                              false => *i,
                            }
                          },
                          _ => *i,
                        }
                      },
                    }
                  },
                  _ => *i,
                }
              },
              false => *i,
            },
            Value::Register(r) => self.registers[*r]
          },
        };
      },
      Instruction::Increment(value) => {
        match value {
          Value::Integer(_) => {},
          Value::Register(register) => self.registers[*register] += 1,
        }
        self.offset += 1;
      },
      Instruction::Decrement(value) => {
        match value {
          Value::Integer(_) => {},
          Value::Register(register) => self.registers[*register] -= 1,
        }
        self.offset += 1;
      },
      Instruction::Toggle(value) => {
        let to_change = self.offset + match value {
          Value::Integer(i) => *i,
          Value::Register(r) => self.registers[*r]
        };
        if to_change >= 0 && to_change < self.instructions.len() as isize {
          let previous = self.instructions.remove(to_change as usize);
          self.instructions.insert(to_change as usize, match previous {
            Instruction::Increment(a) => Instruction::Decrement(a),
            Instruction::Decrement(a) => Instruction::Increment(a),
            Instruction::Toggle(a) => Instruction::Increment(a),
            Instruction::Out(a) => Instruction::Increment(a),
            Instruction::JumpIfNotZero(a, b) => Instruction::CopyTo(a, b),
            Instruction::CopyTo(a, b) => Instruction::JumpIfNotZero(a, b),
          });
        }
        self.offset += 1;
      },
      Instruction::Out(value) => {
        let new_print = match value {
          Value::Integer(i) => *i,
          Value::Register(register) => self.registers[*register],
        };
        match self.last_print {
          Some(i) => {
            if i == new_print || (i != 0 && i != 1)  {
              self.valid = false;
            }
            self.last_print = Some(new_print);
          },
          None => self.last_print = Some(new_print),
        };
        self.offset += 1;
      }
    };
  }
  
  #[allow(dead_code)]
  fn print_instructions(&self) {
    for i in 0..self.instructions.len() {
      let instruction = self.instructions.get(i).unwrap();
      match instruction {
        Instruction::CopyTo(a, b) => println!(
          "{} cpy {} {}",
          match self.offset as usize == i {true=>"->", false=>"  "},
          match a {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          },
          match b {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          }
        ),
        Instruction::JumpIfNotZero(a, b) => println!(
          "{} jnz {} {}",
          match self.offset as usize == i {true=>"->", false=>"  "},
          match a {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          },
          match b {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          }
        ),
        Instruction::Increment(a) => println!(
          "{} inc {}",
          match self.offset as usize == i {true=>"->", false=>"  "},
          match a {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          }
        ),
        Instruction::Decrement(a) => println!(
          "{} dec {}",
          match self.offset as usize == i {true=>"->", false=>"  "},
          match a {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          }
        ),
        Instruction::Toggle(a) => println!(
          "{} tgl {}",
          match self.offset as usize == i {true=>"->", false=>"  "},
          match a {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          }
        ),
        Instruction::Out(a) => println!(
          "{} out {}",
          match self.offset as usize == i {true=>"->", false=>"  "},
          match a {
            Value::Integer(i) => format!("{}", *i),
            Value::Register(r) => format!("{}:{}", match r { 0=>'a',1=>'b',2=>'c',3=>'d',_=>'E' }, self.registers[*r])
          }
        ),
      }
    }
  }

  fn is_finished(&self) -> bool {
    self.instructions.get(self.offset as usize).is_none()
  }
}

pub fn execute(input: String, part: &Part) {
  let mut instructions: Vec<Instruction> = Vec::new();
  for line in input.split("\n") {
    instructions.push(Instruction::from_string(line))
  }

  let mut foundValid = false;
  let mut i = 1;

  while !foundValid {
    let mut computer = Computer {
      instructions: instructions.iter().map(|i| *i).collect(),
      registers: [i, 0, 0, 0],
      offset: 0,
      last_print: None,
      valid: true,
    };

    let mut k = 0;
    while !computer.is_finished() && computer.valid && k < 100000000 {
      computer.compute();
      k += 1;
    }

    if computer.valid && !computer.is_finished() {
      foundValid = true;
    } else {
      i += 1;
    }

  }

  println!("First Found: {}", i);
}
