use core::fmt;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn::std::error::Error>> {
    let s = fs::read_to_string(r#"/Users/rahulrav/rust/aoc2021/data/day8.txt"#)?;
    println!("part1 {}", part1(parse(&s)));
    println!("part2 {}", part2(parse(&s)));
    Ok(())
}

type Segment = HashSet<u8>;

#[derive(Debug)]
pub struct Entry {
    inputs: Vec<Segment>,
    outputs: Vec<Segment>,
}

peg::parser! {
    grammar input_parser() for str {
        rule segment() -> Segment
        = s:$(['a'..='g']+) {
            s.as_bytes().iter().cloned().collect()
        }

        rule segments() -> Vec<Segment>
            = seg:segment() ** " " { seg }

        pub rule line() -> Entry
            = inputs:segments() " | " outputs:segments() { Entry{inputs, outputs} }

        pub rule parser() -> Vec<Entry>
            = entry:line() ** "\n" { entry }
    }
}

fn parse(s: &str) -> Vec<Entry> {
    s.lines()
        .filter_map(|l| input_parser::line(l).ok())
        .collect()
}

fn part1(entries: Vec<Entry>) -> usize {
    entries
        .iter()
        .map(|entry| entry.outputs.iter())
        .flatten()
        .filter(|output| {
            output.len() == 2 || output.len() == 4 || output.len() == 3 || output.len() == 7
        })
        .count()
}

struct State {
    possible_mapping: HashMap<BTreeSet<u8>, HashSet<u8>>,
}

impl State {
    fn new(inputs: &Vec<Segment>) -> Self {
        let number_to_num_segs = [
            (0, 6, "abcefg"),
            (1, 2, "cf"),
            (2, 6, "acdeg"),
            (3, 6, "acdfg"),
            (4, 4, "bcdf"),
            (5, 6, "abdfg"),
            (6, 6, "abdefg"),
            (7, 3, "acf"),
            (8, 7, "abcdefg"),
            (9, 6, "abcdfg"),
        ];
        let mut possible_mapping = HashMap::new();
        for &(_num, num_seg, seg) in number_to_num_segs.iter() {
            let mapping: HashSet<u8> = inputs
                .iter()
                .filter(|s| s.len() == num_seg)
                .flatten()
                .cloned()
                .collect();
            let map_key = seg.bytes().collect();
            possible_mapping.insert(map_key, mapping);
        }
        Self { possible_mapping }
    }

    fn solved(&self) -> bool {
        self.possible_mapping
            .iter()
            .all(|(k, v)| k.len() == 1 && v.len() == 1)
    }

    fn remove_from_other_mappings(&mut self, key: u8, val: u8) -> bool {
        let keys: Vec<BTreeSet<u8>> = self.possible_mapping.keys().cloned().collect();
        let mut something_changed = false;
        for i in 0..keys.len() {
            let mut k = keys[i].clone();
            if !k.contains(&key) {
                continue;
            }
            if self.possible_mapping.get(&k).unwrap().len() == 1 {
                continue;
            }
            k.remove(&key);
            if let Some(mut v) = self.possible_mapping.remove(&keys[i]) {
                v.remove(&val);
                self.possible_mapping.insert(k, v);
                something_changed = true;
            }
        }
        something_changed
    }

    fn sanitize(&mut self) {
        // println!("Start saniitze: {}", self);
        let mut something_changed = true;
        while something_changed {
            something_changed = false;
            let mut remove_list: Vec<(u8, u8)> = Vec::new();
            for (k, v) in self.possible_mapping.iter() {
                if k.len() == 1 && v.len() == 1 {
                    let key = k.iter().last().unwrap();
                    let val = v.iter().last().unwrap();
                    remove_list.push((*key, *val));
                }
            }
            for (k, v) in remove_list.into_iter() {
                something_changed = something_changed || self.remove_from_other_mappings(k, v);
            }
            // if something_changed {
            //     println!("End saniitze: {}", self);
            //     assert!(false);
            // }
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State:\n");
        for (k, v) in self.possible_mapping.iter() {
            write!(
                f,
                "{}: {}\n",
                std::str::from_utf8(&k.iter().cloned().collect::<Vec<u8>>()).unwrap(),
                std::str::from_utf8(&v.iter().cloned().collect::<Vec<u8>>()).unwrap()
            );
        }
        Ok(())
    }
}

fn do_intersection(state: &mut State, i: usize, j: usize) -> bool {
    let mut something_changed = false;
    let keys: Vec<BTreeSet<u8>> = state.possible_mapping.keys().cloned().collect();
    let mapping1 = state.possible_mapping.get(&keys[i]).unwrap();
    let mapping2 = state.possible_mapping.get(&keys[j]).unwrap();
    let intersection: HashSet<u8> = { mapping1 }.intersection(mapping2).cloned().collect();
    let key_intersection: BTreeSet<u8> = keys[i].intersection(&keys[j]).cloned().collect();
    if key_intersection.len() == 0 {
        return false;
    }
    let existing_entry = state.possible_mapping.get(&key_intersection);
    let entry: HashSet<u8>;
    let mut should_exit = false;
    if (existing_entry.is_some() && existing_entry.unwrap().len() > intersection.len())
        || existing_entry.is_none()
    {
        entry = intersection;
        something_changed = true;
        // dbg!(i, j, &key_intersection, &entry);
        state.possible_mapping.insert(key_intersection, entry);
    }
    something_changed
}

fn do_difference(state: &mut State, i: usize, j: usize) -> bool {
    let mut something_changed = false;
    let keys: Vec<BTreeSet<u8>> = state.possible_mapping.keys().cloned().collect();
    let mapping1 = state.possible_mapping.get(&keys[i]).unwrap();
    let mapping2 = state.possible_mapping.get(&keys[j]).unwrap();
    // if &mapping1.len() != &keys[i].len() || &mapping2.len() != &keys[j].len() {
    //     return false;
    // }
    if !(keys[i].is_superset(&keys[j]) && &mapping2.len() == &keys[j].len()) {
        return false;
    }
    println!("i={} j={}", i, j);
    dbg!(i, j, &keys[i], &keys[j], &mapping1, &mapping2);
    let difference: HashSet<u8> = { mapping1 }.difference(mapping2).cloned().collect();
    let key_difference: BTreeSet<u8> = keys[i].difference(&keys[j]).cloned().collect();
    dbg!(&key_difference, &difference);
    if key_difference.len() == 0 || difference.len() == 0 {
        return false;
    }
    assert!(&key_difference.len() <= &difference.len());
    let existing_entry = state.possible_mapping.get(&key_difference);
    let entry: HashSet<u8>;
    if (existing_entry.is_some() && existing_entry.unwrap().len() > difference.len())
        || existing_entry.is_none()
    {
        entry = difference;
        something_changed = true;
        state.possible_mapping.insert(key_difference, entry);
    }
    something_changed
}

fn solve(entry: &Entry) {
    let mut state = State::new(&entry.inputs);
    println!("Initial state:\n{}", state);
    let mut something_changed = true;
    while something_changed {
        something_changed = false;
        let keys: Vec<BTreeSet<u8>> = state.possible_mapping.keys().cloned().collect();
        for i in 0..keys.len() {
            for j in 0..keys.len() {
                something_changed = do_intersection(&mut state, i, j);
                if something_changed {
                    println!("{}", state);
                    break;
                }

                something_changed = do_difference(&mut state, i, j);
                if something_changed {
                    println!("{}", state);
                    break;
                }
            }
            if something_changed {
                state.sanitize();
                break;
            }
        }
        if state.solved() {
            println!("Solved: {}", state);
            return;
        }
    }
    println!("Not solved: {}", state);
}

fn solve_input(entry: &Entry) -> HashMap<u8, u8> {
    let mut mappings: HashMap<u8, u8> = HashMap::new();
    let mut known_nums: [Option<&Segment>; 10] = [None; 10];
    for input in entry.inputs.iter() {
        match input.len() {
            2 => known_nums[1] = Some(input),
            3 => known_nums[7] = Some(input),
            4 => known_nums[4] = Some(input),
            7 => known_nums[8] = Some(input),
            _ => (),
        }
    }
    let a: u8 = known_nums[7]
        .unwrap()
        .difference(known_nums[1].unwrap())
        .cloned()
        .next()
        .unwrap();
    mappings.insert(a, b'a');

    // 6 and 7 have only 'a' and 'f' as common. So we can figure out 'c' and 'f'
    assert_eq!(
        entry
            .inputs
            .iter()
            .filter(|seg| seg.len() == 6)
            .filter(|seg| seg.intersection(known_nums[7].unwrap()).count() == 2)
            .count(),
        1
    );
    let six = entry
        .inputs
        .iter()
        .filter(|seg| seg.len() == 6)
        .filter(|seg| seg.intersection(known_nums[7].unwrap()).count() == 2)
        .next()
        .unwrap();
    known_nums[6] = Some(six);
    let mut inter: HashSet<u8> = six.intersection(known_nums[7].unwrap()).cloned().collect();
    inter.remove(&a);
    assert_eq!(inter.clone().len(), 1);
    let f: u8 = inter.iter().cloned().next().unwrap();
    mappings.insert(f, b'f');
    assert_eq!(
        known_nums[7]
            .unwrap()
            .iter()
            .cloned()
            .filter(|&s| s != a && s != f)
            .count(),
        1
    );
    let c: u8 = known_nums[7]
        .unwrap()
        .iter()
        .cloned()
        .filter(|&s| s != a && s != f)
        .last()
        .unwrap();
    mappings.insert(c, b'c');

    // removing 'acf', 3 == 'dg' and 4 == 'bd'. So we can figure out 'dbg'
    assert_eq!(
        entry
            .inputs
            .iter()
            .filter(|seg| seg.len() == 5)
            .filter(|seg| seg.intersection(known_nums[7].unwrap()).count() == 3)
            .count(),
        1
    );
    let three = entry
        .inputs
        .iter()
        .filter(|seg| seg.len() == 5)
        .filter(|seg| seg.intersection(known_nums[7].unwrap()).count() == 3)
        .next()
        .unwrap();
    known_nums[3] = Some(three);
    assert_eq!(
        three
            .intersection(known_nums[4].unwrap())
            .cloned()
            .filter(|s| *s != a && *s != c && *s != f)
            .count(),
        1
    );
    let d = three
        .intersection(known_nums[4].unwrap())
        .cloned()
        .filter(|s| *s != a && *s != c && *s != f)
        .next()
        .unwrap();
    mappings.insert(d, b'd');
    assert_eq!(
        three
            .iter()
            .cloned()
            .filter(|s| *s != a && *s != c && *s != f && *s != d)
            .count(),
        1
    );
    let g = three
        .iter()
        .cloned()
        .filter(|s| *s != a && *s != c && *s != f && *s != d)
        .next()
        .unwrap();
    mappings.insert(g, b'g');
    assert_eq!(
        known_nums[4]
            .unwrap()
            .iter()
            .cloned()
            .filter(|s| *s != c && *s != d && *s != f)
            .count(),
        1
    );
    let b = known_nums[4]
        .unwrap()
        .iter()
        .cloned()
        .filter(|s| *s != c && *s != d && *s != f)
        .next()
        .unwrap();
    mappings.insert(b, b'b');

    // The remaining one is 'e'
    assert_eq!(
        known_nums[8]
            .unwrap()
            .iter()
            .cloned()
            .filter(|s| !mappings.contains_key(s))
            .count(),
        1
    );
    let e = known_nums[8]
        .unwrap()
        .iter()
        .cloned()
        .filter(|s| *s != a && *s != b && *s != c && *s != d && *s != f && *s != g)
        .next()
        .unwrap();
    mappings.insert(e, b'e');
    assert_eq!(mappings.len(), 7);
    mappings
}

fn segment_to_digit(segment: &Segment, mappings: &HashMap<u8, u8>) -> usize {
    let segment_to_num_map: HashMap<BTreeSet<u8>, usize> = HashMap::from([
        ("abcefg".bytes().collect(), 0),
        ("cf".bytes().collect(), 1),
        ("acdeg".bytes().collect(), 2),
        ("acdfg".bytes().collect(), 3),
        ("bcdf".bytes().collect(), 4),
        ("abdfg".bytes().collect(), 5),
        ("abdefg".bytes().collect(), 6),
        ("acf".bytes().collect(), 7),
        ("abcdefg".bytes().collect(), 8),
        ("abcdfg".bytes().collect(), 9),
    ]);
    let key = segment
        .iter()
        .map(|c| mappings.get(c).unwrap().clone())
        .collect();
    let rv = segment_to_num_map.get(&key).unwrap().clone();
    rv
}

fn part2(entries: Vec<Entry>) -> usize {
    entries
        .iter()
        .map(|entry| {
            let mappings = solve_input(&entry);
            let num = entry.outputs.iter().fold(0, |acc, segment| {
                acc * 10 + segment_to_digit(segment, &mappings)
            });
            num
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_parse() {
        // println!("{:?}", parse(&TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(&TEST_INPUT)), 26);
    }

    #[test]
    fn test_solve_input() {
        let entries = parse(&TEST_INPUT);
        solve_input(&entries[0]);
    }

    #[test]
    fn test_part2() {
        let entries = parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n",
        );
        assert_eq!(part2(entries), 5353);

        let entries = parse(&TEST_INPUT);
        assert_eq!(part2(entries), 61229);
    }
}
