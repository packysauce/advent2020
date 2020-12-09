use std::io::BufRead;
use std::io::{stdin, BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
enum CPUError {
    #[error("loop detected at {ip} to {dst}")]
    WouldLoop { ip: usize, dst: isize },
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
    lr: usize,
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
            lr: 0,
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.ip = 0;
        self.lr = 0;
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
        *ins_count += 1;

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
                self.lr = self.ip;
                let new_ip = (self.ip as isize).saturating_add(arg) as usize;
                if self.counts.get(new_ip).map_or(0, |i| *i) > 0 {
                    Err(CPUError::WouldLoop {
                        ip: self.ip,
                        dst: arg,
                    })
                } else if new_ip >= self.code.len() {
                    Err(CPUError::IndexOutOfBounds(new_ip))
                } else {
                    self.ip = new_ip;
                    Ok(true)
                }
            }
            _ => Err(CPUError::UnknownInstruction(opcode.to_string())),
        }
    }

    fn run(&mut self) -> Result<(), CPUError> {
        loop {
            let keep_running = self.step()?;
            if !keep_running {
                return Ok(())
            }
        }
    }
}

fn run_with_swapped(swap: usize, cpu: &mut CPU) -> Option<isize> {
    // set up swapped instruction
    let ins = cpu.code.get_mut(swap)?;
    match &ins[..3] {
        x @ "nop" => *ins = ins.replace(x, "jmp"),
        x @ "jmp" => *ins = ins.replace(x, "nop"),
        _ => {}
    };

    loop {
        let r = cpu.step();
        match r {
            Ok(true) => {}, // keep running
            Ok(false) => return Some(cpu.acc),
            Err(CPUError::IndexOutOfBounds(612)) => return Some(cpu.acc),
            Err(_) => return None, // error, no valid answer here
        }
    }
}

fn main() {
    let mut r = BufReader::new(stdin());
    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();

    let ix_count = data.lines().count();
    for i in 0..ix_count {
        let mut cpu = CPU::new(&data);
        if let Some(i) = run_with_swapped(i, &mut cpu) {
            println!("cpu stopped, acc: {}", i);
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
    fn runs_fine() {
        let data = "nop +0
                         acc +1
                         jmp +4
                         acc +3
                         jmp -3
                         acc -99
                         acc +1
                         nop -4
                         acc +6";
        let mut cpu = CPU::new(data);
        loop {
            let r = cpu.step();
            match r {
                Ok(true) => {}
                Ok(false) => return,
                Err(e) => panic!("cpu failed to stop: {}", e),
            }
        }
    }

    #[test]
    fn runs_with_swapped() {
        let mut cpu = CPU::new("nop +0\nacc +5\njmp +2");
        run_with_swapped(0, &mut cpu);
        run_with_swapped(1, &mut cpu);
        run_with_swapped(2, &mut cpu);
        assert_eq!(cpu.code, vec!["jmp +0", "acc +5", "nop +2"]);
    }

    #[test]
    fn test_loop() {
        let mut cpu = CPU::new("nop +0\njmp -1");
        assert_eq!(cpu.step(), Ok(true));
        assert_eq!(cpu.step(), Err(CPUError::WouldLoop { ip: 1, dst: 0 }));
    }

    #[test]
    fn test_jumped_out_of_bounds() {
        let mut cpu = CPU::new("nop +0\njmp +2\nnop -99");
        assert_eq!(cpu.step(), Ok(true));
        assert_eq!(cpu.step(), Err(CPUError::IndexOutOfBounds(3)));
    }
}
