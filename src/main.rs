use std::collections::HashMap;

use lazy_static::lazy_static;
use num::Integer;

#[derive(Debug)]
struct DumbMask {
    and_mask: u64,
    or_mask: u64,
}

impl Default for DumbMask {
    fn default() -> Self {
        DumbMask {
            and_mask: u64::MAX,
            or_mask: 0,
        }
    }
}

impl<'a> std::ops::BitOr<&'a DumbMask> for u64 {
    type Output = u64;

    fn bitor(self, rhs: &'a DumbMask) -> Self::Output {
        self & rhs.and_mask | rhs.or_mask
    }
}

impl std::str::FromStr for DumbMask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = DumbMask::default();
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                'X' => {
                    mask.and_mask |= 1 << i;
                    mask.or_mask |= 0 << i;
                } // reset the mask bits
                '0' => {
                    mask.and_mask &= u64::MAX ^ (1 << i);
                }
                '1' => {
                    mask.or_mask |= 1 << i;
                }
                _ => panic!("blew up on s[i] = `{}` in s = `{}`", c, &s),
            }
        }
        Ok(mask)
    }
}

#[derive(Debug)]
enum Command {
    SetMask(DumbMask),
    SetMem(usize, u64),
}

impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
        }
        //let RE: regex::Regex = regex::Regex::new(r"mem\[(\d+)] = \d+").unwrap();
        match &s[..4] {
            "mask" => Ok(Command::SetMask(s[7..].parse()?)),
            "mem[" => {
                let caps = RE.captures(&s).ok_or(())?;
                let loc = caps.get(1).ok_or(())?.as_str().parse().map_err(|_| ())?;
                let value = caps.get(2).ok_or(())?.as_str().parse().map_err(|_| ())?;
                Ok(Command::SetMem(loc, value))
                // met[(\d+)] = (\d+)
            } //parse mem set
            _ => Err(()),
        }
    }
}

fn main() {
    /*
    let data = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                     mem[8] = 11\n\
                     mem[7] = 101\n\
                     mem[8] = 0";
                     */
    let data = include_str!("../inputs/day14.txt");
    let mut memory: HashMap<usize, u64> = HashMap::new();

    let commands = data
        .lines()
        .map(|s| s.parse::<Command>())
        .filter_map(Result::ok);

    let mut mask = DumbMask::default();
    for command in commands {
        match command {
            Command::SetMask(new_mask) => mask = new_mask,
            Command::SetMem(loc, value) => {
                let new_value = value | &mask;
                memory
                    .entry(loc)
                    .and_modify(|f| *f = new_value)
                    .or_insert(new_value);
            }
        }
    }

    let output: u64 = memory
        .values()
        .sum();
    println!("sum is {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mask_works() {
        let mask: DumbMask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
        assert_eq!(
            mask.and_mask,
            0b1111111111111111111111111111111111111111111111111111111111111101
        );
        assert_eq!(
            mask.or_mask,
            0b0000000000000000000000000000000000000000000000000000000001000000
        );
    }
    #[test]
    fn parse_smoke_test() {
        let ugh = DumbMask::default();
        assert_eq!(ugh.and_mask, u64::MAX);
        assert_eq!(ugh.or_mask, 0);

        let mask: DumbMask = "X1X0".parse().unwrap();
        assert_eq!(mask.and_mask, u64::MAX - 1);
        assert_eq!(mask.or_mask, 0b0100);
    }

    #[test]
    fn math_works() {
        let mask: DumbMask = "1XXXX0X".parse().unwrap();
        assert_eq!(11u64 | &mask, 73);
        assert_eq!(101u64 | &mask, 101);
        assert_eq!(0u64 | &mask, 64);
    }
}
