use num::clamp;
use std::cmp::Eq;
use std::hash::Hash;
use std::io::{BufRead, Read};
use std::{
    collections::HashMap,
    io::{stdin, BufReader},
    str::FromStr,
};

fn crunch_batch(batch: &mut dyn BufRead) -> Vec<u32> {
    let mut out = Vec::new();
    let mut lines = batch.lines();
    let mut acc = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            out.push(acc);
            acc = 0;
        } else {
            acc |= str_to_bin(line.trim());
        }
    }
    out.push(acc);

    out
}

fn main() {
    let mut r = BufReader::new(stdin());
    let answers = crunch_batch(&mut r);

    let yeses: u32 = answers
        .iter()
        .map(|i| i.count_ones())
        .sum();
    println!("total yes: {}", yeses);
}

fn char_to_bin(c: char) -> u32 {
    c.to_digit(36).unwrap() - 10
}

fn str_to_bin(s: &str) -> u32 {
    s.chars()
        .map(char_to_bin)
        .fold(0, |init, c| {
            init | (1 << c)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_bin() {
        assert_eq!(str_to_bin("abcd"), 0b1111);
        assert_eq!(str_to_bin("ace"), 0b10101);
        assert_eq!(str_to_bin("abz"), 0b10000000000000000000000011);
    }

    #[test]
    fn test_char_to_bin() {
        assert_eq!(char_to_bin('a'), 0);
        assert_eq!(char_to_bin('z'), 25);
    }

    #[test]
    fn test_examples() {
        let data = "abc

                        a
                        b
                        c

                        ab
                        ac

                        a
                        a
                        a
                        a

                        b\n";
        let mut buf = std::io::Cursor::new(data); 
        let results = crunch_batch(&mut buf);
        assert_eq!(results, vec![0b111, 0b111, 0b111, 0b1, 0b10]);
    }
}