use std::{str::FromStr, io::{BufReader, stdin}};
use std::io::{BufRead};
use std::collections::HashSet;
use regex;

#[derive(Debug, Eq, PartialEq)]
struct Policy{ 
    low: usize,
    high: usize,
    target: char,
}

impl FromStr for Policy {
    type Err = ();

    // example policy line:
    // 3-7 r: mxvlzcjrsqst
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = r"(\d+)-(\d+) ([[:alpha:]])";
        let reg = regex::Regex::new(rx).map_err(|_| ())?;
        let r = reg.captures(s).ok_or(())?;
        let low = r[1].parse().unwrap();
        let high = r[2].parse().unwrap();
        let target: char = r[3].chars().next().unwrap();
        Ok(Policy {
            low,
            high,
            target,
        })
    }
}

impl Policy {
    fn occurrence_check(&self, haystack: &str) -> bool {
        let how_many = haystack.matches(self.target).count();
        (self.low..=self.high).contains(&how_many)
    }

    fn position_check(&self, haystack: &str) -> bool {
        (Some(self.target) == haystack.chars().nth(self.low - 1)) ^
        (Some(self.target) == haystack.chars().nth(self.high - 1))
    }
}

fn main() {
    let r = BufReader::new(stdin());

    let mut occ_ok = 0;
    let mut pos_ok = 0;
    for line in r.lines().filter_map(Result::ok) {
        let mut parts = line.split(':');
        let policy: Policy = parts.next().unwrap().parse().unwrap();
        let test_me = parts.next().unwrap().trim();
        if policy.occurrence_check(test_me) {
            occ_ok += 1;
        }
        if policy.position_check(test_me) {
            pos_ok += 1;
        }
    }

    dbg!(occ_ok);
    dbg!(pos_ok);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one() {
        let test = "3-7 r";
        let pol: Policy = test.parse().unwrap();
        assert_eq!(pol, Policy{low: 3, high: 7, target: 'r'});
    }

    #[test]
    fn test_occ() {
        let pol: Policy = "1-3 c".parse().unwrap();
        assert!(pol.occurrence_check("abc"));
        assert!(pol.occurrence_check("cbc"));
        assert!(pol.occurrence_check("ccc"));
        assert!(!pol.occurrence_check("cccc"));
        assert!(!pol.occurrence_check("abba"));
    }

    #[test]
    fn test_pos() {
        let pol: Policy = "1-3 c".parse().unwrap();
        assert!(pol.position_check("abc"));
        assert!(pol.position_check("cba"));
        assert!(!pol.position_check("cbc"));
        assert!(!pol.position_check("acdc"));
        assert!(!pol.position_check("ec_o"));
    }
}