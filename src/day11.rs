use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    fs,
};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string(r#"/Users/rahulrav/rust/aoc2021/data/day11.txt"#)?;
    println!("part1 {}", part1(State::new(&s)));
    println!("part2 {}", part2(State::new(&s)));
    Ok(())
}

struct State {
    octo: Vec<Vec<u8>>,
}

impl State {
    fn new(s: &str) -> Self {
        let octo = s
            .lines()
            .map(|l| l.bytes().map(|c| c - b'0').collect_vec())
            .collect_vec();
        Self { octo }
    }

    fn neighbors(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let n_rows = self.octo.len() as isize;
        let n_cols = self.octo[0].len() as isize;
        (-1..2)
            .cartesian_product(-1..2)
            .filter_map(|(i, j)| {
                let r_ = r as isize + i;
                let c_ = c as isize + j;
                if r_ == r as isize && c_ == c as isize {
                    None
                } else if r_ >= 0 && r_ < n_rows && c_ >= 0 && c_ < n_cols {
                    Some((r_ as usize, c_ as usize))
                } else {
                    None
                }
            })
            .collect_vec()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.octo {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn step(state: &mut State) -> usize {
    let mut to_process: VecDeque<(usize, usize)> = (0..state.octo.len())
        .cartesian_product(0..state.octo[0].len())
        .collect();
    let mut to_flash = HashSet::new();
    while to_process.len() != 0 {
        let (i, j) = to_process.pop_front().unwrap();
        state.octo[i][j] += 1;
        if state.octo[i][j] == 10 {
            to_flash.insert((i, j));
            for (ni, nj) in state.neighbors(i, j).into_iter() {
                to_process.push_back((ni, nj));
            }
        }
    }
    let num_flashes = to_flash.len();
    for (i, j) in to_flash.drain() {
        state.octo[i][j] = 0
    }
    num_flashes
}

fn part1(mut state: State) -> usize {
    (0..100).map(|_| step(&mut state)).sum()
}

fn part2(mut state: State) -> usize {
    let size = state.octo.len() * state.octo[0].len();
    (1..)
        .find_map(|i| {
            if step(&mut state) == size {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_parse() {
        println!("{}", State::new(&TEST_INPUT));
    }

    #[test]
    fn test_neighbors() {
        let state = State::new(&TEST_INPUT);
        println!("{:?}", state.neighbors(0, 0));
        println!("{:?}", state.neighbors(5, 5));
        println!("{:?}", state.neighbors(5, 9));
    }

    #[test]
    fn test_1() {
        let s = "11111
19991
19191
19991
11111";
        let mut state = State::new(&s);
        step(&mut state);
        step(&mut state);
    }

    #[test]
    fn test_part1() {
        let state = State::new(&TEST_INPUT);
        assert_eq!(1656, part1(state));
    }

    #[test]
    fn test_part2() {
        let state = State::new(&TEST_INPUT);
        assert_eq!(195, part2(state));
    }
}
