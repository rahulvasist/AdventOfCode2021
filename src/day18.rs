aoc_main::main! {
    year 2021;
    day18: generator   => part_1, part_2;
}

mod day18 {
    use core::panic;
    use std::cmp;

    #[derive(Debug, Copy, Clone)]
    pub enum Symbol {
        LBrack,
        RBrack,
        Comma,
        Number(u32),
    }

    pub fn generator(s: &str) -> Vec<Vec<Symbol>> {
        s.lines()
            .map(|l| {
                l.bytes()
                    .map(|b| match b {
                        b'[' => Symbol::LBrack,
                        b']' => Symbol::RBrack,
                        b',' => Symbol::Comma,
                        _ => Symbol::Number((b - b'0') as u32),
                    })
                    .collect()
            })
            .collect()
    }

    fn matching_bracket_idx(s: &[Symbol]) -> usize {
        assert!(matches!(s[0], Symbol::LBrack));
        let mut num_brack = None;
        let mut i = 0;
        while num_brack != Some(0) {
            num_brack = match s[i] {
                Symbol::LBrack => num_brack.map_or(Some(1), |x| Some(x + 1)),
                Symbol::RBrack => Some(num_brack.unwrap() - 1),
                _ => num_brack,
            };
            i += 1;
        }
        i
    }

    pub fn magnitude(s: &[Symbol]) -> u32 {
        if s.len() == 1 {
            if let Symbol::Number(n) = s[0] {
                return n;
            } else {
                panic!();
            }
        }

        let left_node;
        let idx;
        if let Symbol::LBrack = s[1] {
            idx = matching_bracket_idx(&s[1..]) + 1;
            left_node = magnitude(&s[1..idx]);
        } else {
            left_node = magnitude(&s[1..2]);
            idx = 2;
        }

        assert!(matches!(s[idx], Symbol::Comma));
        let s = &s[idx + 1..s.len() - 1];
        let right_node = magnitude(s);
        3 * left_node + 2 * right_node
    }

    pub fn split(s: Vec<Symbol>) -> (bool, Vec<Symbol>) {
        let mut rv = Vec::new();
        let mut idx = 0;
        let mut changed = false;
        while idx < s.len() {
            match s[idx] {
                Symbol::LBrack | Symbol::RBrack | Symbol::Comma => rv.push(s[idx]),
                Symbol::Number(n) => {
                    if n <= 9 || changed {
                        rv.push(s[idx])
                    } else {
                        rv.push(Symbol::LBrack);
                        rv.push(Symbol::Number(n / 2));
                        rv.push(Symbol::Comma);
                        rv.push(Symbol::Number((n + 1) / 2));
                        rv.push(Symbol::RBrack);
                        changed = true;
                    }
                }
            }
            idx += 1;
        }
        (changed, rv)
    }

    pub fn explode(s: Vec<Symbol>) -> (bool, Vec<Symbol>) {
        assert!(matches!(s[0], Symbol::LBrack));
        let mut num_brack = 1;
        let mut i = 1;
        while num_brack != 5 {
            match s[i] {
                Symbol::LBrack => num_brack += 1,
                Symbol::RBrack => num_brack -= 1,
                _ => (),
            }
            i += 1;
            if i == s.len() {
                return (false, s);
            }
        }
        let left_num = if let Symbol::Number(n) = s[i] {
            n
        } else {
            panic!()
        };
        let right_num = if let Symbol::Number(n) = s[i + 2] {
            n
        } else {
            panic!()
        };
        let li = (0..=i - 1).rev().find_map(|k| {
            if let Symbol::Number(_) = s[k] {
                Some(k)
            } else {
                None
            }
        });
        let ri = (i + 3..s.len()).find_map(|k| {
            if let Symbol::Number(_) = s[k] {
                Some(k)
            } else {
                None
            }
        });

        let mut rv = Vec::new();
        let mut idx = 0;
        while idx < s.len() {
            if idx == i - 1 {
                idx += 4;
                rv.push(Symbol::Number(0));
            } else if li.is_some() && idx == li.unwrap() {
                let n = if let Symbol::Number(x) = s[idx] {
                    x
                } else {
                    panic!()
                };
                rv.push(Symbol::Number(n + left_num));
            } else if ri.is_some() && idx == ri.unwrap() {
                let n = if let Symbol::Number(x) = s[idx] {
                    x
                } else {
                    panic!()
                };
                rv.push(Symbol::Number(n + right_num));
            } else {
                rv.push(s[idx]);
            }
            idx += 1;
        }
        (true, rv)
    }

    pub fn reduce(s: Vec<Symbol>) -> Vec<Symbol> {
        let mut s = s;
        loop {
            let rv = explode(s);
            s = rv.1;
            if rv.0 {
                continue;
            }
            let rv = split(s);
            s = rv.1;
            if !rv.0 {
                return s;
            }
        }
    }

    pub fn add(s1: Vec<Symbol>, s2: Vec<Symbol>) -> Vec<Symbol> {
        let mut rv = Vec::new();
        rv.push(Symbol::LBrack);
        rv.extend_from_slice(s1.as_slice());
        rv.push(Symbol::Comma);
        rv.extend_from_slice(s2.as_slice());
        rv.push(Symbol::RBrack);
        reduce(rv)
    }

    pub fn part_1(s: &Vec<Vec<Symbol>>) -> usize {
        let s = s.clone();
        let mut iter = s.iter();
        let mut rv = iter.next().unwrap().clone();
        for l in iter {
            rv = add(rv, l.clone());
        }

        for s in &rv {
            match s {
                Symbol::LBrack => print!("["),
                Symbol::RBrack => print!("]"),
                Symbol::Comma => print!(","),
                Symbol::Number(i) => print!("{}", i),
            }
        }
        println!("");
        magnitude(&rv) as usize
    }

    pub fn part_2(s: &Vec<Vec<Symbol>>) -> usize {
        let mut m = 0;
        for i in 0..s.len() {
            for j in 0..s.len() {
                if i == j {
                    continue;
                }

                m = cmp::max(m, magnitude(&add(s[i].clone(), s[j].clone())));
                m = cmp::max(m, magnitude(&add(s[j].clone(), s[i].clone())));
            }
        }
        m as usize
    }
}
