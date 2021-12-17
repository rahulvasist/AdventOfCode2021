#![feature(map_first_last)]

aoc_main::main! {
    year 2021;
    day15 : generator => part_1;
}

mod day15 {
    use std::{
        cmp::Ordering,
        collections::{BTreeSet, HashMap, HashSet},
    };

    use itertools::Itertools;

    type Input = Vec<Vec<usize>>;

    pub fn generator(input: &str) -> Input {
        input
            .lines()
            .map(|l| l.bytes().map(|b| (b - b'0') as usize).collect())
            .collect_vec()
    }

    #[derive(Copy, Clone, Eq, Debug)]
    pub struct Position {
        pub pos: (usize, usize),
        pub dist: usize,
    }

    impl Position {
        pub fn new(x: usize, y: usize) -> Self {
            Self {
                pos: (x, y),
                dist: usize::MAX,
            }
        }

        fn neighbors(&self, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
            let a: [(bool, (isize, isize)); 4] = [
                (self.pos.0 > 0, (-1, 0)),
                (self.pos.0 < max_row - 1, (1, 0)),
                (self.pos.1 > 0, (0, -1)),
                (self.pos.1 < max_col - 1, (0, 1)),
            ];
            a.iter()
                .filter_map(|&(cond, x)| {
                    let x_ = (self.pos.0 as isize + x.0) as usize;
                    let y_ = (self.pos.1 as isize + x.1) as usize;
                    if cond {
                        Some((x_, y_))
                    } else {
                        None
                    }
                })
                .collect_vec()
        }
    }

    impl PartialEq for Position {
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos
        }
    }

    impl PartialOrd for Position {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(other.pos.cmp(&self.pos).then(other.dist.cmp(&self.dist)))
        }
    }

    impl Ord for Position {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    pub fn part_1(input: &Input) -> usize {
        let n_rows = input.len();
        let n_cols = input[0].len();
        let mut process = BTreeSet::new();
        let mut distance = HashMap::new();
        let mut processed = HashSet::new();

        for (i, j) in (0..input.len()).cartesian_product(0..input[0].len()) {
            distance.insert((i, j), usize::MAX);
        }
        let mut pos = Position::new(0, 0);
        pos.dist = input[0][0];
        process.insert(pos);
        distance.insert((0, 0), pos.dist);

        while let Some(pos) = process.pop_first() {
            if pos.pos == (n_rows - 1, n_cols - 1) {
                return pos.dist;
            }
            distance.insert(pos.pos, pos.dist);
            processed.insert(pos.pos);
            dbg!(&processed);
            for n in pos.neighbors(n_rows, n_cols) {
                dbg!(n);
                if processed.contains(&(n.0, n.1)) {
                    continue;
                }
                let current_cost = distance.get(&(n.0, n.1)).unwrap();
                if dbg!(pos.dist + input[n.0][n.1]) < dbg!(*current_cost) {
                    let entry = distance.entry(pos.pos).or_insert(0);
                    *entry = pos.dist + input[n.0][n.1];
                    process.remove(&Position::new(n.0, n.1));
                    let mut new_pos = Position::new(n.0, n.1);
                    new_pos.dist = pos.dist + input[n.0][n.1];
                    process.insert(new_pos);
                    dbg!(&process);
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::day15::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    #[ignore]
    fn test_generator() {
        println!("{:?}", generator(TEST_INPUT));
    }

    #[test]
    #[ignore]
    fn test_part_1() {
        let input = generator(TEST_INPUT);
        assert_eq!(40, part_1(&input));
    }

    #[test]
    fn test_key() {
        let mut tree = BTreeSet::new();
        let pos = Position::new(0, 1);
        tree.insert(pos);
        let mut pos2 = Position::new(0, 1);
        pos2.dist = 100;
        tree.insert(pos2);
        assert_eq!(tree.len(), 1);
    }
}
