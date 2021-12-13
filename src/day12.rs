use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string(r#"/Users/rahulrav/rust/aoc2021/data/day12.txt"#)?;
    println!("part1 {}", part1(&CaveSystem::new(&s)));
    println!("part2 {}", part2(&CaveSystem::new(&s)));
    Ok(())
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cave(String);

impl Cave {
    fn is_small_cave(&self) -> bool {
        self.0.chars().next().unwrap().is_lowercase()
    }
}

#[derive(Debug)]
struct CaveSystem {
    caves: HashSet<Cave>,
    connections: HashMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
    fn new(s: &str) -> Self {
        let mut caves: HashSet<Cave> = HashSet::new();
        let mut connections: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for l in s.lines() {
            let mut tokens = l.split('-');
            let c0 = String::from(tokens.next().unwrap());
            let c1 = String::from(tokens.next().unwrap());
            caves.insert(Cave(c0.clone()));
            caves.insert(Cave(c1.clone()));
            let entry = connections.entry(Cave(c0.clone())).or_insert(Vec::new());
            entry.push(Cave(c1.clone()));
            let entry = connections.entry(Cave(c1.clone())).or_insert(Vec::new());
            entry.push(Cave(c0.clone()));
        }
        Self { caves, connections }
    }
}

fn paths2<'a>(
    cave: &'a Cave,
    cs: &'a CaveSystem,
    visited: &mut HashMap<&'a Cave, usize>,
    path: &mut Vec<&'a Cave>,
    s_cave_used_twice: bool,
) -> usize {
    // println!("{} {:?} {}", cave.0, visited, s_cave_used_twice);
    if cave.0 == "end" {
        // for c in path {
        //     print!("{}-", c.0);
        // }
        // println!("end");
        return 1;
    }
    path.push(cave);
    if cave.is_small_cave() {
        let entry = visited.entry(cave).or_insert(0);
        *entry += 1;
    }
    let mut rv = 0;
    for con in cs.connections.get(cave).unwrap() {
        if con.0 == "start" {
            continue;
        }
        if visited.contains_key(con) {
            if s_cave_used_twice {
                continue;
            } else {
                // Let's use this small cave twice
                rv += paths2(con, cs, visited, path, true);
            }
        } else {
            rv += paths2(con, cs, visited, path, s_cave_used_twice);
        }
    }
    path.pop();
    if cave.is_small_cave() {
        let ref_count = visited.get_mut(cave).unwrap();
        *ref_count -= 1;
        if *ref_count == 0 {
            visited.remove(cave);
        }
    }
    rv
}

fn paths<'a>(
    cave: &'a Cave,
    cs: &'a CaveSystem,
    visited: &mut HashSet<&'a Cave>,
    path: &mut Vec<&'a Cave>,
) -> usize {
    if cave.0 == "end" {
        // for c in path {
        //     print!("{}-", c.0);
        // }
        // println!("end");
        return 1;
    }
    path.push(cave);
    if cave.is_small_cave() {
        visited.insert(cave);
    }
    let mut rv = 0;
    for con in cs.connections.get(cave).unwrap() {
        if con.is_small_cave() && visited.contains(con) {
            continue;
        }
        rv += paths(con, cs, visited, path);
    }
    path.pop();
    visited.remove(cave);
    rv
}

fn part1(cs: &CaveSystem) -> usize {
    let start = cs.caves.get(&Cave(String::from("start"))).unwrap();
    paths(start, cs, &mut HashSet::new(), &mut Vec::new())
}

fn part2(cs: &CaveSystem) -> usize {
    let start = cs.caves.get(&Cave(String::from("start"))).unwrap();
    paths2(start, cs, &mut HashMap::new(), &mut Vec::new(), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    #[ignore]
    fn test_parse() {
        dbg!(CaveSystem::new(&TEST_INPUT));
    }

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(&CaveSystem::new(&TEST_INPUT)), 10);
    }

    #[test]
    #[ignore]
    fn test_part1_1() {
        let s = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(part1(&CaveSystem::new(&s)), 19);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&CaveSystem::new(&TEST_INPUT)), 36);
    }

    #[test]
    fn test_part2_2() {
        let s = "start-A
A-b
b-c
c-end";
        assert_eq!(part2(&CaveSystem::new(&s)), 2);
    }
}
