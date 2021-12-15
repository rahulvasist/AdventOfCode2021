use std::fs;
use std::str::FromStr;

const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[derive(Debug, PartialEq)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let command = iter.next().unwrap();
        let val: isize = iter.next().unwrap().parse().unwrap();
        let c = match command {
            "forward" => Command::Forward(val),
            "down" => Command::Down(val),
            "up" => Command::Up(val),
            _ => Command::Down(0),
        };
        Ok(c)
    }
}

fn part1(s: &str) -> isize {
    let commands: Vec<Command> = s.lines().map(|s| s.parse().unwrap()).collect();
    let final_pos = commands.into_iter().fold((0, 0), |acc, command| {
        let (p, d) = acc;
        match command {
            Command::Forward(v) => (p + v, d),
            Command::Up(v) => (p, d - v),
            Command::Down(v) => (p, d + v),
        }
    });
    let (p, d) = final_pos;
    p * d
}

fn part2(s: &str) -> isize {
    let commands: Vec<Command> = s.lines().map(|s| s.parse().unwrap()).collect();
    let final_pos = commands.into_iter().fold((0, 0, 0), |acc, command| {
        let (p, d, a) = acc;
        match command {
            Command::Forward(v) => (p + v, d + (a * v), a),
            Command::Up(v) => (p, d, a - v),
            Command::Down(v) => (p, d, a + v),
        }
    });
    let (p, d, _) = final_pos;
    p * d
}

fn main() -> Result<(), std::io::Error> {
    let s = fs::read_to_string("../data/day2.txt")?;
    println!("{}", part1(&s));
    println!("{}", part2(&s));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let c: Command = "forward 5".parse().unwrap();
        assert_eq!(c, Command::Forward(5));
    }

    #[test]
    fn test2() {
        assert_eq!(150, part1(TEST_INPUT));
    }

    #[test]
    fn test3() {
        assert_eq!(900, part2(TEST_INPUT));
    }
}
