use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string(r#"/Users/rahulrav/rust/aoc2021/data/day10.txt"#)?;
    println!("part1 {}", part1(&s));
    println!("part2 {}", part2(&s));
    Ok(())
}

fn corrupted_line(s: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    let matching_bracket = HashMap::from([(']', '['), (')', '('), ('}', '{'), ('>', '<')]);
    let score = HashMap::from([(']', 57), ('}', 1197), (')', 3), ('>', 25137)]);
    for c in s.chars() {
        match c {
            '[' | '<' | '(' | '{' => {
                stack.push(c);
            }
            ']' | '>' | ')' | '}' => {
                let opening_bracket = matching_bracket.get(&c).unwrap();
                if stack[stack.len() - 1] == *opening_bracket {
                    stack.pop();
                } else {
                    return *score.get(&c).unwrap();
                }
            }
            _ => (),
        }
    }
    0
}

fn incomplete_line(s: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    let opening_bracket = HashMap::from([(']', '['), (')', '('), ('}', '{'), ('>', '<')]);
    let closing_bracket = HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);
    let score = HashMap::from([(']', 2), ('}', 3), (')', 1), ('>', 4)]);
    for c in s.chars() {
        match c {
            '[' | '<' | '(' | '{' => {
                stack.push(c);
            }
            ']' | '>' | ')' | '}' => {
                let ob = opening_bracket.get(&c).unwrap();
                if stack[stack.len() - 1] == *ob {
                    stack.pop();
                } else {
                    assert!(false);
                }
            }
            _ => (),
        }
    }
    stack
        .iter()
        .rev()
        .map(|c| {
            closing_bracket
                .get(c)
                .map(|b| score.get(b).unwrap())
                .unwrap()
        })
        .fold(0, |acc, v| acc * 5 + v)
}

fn part1(s: &str) -> usize {
    s.lines().map(corrupted_line).sum()
}

fn part2(s: &str) -> usize {
    let scores = s
        .lines()
        .filter(|l| corrupted_line(l) == 0)
        .map(incomplete_line)
        .sorted()
        .collect_vec();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_corrupted_line() {
        let s = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(corrupted_line(s), 1197);
        let s = "[[<[([]))<([[{}[[()]]]";
        assert_eq!(corrupted_line(s), 3);
        let s = "[{[{({}]{}}([{[{{{}}([]";
        assert_eq!(corrupted_line(s), 57);
        let s = "[<(<(<(<{}))><([]([]()";
        assert_eq!(corrupted_line(s), 3);
        let s = "<{([([[(<>()){}]>(<<{{";
        assert_eq!(corrupted_line(s), 25137);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 26397);
    }

    #[test]
    fn test_incomplete_line() {
        let s = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(incomplete_line(s), 288957);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 288957);
    }
}
