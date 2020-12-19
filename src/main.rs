use std::collections::HashMap;

use lazy_static::lazy_static;
use num::Integer;

#[derive(Debug)]
struct DumbMask {
    and_mask: u64,
    or_mask: u64,
    floating_bits: Vec<usize>,
}

impl Default for DumbMask {
    fn default() -> Self {
        DumbMask {
            and_mask: u64::MAX,
            or_mask: 0,
            floating_bits: Vec::new(),
        }
    }
}

impl DumbMask {
    fn value_mask(&self, rhs: u64) -> u64 {
        rhs & self.and_mask | self.or_mask
    }

   fn address_mask(&self, rhs: usize) -> Vec<usize> {
        let mut masks = Vec::new();
        for pos in 0..(1 << self.floating_bits.len()) {
            let mut tmp = rhs as u64 | self.or_mask;
            for (i, replace_i) in self.floating_bits.iter().enumerate() {
                let modify = 1 << replace_i;
                if pos & (1 << i) != 0 {
                    tmp |= modify;
                } else {
                    tmp &= !modify;
                };
            }
            masks.push(tmp as usize);
        }
        masks
    }
}

impl std::str::FromStr for DumbMask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        let reversed = s.chars().rev().collect::<String>();
        let mut mask = DumbMask::default();
        mask.floating_bits = reversed.match_indices('X').map(|(bit, _)| bit).collect();

        for (i, c) in reversed.chars().enumerate() {
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

type Memory = HashMap<usize, u64>;

fn go(data: &str) -> (Memory, Memory) {
    let mut day1: HashMap<usize, u64> = HashMap::new();
    let mut day2: HashMap<usize, u64> = HashMap::new();

    let commands = data
        .lines()
        .map(|s| s.parse::<Command>())
        .filter_map(Result::ok);

    let mut mask = DumbMask::default();
    for command in commands {
        match command {
            Command::SetMask(new_mask) => mask = new_mask,
            Command::SetMem(loc, value) => {
                let new_value = mask.value_mask(value);
                day1
                    .entry(loc)
                    .and_modify(|f| *f = new_value)
                    .or_insert(new_value);
                for loc in mask.address_mask(loc) {
                    day2.entry(loc)
                        .and_modify(|f| *f = value)
                        .or_insert(value);
                }
            }
        }
    }
    (day1, day2)
}

fn main() {
    /*
    let data = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                     mem[8] = 11\n\
                     mem[7] = 101\n\
                     mem[8] = 0";
                     */
    let data = include_str!("../inputs/day14.txt");

    let (day1, day2) = go(data);

    println!("day1 sum is {}", day1.values().sum::<u64>());
    println!("day2 sum is {}", day2.values().sum::<u64>());
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
    fn test_machine() {
        let data = "mask = 000000000000000000000000000000X1001X\n\
                         mem[42] = 100\n\
                         mask = 00000000000000000000000000000000X0XX\n\
                         mem[26] = 1";

        let (_, addr_mem) = go(data);
        assert_eq!(addr_mem.get(&16), Some(&1));
        assert_eq!(addr_mem.get(&17), Some(&1));
        assert_eq!(addr_mem.get(&18), Some(&1));
        assert_eq!(addr_mem.get(&19), Some(&1));
        assert_eq!(addr_mem.get(&24), Some(&1));
        assert_eq!(addr_mem.get(&25), Some(&1));
        assert_eq!(addr_mem.get(&26), Some(&1));
        assert_eq!(addr_mem.get(&27), Some(&1));
        assert_eq!(addr_mem.get(&58), Some(&100));
        assert_eq!(addr_mem.get(&59), Some(&100));
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
    fn address_mask_works() {
        let mask: DumbMask = "X1001X".parse().unwrap();
        assert_eq!(mask.address_mask(0b101010), vec![
            0b011010,
            0b011011,
            0b111010,
            0b111011,
        ]);

        let mask: DumbMask = "X0XX".parse().unwrap();
        assert_eq!(mask.address_mask(26), vec![
            0b10000,
            0b10001,
            0b10010,
            0b10011,
            0b11000,
            0b11001,
            0b11010,
            0b11011,
        ])
    }

    #[test]
    fn math_works() {
        let mask: DumbMask = "1XXXX0X".parse().unwrap();
        assert_eq!(mask.value_mask(11), 73);
        assert_eq!(mask.value_mask(101), 101);
        assert_eq!(mask.value_mask(0), 64);
    }
}
