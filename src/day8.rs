use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

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
    let f: u8 = inter.iter().cloned().next().unwrap();
    mappings.insert(f, b'f');
    let c: u8 = known_nums[7]
        .unwrap()
        .iter()
        .cloned()
        .filter(|&s| s != a && s != f)
        .last()
        .unwrap();
    mappings.insert(c, b'c');

    // removing 'acf', 3 == 'dg' and 4 == 'bd'. So we can figure out 'dbg'
    let three = entry
        .inputs
        .iter()
        .filter(|seg| seg.len() == 5)
        .filter(|seg| seg.intersection(known_nums[7].unwrap()).count() == 3)
        .next()
        .unwrap();
    known_nums[3] = Some(three);
    let d = three
        .intersection(known_nums[4].unwrap())
        .cloned()
        .filter(|s| *s != a && *s != c && *s != f)
        .next()
        .unwrap();
    mappings.insert(d, b'd');
    let g = three
        .iter()
        .cloned()
        .filter(|s| *s != a && *s != c && *s != f && *s != d)
        .next()
        .unwrap();
    mappings.insert(g, b'g');
    let b = known_nums[4]
        .unwrap()
        .iter()
        .cloned()
        .filter(|s| *s != c && *s != d && *s != f)
        .next()
        .unwrap();
    mappings.insert(b, b'b');

    // The remaining one is 'e'
    let e = known_nums[8]
        .unwrap()
        .iter()
        .cloned()
        .filter(|s| *s != a && *s != b && *s != c && *s != d && *s != f && *s != g)
        .next()
        .unwrap();
    mappings.insert(e, b'e');
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
