use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, map_res, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Instruction {
    opcode: OpCode,
    arg: i32,
}
impl Instruction {
    fn nop(arg: i32) -> Instruction {
        Instruction {
            opcode: OpCode::Nop,
            arg,
        }
    }
    fn jmp(arg: i32) -> Instruction {
        Instruction {
            opcode: OpCode::Jump,
            arg,
        }
    }
    fn acc(arg: i32) -> Instruction {
        Instruction {
            opcode: OpCode::Acc,
            arg,
        }
    }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum OpCode {
    Nop,
    Jump,
    Acc,
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    let (_, vs) = all_consuming(delimited(multispace0, instructions_parser, multispace0))(input)
        .map_err(|_| input.to_owned())?;
    Ok(vs)
}
fn instructions_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(multispace1, instruction_parser)(input)
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    nom::combinator::map(
        separated_pair(opcode_parser, multispace1, arg_parser),
        |(opcode, arg)| Instruction { opcode, arg },
    )(input)
}
fn opcode_parser(input: &str) -> IResult<&str, OpCode> {
    alt((
        value(OpCode::Nop, tag("nop")),
        value(OpCode::Jump, tag("jmp")),
        value(OpCode::Acc, tag("acc")),
    ))(input)
}
fn arg_parser(input: &str) -> IResult<&str, i32> {
    map_res(
        take_while(|c: char| c == '+' || c == '-' || c.is_digit(10)),
        |s: &str| s.parse(),
    )(input)
}

#[derive(Debug, Clone)]
struct Interpreter {
    program: Vec<Instruction>,
    acc: i32,
    pos: i32,
    visited: Vec<i32>,
}
impl Interpreter {
    fn new(program: Vec<Instruction>) -> Interpreter {
        Interpreter {
            program,
            acc: 0,
            pos: 0,
            visited: Vec::new(),
        }
    }
    fn step(&mut self) -> Option<Status> {
        if self.visited.contains(&self.pos) {
            return Some(Status::Loop);
        }
        self.visited.push(self.pos);

        let pos: usize = self.pos as usize;
        if pos == self.program.len() {
            return Some(Status::Complete);
        }
        if pos > self.program.len() {
            return Some(Status::Error);
        }
        let instruction = &self.program[pos];
        match instruction.opcode {
            OpCode::Jump => {
                self.pos += instruction.arg;
            }
            OpCode::Nop => {
                self.pos += 1;
            }
            OpCode::Acc => {
                self.acc += instruction.arg;
                self.pos += 1;
            }
        }
        None
    }
    fn drive(&mut self) -> Status {
        loop {
            if let Some(status) = self.step() {
                return status;
            }
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
enum Status {
    Complete,
    Error,
    Loop,
}

fn final_acc(mut interpreter: Interpreter) -> Option<i32> {
    match interpreter.drive() {
        Status::Complete | Status::Error => None,
        Status::Loop => Some(interpreter.acc),
    }
}

fn final_fixed_acc(interpreter: Interpreter) -> Option<i32> {
    for i in 0..interpreter.program.len() {
        let mut fixed = interpreter.clone();
        match interpreter.program[i].opcode {
            OpCode::Acc => continue,
            OpCode::Nop => fixed.program[i].opcode = OpCode::Jump,
            OpCode::Jump => fixed.program[i].opcode = OpCode::Nop,
        };
        if let Status::Complete = fixed.drive() {
            return Some(fixed.acc);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::{
        final_acc, final_fixed_acc, instruction_parser, parse_instructions, Instruction,
        Interpreter,
    };
    #[test]
    fn parser_one() {
        assert_eq!(instruction_parser("nop +0").unwrap().1, Instruction::nop(0));
        assert_eq!(instruction_parser("acc +1").unwrap().1, Instruction::acc(1));
        assert_eq!(
            instruction_parser("jmp -4").unwrap().1,
            Instruction::jmp(-4)
        );
    }

    const SMALL: &str = r"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        ";
    #[test]
    fn parser_list() {
        let parsed = parse_instructions(SMALL).unwrap();
        assert_eq!(parsed[0], Instruction::nop(0));
        assert_eq!(parsed[1], Instruction::acc(1));
        assert_eq!(parsed[2], Instruction::jmp(4));
    }

    #[test]
    fn small1() {
        let interpreter = Interpreter::new(parse_instructions(SMALL).unwrap());
        assert_eq!(final_acc(interpreter).unwrap(), 5);
    }

    #[test]
    fn normal1() {
        let raw = std::fs::read_to_string("data/day08.input").unwrap();
        let interpreter = Interpreter::new(parse_instructions(&raw).unwrap());
        assert_eq!(final_acc(interpreter).unwrap(), 1087);
    }

    #[test]
    fn small2() {
        let interpreter = Interpreter::new(parse_instructions(SMALL).unwrap());
        assert_eq!(final_fixed_acc(interpreter).unwrap(), 8);
    }
    #[test]
    fn normal2() {
        let raw = std::fs::read_to_string("data/day08.input").unwrap();
        let interpreter = Interpreter::new(parse_instructions(&raw).unwrap());
        assert_eq!(final_fixed_acc(interpreter).unwrap(), 780);
    }
}
