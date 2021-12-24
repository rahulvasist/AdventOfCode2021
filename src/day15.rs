aoc_main::main! {
    year 2021;
    day15 : generator => part_1, part_2;
}

mod day15 {
    use core::fmt;

    use itertools::Itertools;
    use pathfinding::dijkstra;

    type Input = Vec<Vec<usize>>;

    pub fn generator(input: &str) -> Input {
        input
            .lines()
            .map(|l| l.bytes().map(|b| (b - b'0') as usize).collect())
            .collect_vec()
    }

    fn print_2dvec<T: fmt::Display>(v: &Vec<Vec<T>>) {
        for row in v {
            for val in row {
                print!("{} ", val);
            }
            println!();
        }
    }

    pub fn part_1(input: &Input) -> usize {
        let path = dijkstra(
            &(0, 0),
            |&(x, y)| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .filter_map(|(i, j)| {
                        let x_: isize = x as isize + i;
                        let y_: isize = y as isize + j;
                        if x_ >= 0
                            && x_ < input.len() as isize
                            && y_ >= 0
                            && y_ < input[0].len() as isize
                        {
                            let x_ = x_ as usize;
                            let y_ = y_ as usize;
                            Some(((x_, y_), input[x_][y_]))
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            },
            |&pos| pos == (input.len() - 1, input[0].len() - 1),
        );

        println!("{:?}", path);
        path.unwrap().1
    }

    pub fn part_2(input: &Input) -> usize {
        let input2 = (0..5 * input.len())
            .map(|row| {
                (0..5 * input[0].len())
                    .map(|col| {
                        let nx = row / input.len();
                        let ny = col / input[0].len();
                        let n = input[row % input.len()][col % input[0].len()] + nx + ny;
                        (n % 10) + n / 10
                    })
                    .collect::<Vec<usize>>()
            })
            .collect();
        part_1(&input2)
        // print_2dvec(&input);
        // print_2dvec(&input2);
        // 0
    }
}

#[cfg(test)]
mod tests {
    

    use super::day15::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    #[ignore]
    fn test_generator() {
        println!("{:?}", generator(TEST_INPUT));
    }

    #[test]
    fn test_part_1() {
        let input = generator(TEST_INPUT);
        assert_eq!(40, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = generator(TEST_INPUT);
        assert_eq!(315, part_2(&input));
    }

    #[test]
    fn test_part_3() {
        let s = "8";
        let input = generator(s);
        assert_eq!(315, part_2(&input));
    }
}
