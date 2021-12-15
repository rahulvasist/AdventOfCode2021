aoc_main::main! {
    year 2021;
    day14 : generator => part_1, part_2;
}

mod day14 {
    use std::{cmp, collections::HashMap};

    use itertools::Itertools;

    #[derive(Debug)]
    pub struct Input {
        template: Vec<u8>,
        rules: HashMap<(u8, u8), u8>,
    }

    pub fn generator(input: &str) -> Input {
        let mut split = input.split("\n\n");
        let template = split.next().unwrap().bytes().collect_vec();
        let rules = split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let tmp = l.split(" -> ").collect_vec();
                // dbg!(tmp);
                (
                    (
                        (tmp[0].as_bytes()[0]).clone(),
                        (tmp[0].as_bytes()[1]).clone(),
                    ),
                    (tmp[1].bytes().next().unwrap()).clone(),
                )
            })
            .collect();
        Input { template, rules }
    }

    #[allow(dead_code)]
    fn polymerize(chain: Vec<u8>, rules: &HashMap<(u8, u8), u8>) -> Vec<u8> {
        let mut new_chain = Vec::with_capacity(chain.len() * 2);
        for (&i, &j) in chain.iter().tuple_windows() {
            new_chain.push(i);
            new_chain.push(*rules.get(&(i, j)).unwrap());
        }
        new_chain.push(*chain.last().unwrap());
        new_chain
    }

    #[allow(dead_code)]
    fn brute_force(input: &Input, num_steps: usize) -> usize {
        let mut chain = input.template.clone();
        for _ in 0..num_steps {
            chain = polymerize(chain, &input.rules);
        }
        let mut counts = HashMap::new();
        for c in chain {
            let entry = counts.entry(c).or_insert(0);
            *entry += 1;
        }
        let max_val = counts.iter().fold((b'0', 0), |(acc_k, acc_v), (k, v)| {
            if v > &acc_v {
                (*k, *v)
            } else {
                (acc_k, acc_v)
            }
        });
        let min_val = counts
            .iter()
            .fold((b'0', usize::MAX), |(acc_k, acc_v), (k, v)| {
                if v < &acc_v {
                    (*k, *v)
                } else {
                    (acc_k, acc_v)
                }
            });
        return max_val.1 - min_val.1;
    }

    fn merge_map(
        mut left_map: HashMap<u8, usize>,
        mut right_map: HashMap<u8, usize>,
    ) -> HashMap<u8, usize> {
        for (k, v) in left_map.drain() {
            let entry = right_map.entry(k).or_insert(0);
            *entry += v;
        }
        right_map
    }

    #[allow(dead_code)]
    fn print_map(map: &HashMap<u8, usize>) {
        print!("{{");
        for (k, v) in map {
            print!("  {}:{} ", *k as char, v);
        }
        print!("}}");
    }

    fn recurse(
        pair: (u8, u8),
        rules: &HashMap<(u8, u8), u8>,
        current_step: usize,
        target_step: usize,
        cache: &mut HashMap<(u8, u8, usize), HashMap<u8, usize>>,
    ) -> HashMap<u8, usize> {
        if let Some(v) = cache.get(&(pair.0, pair.1, current_step)) {
            return v.clone();
        }
        if current_step == target_step {
            let mut rv = HashMap::new();
            rv.insert(pair.0, 1);
            let entry = rv.entry(pair.1).or_insert(0);
            *entry += 1;
            return rv;
        }
        let &val = rules.get(&pair).unwrap();
        let left_map = recurse((pair.0, val), rules, current_step + 1, target_step, cache);
        let right_map = recurse((val, pair.1), rules, current_step + 1, target_step, cache);
        let rv = merge_map(left_map, right_map);
        cache.insert((pair.0, pair.1, current_step), rv.clone());
        return rv;
    }

    fn using_dfs(input: &Input, target_step: usize) -> usize {
        let mut rv_map = HashMap::new();
        let mut cache = HashMap::new();
        for (&i, &j) in input.template.iter().tuple_windows() {
            rv_map = merge_map(
                rv_map,
                recurse((i, j), &input.rules, 0, target_step, &mut cache),
            );
        }
        let mut min_val = usize::MAX;
        let mut max_val = 0;
        for (k, v) in rv_map {
            let mut tmp = v / 2;
            if k == input.template[0] {
                tmp += 1;
            }
            if k == input.template[input.template.len() - 1] {
                tmp += 1;
            }
            min_val = cmp::min(min_val, tmp);
            max_val = cmp::max(max_val, tmp);
        }
        max_val - min_val
    }

    pub fn part_1(input: &Input) -> usize {
        using_dfs(input, 10)
    }

    pub fn part_2(input: &Input) -> usize {
        using_dfs(input, 40)
    }
}

#[cfg(test)]
mod tests {
    use super::day14::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    #[ignore]
    fn test_generator() {
        println!("{:?}", generator(&TEST_INPUT));
    }

    #[test]
    fn test_part_1() {
        let input = generator(&TEST_INPUT);
        assert_eq!(1588, part_1(&input));
    }
}
