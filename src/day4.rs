use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("/Users/rahulrav/rust/aoc2021/data/day4.txt")?;
    println!("part1 {}", part1(Bingo::new(&s)));
    println!("part2 {}", part2(Bingo::new(&s)));
    Ok(())
}

#[derive(Debug)]
struct Board {
    board_num: usize,
    grid: HashMap<usize, (usize, usize, bool)>,
    row_state: HashMap<usize, usize>,
    col_state: HashMap<usize, usize>,
}

impl Board {
    fn new(s: &str, n: usize) -> Self {
        let mut m = HashMap::new();
        let mut row_map = HashMap::new();
        let mut col_map = HashMap::new();
        for (row, row_s) in s.lines().enumerate() {
            row_map.insert(row, 0);
            for (col, num) in row_s.split_whitespace().enumerate() {
                col_map.insert(col, 0);
                m.insert(num.parse().unwrap(), (row, col, false));
            }
        }
        Self {
            board_num: n,
            grid: m,
            row_state: row_map,
            col_state: col_map,
        }
    }

    fn mark(&mut self, n: usize) -> bool {
        let v = self.grid.get(&n);
        if v.is_none() {
            return false;
        }
        let &val = v.unwrap();
        let (r, c, _) = val;
        // Mark as seen
        self.grid.insert(n, (r, c, true));
        // Increment row, col counters
        let rs = self.row_state.entry(r).or_insert(0);
        let cs = self.col_state.entry(c).or_insert(0);
        *rs += 1;
        *cs += 1;

        *rs == 5 || *cs == 5
    }

    fn sum_unmarked(&self) -> usize {
        self.grid
            .iter()
            .filter(|(_, (_, _, marked))| !marked)
            .map(|(k, _)| k)
            .sum()
    }
}

#[derive(Debug)]
struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new(s: &str) -> Self {
        let mut split = s.split("\n\n");
        let numbers = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let boards = split.enumerate().map(|(i, s)| Board::new(s, i)).collect();
        Self { numbers, boards }
    }
}

fn part1(mut bingo: Bingo) -> usize {
    for n in bingo.numbers.into_iter() {
        for b in bingo.boards.iter_mut() {
            if b.mark(n) {
                return n * b.sum_unmarked();
            }
        }
    }
    0
}

fn part2(mut bingo: Bingo) -> usize {
    let mut remaining_boards: HashSet<usize> = bingo
        .boards
        .iter()
        .map(|b| b.board_num.to_owned())
        .collect();
    for (_i, &n) in bingo.numbers.iter().to_owned().enumerate() {
        for b in bingo.boards.iter_mut() {
            if !remaining_boards.contains(&b.board_num) {
                continue;
            }
            if b.mark(n) {
                if remaining_boards.len() == 1 {
                    return n * &b.sum_unmarked();
                } else {
                    remaining_boards.remove(&b.board_num);
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    use super::*;
    #[test]
    fn test_board_parse() {
        let s = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";
        println!("{:?}", Board::new(s, 0));
    }

    #[test]
    fn test_parse() {
        println!("{:?}", Bingo::new(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        let bingo = Bingo::new(TEST_INPUT);
        assert_eq!(part1(bingo), 4512);
    }

    #[test]
    fn test_part2() {
        let bingo = Bingo::new(TEST_INPUT);
        assert_eq!(part2(bingo), 1924);
    }
}
