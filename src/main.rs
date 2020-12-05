use num::clamp;
use std::cmp::Eq;
use std::hash::Hash;
use std::io::{BufRead, Read};
use std::{
    collections::HashMap,
    io::{stdin, BufReader},
    str::FromStr,
};

const MAX_ROW: u8 = 128;
const MAX_COL: u8 = 8;

#[derive(Debug, Eq, PartialEq)]
struct SeatLocation {
    row: u8,
    column: u8,
    seat_id: u16,
}

impl SeatLocation {
    fn new(s: &str) -> Self {
        assert_eq!(s.len(), 10);
        let row = char_to_bin(&s[0..7], 'F', 'B');
        let column = char_to_bin(&s[7..10], 'L', 'R');
        SeatLocation {
            row,
            column,
            seat_id: row as u16 * MAX_COL as u16 + column as u16,
        }
    }
}

impl std::cmp::PartialOrd for SeatLocation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.seat_id.partial_cmp(&other.seat_id)
    }
}

fn char_to_bin(s: &str, low: char, high: char) -> u8 {
    let mut out = 0;
    for c in s.chars() {
        // using the F/B example, if we have F, push a 0
        // if we have B, push a 1
        match c {
            _ if c == low => out <<= 1,
            _ if c == high => out = out << 1 | 1,
            _ => panic!("found a string with strange runes"),
        }
    }
    out
}

fn main() {
    let mut r = BufReader::new(stdin());
    let mut max_seat = 0;
    for line in r.lines().filter_map(Result::ok) {
        let loc = SeatLocation::new(&line);
        max_seat = max_seat.max(loc.seat_id)
    }

    println!("Highest seat: {}", max_seat);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rows() {
        let known = [
            ("BFFFBBFRRR", 70),
            ("FFFBBBFRRR", 14),
            ("BBFFBBFRLL", 102),
        ];
        for (case, answer) in known.iter() {
            let row = &case[0..7];
            assert_eq!(char_to_bin(row, 'F', 'B'), *answer);
        }
    }

    #[test]
    fn test_cols() {
        let known = [
            ("BFFFBBFRRR", 7),
            ("FFFBBBFRRR", 7),
            ("BBFFBBFRLL", 4),
        ];
        for (case, answer) in known.iter() {
            let row = &case[7..10];
            assert_eq!(case.len(), 10);
            assert_eq!(char_to_bin(row, 'L', 'R'), *answer);
        }
    }

    #[test]
    fn test_locations() {
        let known = [
            ("BFFFBBFRRR", 70, 7, 567),
            ("FFFBBBFRRR", 14, 7, 119),
            ("BBFFBBFRLL", 102, 4, 820),
        ];
        for (case, row, column, seat_id) in known.iter() {
            let rhs = SeatLocation {
                row: *row,
                column: *column,
                seat_id: *seat_id,
            };
            assert_eq!(SeatLocation::new(case), rhs);
        }
    }
}