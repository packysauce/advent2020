use num::clamp;
use std::io::{BufRead, Read};
use std::{
    collections::HashMap,
    io::{stdin, BufReader},
    str::FromStr,
};

fn generate_indices(right: usize, down: usize, width: usize, len: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut cursor = 0;
    let mut pos_in_row = 0;
    while cursor <= len {
        pos_in_row += right;
        let new_down = if pos_in_row >= width {
            // we wrapped, no down
            pos_in_row %= width;
            0
        } else {
            down
        };
        cursor += right + width * new_down;
        indices.push(cursor);
    }
    indices
}

fn do_the_thing(right: usize, down: usize, input: &[u8]) -> usize {
    // think like an image. sure, its a finite width thing,
    // but we dont actually need to go _down_
    // if we smoosh this thing together, and make note of the width
    // we can just traverse it linearly, until our cursor is > length
    // S.X..   width 5, S is start (right 3, down 1, memba?)
    // .X.X.
    // ..X..
    // S.X1..X.X...X..

    let reader = BufReader::new(input);

    let mut width = 0;
    let mut buf: Vec<u8> = Vec::new();
    for line in reader.lines().map(Result::unwrap) {
        let line = line.trim();
        // its the same for every line so whocares
        width = line.len();
        buf.extend(line.as_bytes());
    }

    let indices = generate_indices(right, down, width, buf.len());
    indices
        .into_iter()
        .filter_map(|i| buf.get(i).cloned())
        .filter(|c| *c == b'#')
        .count()
}

fn main() {
    let mut r = BufReader::new(stdin());
    let mut buf = Vec::new();
    r.read_to_end(&mut buf).expect("stdin fucked me");
    let part2: usize = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ]
    .iter()
    .map(|(right, down)| {
        do_the_thing(*right, *down, &buf)
    })
    .inspect(|x| {
        println!("got {} trees", x);
    })
    .product();
    println!("multiplied, we have {} trees", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_the_thing() {
        /*
        to_check = [0, 14, 28, 42, 45]
        O.##....... // 0
        #..O#...#.. // 14
        .#....X..#. // 28
        ..#.#...#O# // 42 - oh shit, right 3 loops us back around on this line before going down, effective increment of
        .?...##..#. // 45 - should be at the question mark
        .!#.##..... // 70 - but we actually end up here!
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";
        */
        //bogart the example from the page! built in unit tests!

        
        let map = br"..##.......
                             #...#...#..
                             .#....#..#.
                             ..#.#...#.#
                             .#...##..#.
                             ..#.##.....
                             .#.#.#....#
                             .#........#
                             #.##...#...
                             #...##....#
                             .#..#...#.#";
        [
            ((1, 1), 2usize),
            ((3, 1), 7usize),
            ((5, 1), 3usize),
            ((7, 1), 4usize),
            ((1, 2), 2usize),
        ]
        .iter()
        .for_each(|((right, down), answer)| {
            assert_eq!(do_the_thing(*right, *down, map), *answer);
        })
    }
}
