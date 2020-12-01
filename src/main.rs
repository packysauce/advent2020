use std::io::{BufReader, stdin};
use std::io::{Read, BufRead};
use std::collections::HashSet;

fn main() {
    let mut numbers: Vec<i32> = Default::default();
    let r = BufReader::new(stdin());
    for line in r.lines() {
        if let Ok(line) = line {
            numbers.push(line.trim().parse().unwrap())
        }
    }
    for number1 in numbers.iter() {
        let start = 2020 - number1;
        for number in numbers.iter() {
            let result = start - number;
            if numbers.contains(&result) {
                println!("{} {} {} = {}", number1, number, result,  number1 * number * result);
                println!("{} {} {} | {} , {}", number1, number, result, number1 + number + result, number1 * number * result);
            }
        }
    }
}
