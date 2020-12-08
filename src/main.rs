use std::io::BufRead;
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum CPUError {
    #[error("the cpu terminated successfully")]
    Terminated,
    #[error("loop detected")]
    LoopDetected(usize),
    #[error("index out of bounds: {0}")]
    IndexOutOfBounds(usize),
    #[error("failed to decode string as integer `{0}`")]
    IntegerDecode(String),
    #[error("unknown instruction: `{0}`")]
    UnknownInstruction(String),
}

#[derive(Debug)]
struct CPU {
    code: Vec<String>,
    counts: Vec<usize>,
    acc: isize,
    ip: usize,
}

impl CPU {
    fn new(code: &str) -> CPU {
        let cursor = std::io::Cursor::new(code);
        let code: Vec<String> = cursor.lines().filter_map(Result::ok).collect();
        let code_len = code.len();
        CPU {
            code,
            counts: vec![0; code_len],
            acc: 0,
            ip: 0,
        }
    }

    fn step(&mut self) -> Result<bool, CPUError> {
        if self.ip == self.code.len() {
            return Ok(false);
        }
        let (opcode, arg) = self
            .code
            .get(self.ip)
            .and_then(|s| {
                let mut parts = s.trim().split_whitespace();
                let op = parts.next()?;
                let arg = parts.next()?;
                Some((op, arg))
            })
            .ok_or(CPUError::IndexOutOfBounds(self.ip))?;

        let arg: isize = arg
            .parse()
            .map_err(|_| CPUError::IntegerDecode(arg.to_string()))?;

        let ins_count = self
            .counts
            .get_mut(self.ip)
            .ok_or(CPUError::IndexOutOfBounds(self.ip))?;

        // part 1 code!
        *ins_count += 1;
        if *ins_count == 2 {
            println!("acc: {}", self.acc);
            return Ok(false);
        }


        match opcode {
            "nop" => {
                self.ip += 1;
                Ok(true)
            }
            "acc" => {
                self.acc += arg;
                self.ip += 1;
                Ok(true)
            }
            "jmp" => {
                self.ip = (self.ip as isize).saturating_add(arg) as usize;
                Ok(true)
            }
            _ => Err(CPUError::UnknownInstruction(opcode.to_string())),
        }
    }
}

fn main() {
    let mut r = BufReader::new(stdin());
    let mut buf = String::new();
    r.read_to_string(&mut buf).unwrap();

    let mut cpu = CPU::new(&buf);

    loop {
        let res = cpu.step();
        if let Ok(false) = res {
            break
        }
        if let Err(e) = res {
            eprintln!("{:?}", e);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "nop +0
                        acc +1
                        jmp +4
                        acc +3
                        jmp -3
                        acc -99
                        acc +1
                        jmp -4
                        acc +6";

    #[test]
    fn run_program() {
        let mut cpu = CPU::new(DATA);
        cpu.step().unwrap();
        assert_eq!(cpu.ip, 1);
        assert_eq!(cpu.acc, 0);
        cpu.step().unwrap();
        assert_eq!(cpu.ip, 2);
        assert_eq!(cpu.acc, 1);
        cpu.step().unwrap();
        assert_eq!(cpu.ip, 6);
    }

    #[test]
    fn test_terminate() {
        let mut cpu = CPU::new("nop +0\njmp +2\nnop -99");
        cpu.step().unwrap();
        cpu.step().unwrap();
        cpu.step().unwrap();
        match cpu.step() {
            Err(CPUError::IndexOutOfBounds(x)) => {
                assert_eq!(x, 3);
            }
            _ => panic!("cpu failed to terminate"),
        }
    }
}
