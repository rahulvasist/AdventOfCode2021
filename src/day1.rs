use itertools::Itertools;
use std::fs;
use std::io::Error;

fn part1(v: &Vec<i64>) -> i64 {
    let ans: i64 = v
        .iter()
        .tuple_windows::<(_, _)>()
        .fold(0, |acc, (x, y)| if y > x { acc + 1 } else { acc });
    ans
}

fn part2(v: &Vec<i64>) -> i64 {
    let ans: i64 = v
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows::<(_, _)>()
        .fold(0, |acc, (x, y)| if y > x { acc + 1 } else { acc });
    ans
}

fn main() -> Result<(), Error> {
    // let v: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let s = fs::read_to_string("../data/day1.txt")?;
    let v: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();

    println!("{:?}", part1(&v));
    println!("{:?}", part2(&v));
    Ok(())
}
