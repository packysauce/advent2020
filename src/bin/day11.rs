use std::collections::{binary_heap::Iter, HashMap};
use std::io::BufRead;
use std::io::{stdin, BufReader, Read};
use thiserror::Error;

type Tile = u8;

#[derive(Eq, PartialEq)]
struct SeatMap {
    map: Vec<u8>,
    width: isize,
}

impl std::ops::Deref for SeatMap {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl SeatMap {
    fn new(map: &[u8], width: isize) -> SeatMap {
        SeatMap {
            map: map.to_vec(),
            width,
        }
    }

    fn transform_coord(x: isize, y: isize, width: isize, height: isize) -> Option<usize> {
        // simple box check
        if x >= 0 && y >= 0 && x < width && y < height {
            Some((x + y * width) as usize)
        } else {
            None
        }
    }

    fn sightlines(&self, idx: usize) -> impl Iterator<Item=impl Iterator<Item=usize> + '_> + '_ /*<(usize, Tile)>*/ {
        let width = self.width;
        let height = self.map.len() as isize / self.width;
        let mut x = idx as isize % width;
        let mut y = idx as isize / width;

        let directions = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), /*skip middle*/ (1, 0),
            (-1, 1), (0, 1), (1, 1)
        ];

        directions
            .into_iter()
            .map(move |(dir_x, dir_y)| {
                std::iter::from_fn(move || {
                    x += dir_x;
                    y += dir_y;
                    SeatMap::transform_coord(x, y, width, height)
                })
            })
    }

    fn visible(&self, idx: usize) -> impl Iterator<Item=(usize, Tile)> + '_ {
        self.sightlines(idx)
            .flat_map(move |rays| {
                rays
                    .map(move |idx| (idx, self.map[idx]))
                    .skip_while(|(_, tile)| *tile == b'.')
                    .take(1)
            })
    }

    fn neighbors(&self, idx: usize) -> Vec<(usize, Tile)> {
        self
            .sightlines(idx)
            .filter_map(|mut i| i.next())
            .map(|idx| {
                (idx, self.map[idx])
            })
            .collect()
    }

    fn occupied(&self) -> usize {
        self.iter().filter(|&c| *c == b'#').count()
    }

    fn occupied_visible(&self, idx: usize) -> usize {
        self.visible(idx)
            .filter(|(_, tile)| *tile == b'#')
            .count()
    }

    fn occupied_near(&self, idx: usize) -> usize {
        self.neighbors(idx)
            .iter()
            .filter(|(_, tile)| *tile == b'#')
            .count()
    }

    fn step(&self) -> SeatMap {
        let map = self
            .iter()
            .enumerate()
            .map(|(idx, seat)| {
                let occupied = self.occupied_visible(idx);
                match *seat {
                    b'L' if occupied == 0 => b'#',
                    b'#' if occupied >= 5 => b'L',
                    x => x,
                }
            })
            .collect();
        SeatMap {
            map,
            width: self.width,
        }
    }

    // consume it because it cant change after this..
    // kinda neat sounding!
    fn finalize(self) -> SeatMap {
        let mut prev = self.step();
        let mut next = prev.step();
        while next != prev {
            let tmp = next.step();
            prev = next;
            next = tmp;
        }
        next
    }
}

fn main() {
    let data = include_bytes!("../inputs/day11.txt");
    let lines = data.split(|c| *c == b'\n');
    let mut data: Vec<u8> = Vec::new();
    let mut width = 0;
    for line in lines {
        width = width.max(line.len() as isize);
        data.extend(line)
    }
    let map = SeatMap::new(&data, width);

    // let the record show that i got distracted thinking about whether or not
    // ^^ that Vec is gonna copy stuff
    let map = map.finalize();
    println!("final count: {}", map.occupied());

}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &[u8] = b"L.LL.LL.LL\
        LLLLLLL.LL\
        L.L.L..L..\
        LLLL.LL.LL\
        L.LL.LL.LL\
        L.LLLLL.LL\
        ..L.L.....\
        LLLLLLLLLL\
        L.LLLLLL.L\
        L.LLLLL.LL";

    #[test]
    fn visible() {
        const SIMPLE: &[u8] = b".......#.\
                                ...#.....\
                                .#.......\
                                .........\
                                ..#L....#\
                                ....#....\
                                .........\
                                #........\
                                ...#.....";
        let map = SeatMap::new(SIMPLE, 9);
        assert_eq!(map.occupied_visible(39), 8);
        let map = SeatMap::new(b".............\
                                             .L.L.#.#.#.#.\
                                             .............", 13);
        assert_eq!(map.occupied_visible(14), 0);
        assert_eq!(map.visible(14).collect::<Vec<_>>(), vec![(16, b'L')]);
    }

    #[test]
    fn sightline() {
        let map = SeatMap::new(DATA, 10);
        let r = map.sightlines(0)
           .flatten()
           .count();

        assert_eq!(r, 27);
    }

    #[test]
    fn ray_len_correct() {
        let map = SeatMap::new(DATA, 10);
        let rays = map.sightlines(0);

        assert_eq!(rays.count(), 8);
    }

    #[test]
    fn top_left_sightline() {
        let map = SeatMap::new(DATA, 10);
        let mut rays = map.sightlines(0);
        // up + left
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![]);
        // up
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![]);
        // up + right
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![]);
        // left
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![]);
        // right
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![1,2,3,4,5,6,7,8,9]);
        // down + left
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![]);
        // down 
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);
        // down + right
        assert_eq!(rays.next().unwrap().collect::<Vec<usize>>(), vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
        // done!
        assert!(rays.next().is_none());
    }

    const STEPS: [&[u8]; 2] = [
       b"#.##.##.##\
         #######.##\
         #.#.#..#..\
         ####.##.##\
         #.##.##.##\
         #.#####.##\
         ..#.#.....\
         ##########\
         #.######.#\
         #.#####.##",
       b"#.LL.L#.##\
         #LLLLLL.L#\
         L.L.L..L..\
         #LLL.LL.L#\
         #.LL.LL.LL\
         #.LLLL#.##\
         ..L.L.....\
         #LLLLLLLL#\
         #.LLLLLL.L\
         #.#LLLL.##"
    ];

    const END: &[u8] =
       b"#.#L.L#.##\
         #LLL#LL.L#\
         L.#.L..#..\
         #L##.##.L#\
         #.#L.LL.LL\
         #.#L#L#.##\
         ..L.L.....\
         #L#L##L#L#\
         #.LLLLLL.L\
         #.#L#L#.##";

    // protection from myself
    #[test]
    fn test_sample_data_isnt_fucked() {
        assert_eq!(DATA.len(), 100);
    }

    #[test]
    fn test_stepper() {
        let mut map = SeatMap::new(DATA, 10);
        for step in STEPS.iter() {
            map = map.step();
            assert_eq!(step, &map.map);
        }
    }

    #[test]
    fn test_finalizer() {
        let map = SeatMap::new(DATA, 10).finalize();
        assert_eq!(&END, &map.map);
        assert_eq!(map.occupied(), 37);
    }

    #[test]
    fn get_it_right_already() {
        let map = SeatMap::new(&DATA, 10);
        assert_eq!(
            map.neighbors(22),
            vec![
                (11, b'L'),
                (12, b'L'),
                (13, b'L'),
                (21, b'.'),
                (23, b'.'),
                (31, b'L'),
                (32, b'L'),
                (33, b'L'),
            ]
        )
    }

    #[test]
    fn test_edges() {
        let map = SeatMap::new(&DATA, 10);
        assert_eq!(map.neighbors(0), vec![(1, b'.'), (10, b'L'), (11, b'L'),]);

        assert_eq!(map.neighbors(9), vec![(8, b'L'), (18, b'L'), (19, b'L'),]);
    }

    #[test]
    fn test_counts() {
        const DATA: &[u8] = b"L#L\
            #L#\
            L.L";

        let map = SeatMap::new(&DATA, 3);
        let counts = vec![2, 2, 2, 1, 3, 1, 1, 2, 1];

        for (idx, answer) in counts.iter().enumerate() {
            assert_eq!(map.occupied_near(idx), *answer)
        }
    }

    #[test]
    fn test_neighbors() {
        let map = SeatMap::new(&DATA, 10);
        assert_eq!(
            map.neighbors(11),
            vec![
                (0, b'L'),
                (1, b'.'),
                (2, b'L'),
                (10, b'L'),
                (12, b'L'),
                (20, b'L'),
                (21, b'.'),
                (22, b'L'),
            ]
        );
    }
}
