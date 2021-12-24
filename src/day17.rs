aoc_main::main! {
    year 2021;
    day17 : generator => part_1, part_2;
}

mod day17 {
    use std::cmp;

    use itertools::Itertools;

    type Target = ((isize, isize), (isize, isize));
    pub fn generator(_input: &str) -> Target {
        ((137, 171), (-73, -98))
    }

    fn simulate(mut x_vel: isize, mut y_vel: isize, target: Target) -> Option<isize> {
        let mut x = 0;
        let mut y = 0;
        let mut max_y = 0;
        loop {
            x += x_vel;
            y += y_vel;
            max_y = cmp::max(max_y, y);
            if x >= target.0 .0 && x <= target.0 .1 && y <= target.1 .0 && y >= target.1 .1 {
                return Some(max_y);
            }
            if x_vel == 0 && x < target.0 .0 {
                return None;
            }
            if x > target.0 .1 {
                return None;
            }
            if y < target.1 .1 {
                return None;
            }
            if x_vel != 0 {
                x_vel -= 1;
            }
            y_vel -= 1;
        }
    }

    pub fn part_1(input: &Target) -> isize {
        (1..=(input.0 .1))
            .cartesian_product(1..=(input.1 .1 * -1))
            .filter_map(|(xv, yv)| simulate(xv, yv, *input))
            .max()
            .unwrap()
    }

    pub fn part_2(input: &Target) -> isize {
        (1..=(input.0 .1))
            .cartesian_product(input.1 .1..=(input.1 .1 * -1))
            .filter_map(|(xv, yv)| simulate(xv, yv, *input))
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::day17::*;

    #[test]
    fn test_1() {
        let input = ((20, 30), (-5, -10));
        assert_eq!(45, part_1(&input));
    }
}
