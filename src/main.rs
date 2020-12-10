use std::io::BufRead;
use std::io::{stdin, BufReader, Read};
use thiserror::Error;

/// Returns an Option containing the "partner" number
/// if this is a valid XMAS number
fn valid_xmas(window: &[u32]) -> Option<(u32, u32)> {
    let needle = window.last().cloned()?;
    let preamble = &window[0..window.len() - 1];
    for i in 0..preamble.len() {
        let partner = if let Some(x) = needle.checked_sub(preamble[i]) {
            x
        } else {
            continue
        };

        if let Some((_idx, x)) = preamble.iter().enumerate().find(|(j, &x)| {
            if *j == i {
                false // skip currently looked-at number
            } else {
                x == partner
            }
        }) {
            return Some((needle - *x, *x));
        }
    }
    None
}

fn find_subsum(window: &[u32], needle: u32) -> Option<Vec<u32>> {
    // if window is 10 things long, we wanna take 2, then 3, then 4, etc
    // until we find a sum equalling the needle
    for window_length in 2..window.len() {
        for subwindow in window.windows(window_length) {
            let subsum: u64 = subwindow.iter().map(|s| *s as u64).sum();
            if subsum == needle as u64 {
                return Some(subwindow.to_vec())
            }
        }
    }
    None
}

fn main() {
    /*
    let mut r = BufReader::new(stdin());
    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();
    */
    let data = include_str!("../inputs/day9.txt");

    let lines = data
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect::<Vec<u32>>();
    let preamble_len = 25;

    let out = lines.windows(preamble_len + 1)
        .filter_map(|s| {
            if valid_xmas(s).is_none() {
                s.last().cloned()
            } else {
                None
            }
        });

    for i in out {
        println!("first bad one is {}", i);
        if let Some(x) = find_subsum(&lines, i) {
            println!("{:?}", x);
            let min = x.iter().min().unwrap();
            let max = x.iter().max().unwrap();
            println!("... key is {}", min + max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    lazy_static::lazy_static! {
        static ref DATA: Vec<u32> = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];
    }

    #[test]
    fn subsum_thing_works() {
        let data = vec![2,3,4,5,6,8,9,10,11];
        let result = find_subsum(&data, 7);
        assert_eq!(result, Some(vec![3,4]));
    }

    #[test]
    fn test_valid_number() {
        let preamble_len = 5;
        let mut windows = DATA.windows(preamble_len + 1);
        let known = (15, 25);
        assert_eq!(valid_xmas(windows.next().unwrap()), Some(known));

        /*
        for segment_data in DATA.windows(preamble_len + 1) {
            let len = segment_data.len() - 1;
            let preamble = &segment_data[0..len];
            let r = valid_number(*segment_data.last().unwrap(), preamble);
            dbg!(r);
        }*/
    }
}
