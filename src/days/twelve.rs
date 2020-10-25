use crate::utils::Part;

enum Value {
  Integer(isize), Register(usize)
}

enum Instruction {
  CopyTo(Value, usize),
  Increment(usize),
  Decrement(usize),
  JumpIfNotZero(Value, isize),
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
    let second_arg = parts.next();
    match verb {
      "cpy" => {
        Self::CopyTo(
          match first_arg.parse::<isize>() {
            Ok(v) => Value::Integer(v),
            Err(_) => Value::Register(Self::register_index(first_arg)),
          },
          Self::register_index(second_arg.expect(format!("Invalid instruction (No second arg): {}", s).as_str())),
        )
      }
      "jnz" => {
        Self::JumpIfNotZero(
          match first_arg.parse::<isize>() {
            Ok(v) => Value::Integer(v),
            Err(_) => Value::Register(Self::register_index(first_arg)),
          },
          second_arg
            .expect(format!("Invalid instruction (No second arg): {}", s).as_str())
            .parse::<isize>()
            .expect(format!("Invalid jump amount. Expected integer: {}", s).as_str()),
        )
      }
      "inc" => Self::Increment(Self::register_index(first_arg)),
      "dec" => Self::Decrement(Self::register_index(first_arg)),
      _ => panic!("Unknown verb: {}", s),
    }
  }
}

struct Computer {
  instructions: Vec<Instruction>,
  registers: [isize; 4],
  offset: isize,
}

impl Computer {
  fn compute(&mut self) {
    let current_instruction = self.instructions.get(self.offset as usize).expect("Offset points to an invalid instruction");
    match current_instruction {
      Instruction::CopyTo(value, register) => {
        match value {
          Value::Integer(i) => self.registers[*register] = *i,
          Value::Register(r) => self.registers[*register] = self.registers[*r],
        };
        self.offset += 1;
      },
      Instruction::JumpIfNotZero(value, amount) => {
        let condition = match value {
          Value::Integer(i) => &i,
          Value::Register(r) => &self.registers[*r],
        };
        match condition {
          0 => self.offset += 1,
          _ => self.offset += amount,
        };
      },
      Instruction::Increment(register) => {
        self.registers[*register] += 1;
        self.offset += 1;
      },
      Instruction::Decrement(register) => {
        self.registers[*register] -= 1;
        self.offset += 1;
      },
    };
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

  let mut computer = Computer {
    instructions: instructions,
    registers: [0, 0, match part { Part::PartOne => 0, Part::PartTwo => 1 }, 0],
    offset: 0,
  };

  while !computer.is_finished() {
    computer.compute();
  }

  println!("Register A : {}", computer.registers[Instruction::register_index("a")]);
}
