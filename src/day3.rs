use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Number = Vec<u8>;
type Report = HashSet<Number>;

fn parse(s: &str) -> Report {
    let r: Report = s
        .lines()
        .map(|l| {
            l.as_bytes()
                .into_iter()
                .map(|&b| if b == b'0' { 0 } else { 1 })
                .collect()
        })
        .collect();
    r
}

fn get_counters(r: &Report) -> HashMap<(u64, u8), u64> {
    let mut counter: HashMap<(u64, u8), u64> = HashMap::new();
    let num_bits = r.iter().next().unwrap().len() as u64;
    for bit in 0..num_bits {
        counter.insert((bit, 0), 0);
        counter.insert((bit, 1), 0);
        for number in r.iter() {
            let value = number[bit as usize];
            let &count = counter.get(&(bit, value)).unwrap();
            counter.insert((bit, value), count + 1);
        }
    }
    counter
}

fn part1(r: &Report) -> u64 {
    let counter = get_counters(&r);
    let num_bits = r.iter().next().unwrap().len();
    let gamma = (0..num_bits)
        .into_iter()
        .map(|bit| {
            if counter.get(&(bit as u64, 1)) > counter.get(&(bit as u64, 0)) {
                1
            } else {
                0
            }
        })
        .fold(0, |acc, x| (acc << 1) | x);
    let epsilon = (0..num_bits)
        .into_iter()
        .map(|bit| {
            if counter.get(&(bit as u64, 1)) < counter.get(&(bit as u64, 0)) {
                1
            } else {
                0
            }
        })
        .fold(0, |acc, x| (acc << 1) | x);
    gamma * epsilon
}

fn vec_to_num(n: &Number) -> u64 {
    n.into_iter().fold(0, |acc, &x| (acc << 1 | x as u64))
}

fn oxygen(mut r: Report) -> u64 {
    let mut bit = 0 as u64;
    while r.len() != 1 {
        let counter = get_counters(&r);
        let val = if counter.get(&(bit, 0)) > counter.get(&(bit, 1)) {
            0
        } else {
            1
        };
        r.retain(|n| n[bit as usize] == val);
        bit += 1;
    }
    vec_to_num(r.iter().next().unwrap())
}

fn co2(mut r: Report) -> u64 {
    let mut bit = 0 as u64;
    while r.len() != 1 {
        let counter = get_counters(&r);
        let val = if counter.get(&(bit, 1)) < counter.get(&(bit, 0)) {
            1
        } else {
            0
        };
        r.retain(|n| n[bit as usize] == val);
        bit += 1;
    }
    vec_to_num(r.iter().next().unwrap())
}

fn part2(r: &Report) -> u64 {
    oxygen(r.clone()) * co2(r.clone())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("../data/day3.txt")?;
    println!("part1 {}", part1(&parse(&s)));
    println!("part2 {}", part2(&parse(&s)));
    dbg!(part1(&parse(&s)));
    Ok(())
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    use super::*;
    #[test]
    fn test_parse() {
        let p = parse(&TEST_INPUT);
        // println!("{:?}", p);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&TEST_INPUT)), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&TEST_INPUT)), 230);
    }
}
