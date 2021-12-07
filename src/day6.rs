use std::fs;
use std::{collections::HashMap, error};

fn descendants(timer: u64, tick: u64, last_day: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if tick + timer > last_day {
        return 1;
    }
    if let Some(&v) = cache.get(&(tick, timer)) {
        return v;
    }

    let mut rv: u64 = 1;
    let mut m_tick = tick;
    let mut m_timer = timer;
    while m_tick + m_timer < last_day {
        m_tick = m_tick + m_timer + 1;
        rv += descendants(8, m_tick, last_day, cache);
        m_timer = 6;
    }
    cache.insert((tick, timer), rv);
    rv
}

fn part1(initial: Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    initial
        .iter()
        .map(|&i| descendants(i, 0, 80, &mut cache))
        .sum()
}

fn part2(initial: Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    initial
        .iter()
        .map(|&i| descendants(i, 0, 256, &mut cache))
        .sum()
}

fn parse(s: &str) -> Vec<u64> {
    s.trim().split(',').filter_map(|s| s.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_1() {
        assert_eq!(descendants(3, 0, 1, &mut HashMap::new()), 1);
        assert_eq!(descendants(3, 0, 4, &mut HashMap::new()), 2);
        assert_eq!(descendants(3, 0, 10, &mut HashMap::new()), 2);
        assert_eq!(descendants(3, 0, 11, &mut HashMap::new()), 3);
        assert_eq!(descendants(3, 0, 14, &mut HashMap::new()), 4);
    }

    #[test]
    fn test_part1() {
        let initial = vec![3, 4, 3, 1, 2];
        assert_eq!(part1(initial), 5934);
    }

    #[test]
    fn test_parse() {
        let initial = vec![3, 4, 3, 1, 2];
        assert_eq!(parse(&TEST_INPUT), initial);
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let s = fs::read_to_string("/Users/rahulrav/rust/aoc2021/data/day6.txt")?;
    println!("Part1={}", part1(parse(&s)));
    println!("Part2={}", part2(parse(&s)));
    Ok(())
}
