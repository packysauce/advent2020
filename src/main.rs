use std::{str::FromStr, io::{BufReader, stdin}};
use std::io::{BufRead};
use std::collections::HashSet;
use regex;

#[derive(Debug, Eq, PartialEq)]
struct Policy{ 
    low: usize,
    high: usize,
    target: String,
}

impl FromStr for Policy {
    type Err = ();

    // example policy line:
    // 3-7 r: mxvlzcjrsqst
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = r"(\d+)-(\d+) ([[:alpha:]]+)";
        let reg = regex::Regex::new(rx).map_err(|_| ())?;
        let r = reg.captures(s).ok_or(())?;
        let low = r[1].parse().unwrap();
        let high = r[2].parse().unwrap();
        let target: String = r[3].to_string();
        Ok(Policy {
            low,
            high,
            target,
        })
    }
}

fn main() {
    let r = BufReader::new(stdin());

    let mut valid = 0;
    for line in r.lines().filter_map(Result::ok) {
        let mut parts = line.split(':');
        let policy: Policy = parts.next().unwrap().parse().unwrap();
        let test_me = parts.next().unwrap().trim();

        let how_many = test_me.matches(&policy.target).count();
        if (policy.low..=policy.high).contains(&how_many) {
            valid += 1;
        }
    }

    dbg!(valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one() {
        let test = "3-7 r";
        let pol: Policy = test.parse().unwrap();
        assert_eq!(pol, Policy{low: 3, high: 7, target: "r".to_string()});
    }
}