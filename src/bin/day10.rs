use std::collections::HashMap;
use std::io::BufRead;
use std::io::{stdin, BufReader, Read};
use thiserror::Error;

fn main() {
    /*
    let mut r = BufReader::new(stdin());
    let mut data = String::new();
    r.read_to_string(&mut data).unwrap();
    */
    let data = include_str!("../inputs/day10.txt");
    let data: Vec<_> = data.lines().filter_map(|s| s.parse::<u8>().ok()).collect();
    let dist = get_distribution(&data);

    for (k, v) in dist.iter() {
        println!("{}: {}", k, v);
    }
    let out = dist.get(&1u8).unwrap() * dist.get(&3u8).unwrap();
    println!("result {}", out);
    let out = total_permutations(&data);
    println!("total perms: {}", out);
}

fn get_distribution(jolts: &[u8]) -> HashMap<u8, i32> {
    let mut data = vec![0u8];
    data.extend_from_slice(jolts);
    data.sort_unstable();
    data.push(data.last().unwrap() + 3);
    data.windows(2).fold(HashMap::new(), |mut e, r| {
        let diff = r[1] - r[0];
        e.entry(diff).and_modify(|x| *x += 1).or_insert(1);
        e
    })
}

fn total_permutations(data: &[u8]) -> u64 {
    let mut data = data.to_vec();
    data.push(0);
    data.sort_unstable();
    data.push(data.last().unwrap() + 3);

    let trib = [1,1,2,4,7];

    let diffs = data.iter().zip(&data[1..])
        .map(|(prev, current)| {
            current - prev
        })
        .collect::<Vec<u8>>();

    diffs
        .split(|diff| *diff == 3)
        .filter(|diffs| !diffs.is_empty())
        .map(|ones| ones.len())
        .map(|i| trib[i])
        .product()
}

#[cfg(test)]
mod tests {
    const DATA: [u8; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    use super::*;

    #[test]
    fn test_joltage_dist() {
        let dist = get_distribution(&DATA);
        assert_eq!(dist.get(&1u8).cloned(), Some(7));
        assert_eq!(dist.get(&3u8).cloned(), Some(5));
    }

    #[test]
    fn larger_example() {
        let data = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        
        let dist = get_distribution(&data);
        assert_eq!(dist.get(&1u8).cloned(), Some(22));
        assert_eq!(dist.get(&3u8).cloned(), Some(10));
    }

    #[test]
    fn total_perms() {
        let data = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let r = total_permutations(&data);

        assert_eq!(19208, r);
    }
}
