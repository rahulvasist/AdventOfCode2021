use std::fs;
use std::{cmp, collections::HashMap, error};

type Point = (isize, isize);
type LS = (Point, Point);

peg::parser! {
  grammar line_parser() for str {
    rule number() -> isize
      = n:$(['0'..='9']+) {? n.parse().or(Err("isize")) }

    rule point() -> Point
          = x:number() "," y:number() { (x, y) }

    pub rule ls() -> LS
        = p1:point() " -> " p2:point() { (p1, p2) }
  }
}

fn on_ls(ls: LS, p: Point) -> bool {
    //                3   -  9     > 0   !=  3   - 3      > 0
    let x_in_range = (p.0.cmp(&ls.0 .0) != p.0.cmp(&ls.1 .0)) || (p.0 == ls.0 .0 && p.0 == ls.1 .0);
    let y_in_range = (p.1.cmp(&ls.0 .1) != p.1.cmp(&ls.1 .1)) || (p.1 == ls.0 .1 && p.1 == ls.1 .1);
    let within_range = x_in_range && y_in_range;
    //dbg!(ls, p, x_in_range, y_in_range, within_range);
    if ls.0 .0 == ls.1 .0 {
        return within_range;
    } else if ls.0 .1 == ls.1 .1 {
        return within_range;
        // let slope: isize = (ls.1 .1 - ls.0 .1) / (ls.1 .0 - ls.0 .0);
        // let c: isize = ls.0 .1 - slope * ls.0 .0;
        // return within_range && (p.1 == (slope * p.0 + c));
    }
    false
}

fn on_ls2(ls: LS, p: Point) -> bool {
    //                3   -  9     > 0   !=  3   - 3      > 0
    let x_in_range = (p.0.cmp(&ls.0 .0) != p.0.cmp(&ls.1 .0)) || (p.0 == ls.0 .0 && p.0 == ls.1 .0);
    let y_in_range = (p.1.cmp(&ls.0 .1) != p.1.cmp(&ls.1 .1)) || (p.1 == ls.0 .1 && p.1 == ls.1 .1);
    let within_range = x_in_range && y_in_range;
    //dbg!(ls, p, x_in_range, y_in_range, within_range);
    if ls.0 .0 == ls.1 .0 {
        within_range
    } else {
        let slope: isize = (ls.1 .1 - ls.0 .1) / (ls.1 .0 - ls.0 .0);
        let c: isize = ls.0 .1 - slope * ls.0 .0;
        within_range && (p.1 == (slope * p.0 + c))
    }
}

fn max3(a: isize, b: isize, c: isize) -> isize {
    cmp::max(a, cmp::max(b, c))
}

fn parse(s: &str) -> Vec<LS> {
    s.lines().map(|s| line_parser::ls(s).unwrap()).collect()
}

fn part1(ls: Vec<LS>) -> usize {
    let (max_x, max_y): (isize, isize) = ls.iter().fold((0, 0), |(acc_x, acc_y), &(p1, p2)| {
        (max3(acc_x, p1.0, p2.0), max3(acc_y, p1.1, p2.1))
    });
    //dbg!(max_x, max_y);

    let mut point_score: HashMap<(isize, isize), isize> = HashMap::new();

    for i in 0..=max_x {
        for j in 0..=max_y {
            for &line_seg in ls.iter() {
                if (line_seg.0 .0 == line_seg.1 .0 || line_seg.0 .1 == line_seg.1 .1) && on_ls(line_seg, (i, j)) {
                    let cur_score = point_score.entry((i, j)).or_insert(0);
                    *cur_score += 1;
                    //dbg!(i, j, line_seg, *cur_score);
                }
            }
        }
    }
    // //dbg!(&point_score);
    // for i in 0..=max_x {
    //     for j in 0..=max_y {
    //         match point_score.get(&(i, j)) {
    //             Some(v) => print!("{}", v),
    //             None => print!("."),
    //         }
    //     }
    //     print!("\n");
    // }

    point_score.iter().filter(|&(_k, v)| *v >= 2).count()
}

fn part2(ls: Vec<LS>) -> usize {
    let (max_x, max_y): (isize, isize) = ls.iter().fold((0, 0), |(acc_x, acc_y), &(p1, p2)| {
        (max3(acc_x, p1.0, p2.0), max3(acc_y, p1.1, p2.1))
    });
    //dbg!(max_x, max_y);

    let mut point_score: HashMap<(isize, isize), isize> = HashMap::new();

    for i in 0..=max_x {
        dbg!(i);
        for j in 0..=max_y {
            for &line_seg in ls.iter() {
                if on_ls2(line_seg, (i, j)) {
                    let cur_score = point_score.entry((i, j)).or_insert(0);
                    *cur_score += 1;
                    //dbg!(i, j, line_seg, *cur_score);
                }
            }
        }
    }
    // //dbg!(&point_score);
    // for i in 0..=max_x {
    //     for j in 0..=max_y {
    //         match point_score.get(&(i, j)) {
    //             Some(v) => print!("{}", v),
    //             None => print!("."),
    //         }
    //     }
    //     print!("\n");
    // }

    point_score.iter().filter(|&(_k, v)| *v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_line_parse() {
        let s = "0,9 -> 5,9";
        assert_eq!(line_parser::ls(s), Ok(((0, 9), (5, 9))));
    }

    #[test]
    fn test_parse() {
        let s = TEST_INPUT;
        let rs = parse(s);
        println!("{:?}", rs);
    }

    #[test]
    fn test_on_ls() {
        // let ls = ((0, 9), (5, 9));
        // assert_eq!(on_ls(ls, (0, 9)), true);
        // assert_eq!(on_ls(ls, (1, 9)), true);
        // assert_eq!(on_ls(ls, (2, 9)), true);
        // assert_eq!(on_ls(ls, (5, 9)), true);
        // assert_eq!(on_ls(ls, (6, 9)), false);
        // assert_eq!(on_ls(ls, (0, 8)), false);

        // let ls = ((5, 5), (5, 9));
        // assert_eq!(on_ls(ls, (5, 8)), true);

        let ls = ((9, 4), (3, 4));
        assert_eq!(on_ls(ls, (3, 4)), true);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(TEST_INPUT)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(TEST_INPUT)), 12);
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let s = fs::read_to_string("/Users/rahulrav/rust/aoc2021/data/day5.txt")?;
    // println!("part1 {}", part1(parse(&s)));
    println!("part2 {}", part2(parse(&s)));
    Ok(())
}
