use std::{collections::HashSet, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string(r#"/Users/rahulrav/rust/aoc2021/data/day9.txt"#)?;
    println!("part1 {}", part1(parse(&s)));
    println!("part2 {}", part2(parse(&s)));
    Ok(())
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn part1(input: Vec<Vec<u32>>) -> u32 {
    let ns: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut risk = 0;
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            let val = input[r][c];
            let mut low_point = true;
            for &(x, y) in ns.iter() {
                let r = r as i32;
                let c = c as i32;
                if (r + x < 0) || (r + x >= input.len() as i32) {
                    continue;
                }
                if (c + y < 0) || (c + y >= input[0].len() as i32) {
                    continue;
                }
                if input[(r + x) as usize][(c + y) as usize] <= val {
                    low_point = false;
                }
            }
            if low_point {
                risk += val + 1;
            }
        }
    }
    risk
}

fn basin_size(
    input: &Vec<Vec<u32>>,
    r: usize,
    c: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    visited.insert((r, c));
    if input[r][c] == 9 {
        return 0;
    }
    let mut rv = 1;
    let ns: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for &(x, y) in ns.iter() {
        let r_ = r as i32 + x;
        let c_ = c as i32 + y;
        if r_ < 0 || r_ >= input.len() as i32 || c_ < 0 || c_ >= input[0].len() as i32 {
            continue;
        }
        if visited.contains(&(r_ as usize, c_ as usize)) {
            continue;
        }
        rv += basin_size(input, r_ as usize, c_ as usize, visited);
    }
    rv
}

fn part2(input: Vec<Vec<u32>>) -> u32 {
    let ns: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut s = Vec::new();
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            let val = input[r][c];
            let mut low_point = true;
            for &(x, y) in ns.iter() {
                let r = r as i32;
                let c = c as i32;
                if (r + x < 0) || (r + x >= input.len() as i32) {
                    continue;
                }
                if (c + y < 0) || (c + y >= input[0].len() as i32) {
                    continue;
                }
                if input[(r + x) as usize][(c + y) as usize] <= val {
                    low_point = false;
                }
            }
            if low_point {
                s.push(basin_size(&input, r, c, &mut HashSet::new()));
            }
        }
    }
    s.sort();
    s.reverse();
    s.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_parse() {
        println!("{:?}", parse(&TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(&TEST_INPUT)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(&TEST_INPUT)), 1134);
    }

    #[test]
    fn test_basin_size() {
        assert_eq!(
            basin_size(&parse(&TEST_INPUT), 0, 1, &mut HashSet::new()),
            3
        );
        assert_eq!(
            basin_size(&parse(&TEST_INPUT), 0, 9, &mut HashSet::new()),
            9
        );
        assert_eq!(
            basin_size(&parse(&TEST_INPUT), 2, 2, &mut HashSet::new()),
            14
        );
    }
}
