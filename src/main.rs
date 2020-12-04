use num::clamp;
use std::cmp::Eq;
use std::hash::Hash;
use std::io::{BufRead, Read};
use std::{
    collections::HashMap,
    io::{stdin, BufReader},
    str::FromStr,
};

trait Passport {
    fn is_valid(&self) -> bool;
    fn country(&self) -> Result<String, ()>;
}

fn valid_year(s: &str, min: u16, max: u16) -> bool {
    if s.len() != 4 {
        return false
    }
    let x: u16 = s.parse().unwrap();
    min <= x && x <= max
}

fn valid_height(s: &str) -> bool {
    // lol bad data go brrr
    let re = regex::Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let caps = re.captures(s);
    let captures = if let Some(x) = caps {
        x
    } else {
        println!("bad height: {}", s);
        return false
    };
    let height: u16 = captures.get(1).unwrap().as_str().parse().unwrap();
    let unit = captures.get(2).unwrap().as_str();
    match unit {
        "cm" => 150 <= height && height <= 193,
        "in" => 59 <= height && height <= 76,
        _ => panic!("bad height: {}", s),
    }
}

// racist
fn valid_hair(s: &str) -> bool {
    let re = regex::Regex::new("#[0-9a-fA-F]{6}").unwrap();
    re.find(s).is_some()
}

// oof, maybe worse?
fn valid_eyes(s: &str) -> bool {
    [
       "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
    ].contains(&s)
}

fn valid_pid(s: &str) -> bool {
    let re = regex::Regex::new(r"\d{9}").unwrap();
    re.find(s).is_some()
}

impl Passport for HashMap<&str, &str> {
    fn is_valid(&self) -> bool {
        // simple day 1 check 
        let fields_present = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|field| self.contains_key(field));

        let fields_valid = self.iter()
            .all(|(field, value)| {
                match *field {
                    "byr" => valid_year(*value, 1920, 2002),
                    "iyr" => valid_year(*value, 2010, 2020),
                    "eyr" => valid_year(*value, 2020, 2030),
                    "hgt" => valid_height(*value),
                    "hcl" => valid_hair(*value),
                    "ecl" => valid_eyes(*value),
                    "pid" => valid_pid(*value),
                    "cid" => true,
                    _ => false,
                }
            });
        
        fields_present && fields_valid
    }

    fn country(&self) -> Result<String, ()> {
        self.get("cid").cloned().ok_or(()).map(str::to_string)
    }
}

fn parse_passports(input: &str) -> Vec<HashMap<&str, &str>> {
    let passport_data: Vec<_> = input.split("\r\n\r\n").collect();
    // now we have individual chunks of shit separated by whitespace
    let mut passports = Vec::new();
    let mut count = 0;
    for data in passport_data {
        count += 1;
        let parts = data.split_whitespace();
        let chunk = parts
            .map(|s| {
                let mut field = s.split(':');
                (field.next().unwrap().trim(), field.next().unwrap().trim())
            })
            .collect::<HashMap<_, _>>();
        passports.push(chunk)
    }
    dbg!(count);
    passports
}

fn main() {
    let mut r = BufReader::new(stdin());
    let mut buf = String::new();
    // when i can think of a good way to stream this, i will
    r.read_to_string(&mut buf).unwrap();
    dbg!(buf.len());
    let passports = parse_passports(&buf);
    let thing: usize = passports
        .iter()
        .map(|e| e.is_valid() as usize)
        .sum();
    println!("valid passports! {}", thing);
}

#[cfg(test)]
#[test]
fn test_the_thing() {
    let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
    byr:1937 iyr:2017 cid:147 hgt:183cm\n\
    \n\
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
    hcl:#cfa07d byr:1929\n\
    \n\
    hcl:#ae17e1 iyr:2013\n\
    eyr:2024\n\
    ecl:brn pid:760753108 byr:1931\n\
    hgt:179cm\n\
    \n\
    hcl:#cfa07d eyr:2025 pid:166559648\n\
    iyr:2011 ecl:brn hgt:59in";

    let d = parse_passports(data);
    assert_eq!(d.len(), 4);
}
