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

fn main() {
    let mut r = BufReader::new(stdin());
    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();

    let lines = data
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect::<Vec<u32>>();
    let preamble_len = 25;

    let out = lines.windows(preamble_len + 1)
        .filter_map(|s| {
            if valid_xmas(s).is_none() {
                s.last()
            } else {
                None
            }
        });

    for i in out {
        println!("first bad one is {}", i);
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
