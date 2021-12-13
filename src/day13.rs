aoc_main::main! {
    year 2021;
    day13 : generator => part_1, part_2;
}

mod day13 {
    use std::collections::HashSet;

    use itertools::Itertools;

    #[derive(Debug)]
    enum Fold {
        X(usize),
        Y(usize),
    }

    #[derive(Debug)]
    pub struct Input {
        dots: Vec<(usize, usize)>,
        folds: Vec<Fold>,
    }

    pub fn generator(input: &str) -> Input {
        let mut split = input.split("\n\n");
        let dots = split
            .next()
            .unwrap()
            .lines()
            .filter_map(|l| l.split(',').map(|c| c.parse().unwrap()).collect_tuple())
            .collect_vec();
        let mut folds = Vec::new();
        let const_len = "fold along ".len();
        for l in split.next().unwrap().lines() {
            let s = &l[const_len..];
            let mut tokens = s.split('=');
            let axis = tokens.next().unwrap();
            let val = tokens.next().unwrap().parse().unwrap();
            let fold = match axis {
                "x" => Fold::X(val),
                "y" => Fold::Y(val),
                _ => unreachable!(),
            };
            folds.push(fold);
        }
        Input { dots, folds }
    }

    fn fold(fold: &Fold, dot_map: &mut HashSet<(usize, usize)>) {
        let dots = dot_map
            .iter()
            .filter(|(x, y)| match fold {
                Fold::X(val) => x > &val,
                Fold::Y(val) => y > &val,
            })
            .cloned()
            .collect_vec();
        let transform_fn = |(x, y)| match fold {
            Fold::X(val) => (x - 2 * (x - val), y),
            Fold::Y(val) => (x, y - 2 * (y - val)),
        };
        for (x, y) in dots.into_iter() {
            dot_map.remove(&(x, y));
            dot_map.insert(transform_fn((x, y)));
        }
    }

    pub fn part_1(input: &Input) -> usize {
        let mut dot_map = input.dots.iter().cloned().collect();
        fold(&input.folds[0], &mut dot_map);
        dot_map.len()
    }

    pub fn part_2(input: &Input) -> usize {
        let mut dot_map = input.dots.iter().cloned().collect();
        for f in &input.folds {
            fold(&f, &mut dot_map);
        }
        let max_col = dot_map.iter().map(|&(x, _)| x).max().unwrap();
        let max_row = dot_map.iter().map(|&(_, y)| y).max().unwrap();
        for r in 0..=max_row {
            for c in 0..=max_col {
                if dot_map.contains(&(c, r)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::day13::*;

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    #[ignore]
    fn test_generator() {
        println!("{:?}", generator(&TEST_INPUT));
    }

    #[test]
    fn test_part_1() {
        let input = generator(&TEST_INPUT);
        assert_eq!(17, part_1(&input));
    }
}
