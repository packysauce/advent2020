use num::clamp;
use std::cmp::Eq;
use std::hash::Hash;
use std::io::{BufRead, Read};
use std::{
    collections::HashMap,
    io::{stdin, BufReader},
    str::FromStr,
};

fn anybody_said_yes(batch: &mut dyn BufRead) -> Vec<u32> {
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

fn everybody_said_yes(batch: &mut dyn BufRead) -> Vec<u32> {
    let mut out = Vec::new();
    let mut lines = batch.lines();
    let mut acc = u32::MAX;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            out.push(acc);
            acc = u32::MAX;
        } else {
            acc &= str_to_bin(line.trim());
        }
    }
    out.push(acc);

    out
}



fn main() {
    let mut r = BufReader::new(stdin());
    let mut buf = Vec::new();
    r.read_to_end(&mut buf).unwrap();
    use std::io::Cursor;
    let anybody = anybody_said_yes(&mut Cursor::new(&buf));
    let everybody = everybody_said_yes(&mut Cursor::new(&buf));

    let anybody_yes: u32 = anybody
        .iter()
        .map(|i| i.count_ones())
        .sum();

    let everybody_yes: u32 = everybody
        .iter()
        .map(|i| i.count_ones())
        .sum();

    println!("anybody: {}, everybody: {}", anybody_yes, everybody_yes);
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
    fn test_anybody_said_yes() {
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
        let results = anybody_said_yes(&mut buf);
        assert_eq!(results, vec![0b111, 0b111, 0b111, 0b1, 0b10]);
    }

    #[test]
    fn test_everybody_said_yes() {
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
        let results = everybody_said_yes(&mut buf);
        assert_eq!(results, vec![0b111, 0, 0b1, 0b1, 0b10]);
    }
}