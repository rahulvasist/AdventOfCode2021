use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs, io::Error};

type Point = (isize, isize, isize);

fn main() -> Result<(), Error> {
    let s = fs::read_to_string("/Users/rahulrav/rust/aoc2021/data/day19.txt")?;
    let input: Vec<Vec<Point>> = s.split("\n\n").map(parse_scanner).collect();
    solve(input);
    Ok(())
}

fn solve(mut input: Vec<Vec<Point>>) {
    let mut scanner_infos = Vec::new();
    let mut found_set = HashSet::new();
    found_set.insert(0);
    while found_set.len() != input.len() {
        for scanner in 0..input.len() {
            if found_set.contains(&scanner) {
                continue;
            }
            let found_list = found_set.iter().copied().collect_vec();
            for i in found_list {
                if let Some(info) = scanner_pos(&input[i], &input[scanner]) {
                    println!("Scanner{} and Scanner{} intersect", i, scanner);
                    modify_beacons(&mut input[scanner], &info);
                    found_set.insert(scanner);
                    scanner_infos.push(info);
                }
            }
        }
    }

    let mut beacon_set: HashSet<Point> = HashSet::new();
    for scanner in input {
        for b in scanner {
            beacon_set.insert(b);
        }
    }

    println!("Total num beacons={}", beacon_set.len());

    let m = (0..scanner_infos.len())
        .cartesian_product(0..scanner_infos.len())
        .filter(|(i, j)| i != j)
        .map(|(i, j)| distance(scanner_infos[i].position, scanner_infos[j].position))
        .map(|(x, y, z)| x + y + z)
        .reduce(isize::max)
        .unwrap();
    println!("Max manhattan distance = {}", m);
}

fn distance(p1: Point, p2: Point) -> (isize, isize, isize) {
    (
        (p1.0 - p2.0).abs(),
        (p1.1 - p2.1).abs(),
        (p1.2 - p2.2).abs(),
    )
}

#[derive(Debug)]
struct ScannerInfo {
    position: Point,
    axis_order: (usize, usize, usize),
    axis_direction: (isize, isize, isize),
}

fn modify_beacons(beacons: &mut Vec<Point>, info: &ScannerInfo) {
    for beacon in beacons.iter_mut() {
        let reordered = reorder_axis(*beacon, info.axis_order);
        let (x, y, z) = (
            (info.position.0 + info.axis_direction.0 * reordered.0),
            (info.position.1 + info.axis_direction.1 * reordered.1),
            (info.position.2 + info.axis_direction.2 * reordered.2),
        );
        *beacon = (x, y, z);
    }
}

fn reorder_axis(p: Point, order: (usize, usize, usize)) -> Point {
    let tmp = [p.0, p.1, p.2];
    (tmp[order.0], tmp[order.1], tmp[order.2])
}

fn scanner_pos(s1: &Vec<Point>, s2: &Vec<Point>) -> Option<ScannerInfo> {
    let mut p_to_dist: HashMap<(usize, usize), (isize, isize, isize)> = HashMap::new();
    let mut dist_to_p: HashMap<(isize, isize, isize), (usize, usize)> = HashMap::new();
    for i in 0..s1.len() {
        for j in i + 1..s1.len() {
            p_to_dist.insert((i, j), distance(s1[i], s1[j]));
            p_to_dist.insert((j, i), distance(s1[i], s1[j]));
            dist_to_p.insert(distance(s1[i], s1[j]), (i, j));
        }
    }

    assert_eq!(p_to_dist.len(), dist_to_p.len() * 2);

    let order = vec![
        (0, 1, 2),
        (0, 2, 1),
        (1, 0, 2),
        (1, 2, 0),
        (2, 0, 1),
        (2, 1, 0),
    ];
    let mut max_common = 0;
    let mut axis_order = (0, 0, 0);
    let mut s2_common_pts = HashSet::new();

    for o in order {
        let mut num_common = 0;
        let mut common_pts = HashSet::new();
        for i in 0..s2.len() {
            for j in i + 1..s2.len() {
                if dist_to_p.contains_key(&distance(reorder_axis(s2[i], o), reorder_axis(s2[j], o)))
                {
                    num_common += 1;
                    common_pts.insert(s2[i]);
                    common_pts.insert(s2[j]);
                }
            }
        }

        if num_common > max_common {
            axis_order = o;
            max_common = num_common;
            s2_common_pts = common_pts;
        }
    }

    if max_common != 66 {
        return None;
    }

    let mut iter = s2_common_pts.iter();
    let p2_1 = reorder_axis(*iter.next().unwrap(), axis_order);
    let p2_2 = reorder_axis(*iter.next().unwrap(), axis_order);
    let p2_3 = reorder_axis(*iter.next().unwrap(), axis_order);
    let d1 = distance(p2_1, p2_2);
    let d2 = distance(p2_1, p2_3);

    let (p1_1, p1_2) = dist_to_p.get(&d1).unwrap();
    let (p1_3, p1_4) = dist_to_p.get(&d2).unwrap();
    let actual_p1_1;
    let actual_p1_2;
    if *p1_1 == *p1_3 || *p1_1 == *p1_4 {
        actual_p1_1 = s1[*p1_1];
        actual_p1_2 = s1[*p1_2];
    } else if *p1_2 == *p1_3 || *p1_2 == *p1_4 {
        actual_p1_1 = s1[*p1_2];
        actual_p1_2 = s1[*p1_1];
    } else {
        panic!();
    }

    let x_dir = if p2_1.0.cmp(&p2_2.0) == actual_p1_1.0.cmp(&actual_p1_2.0) {
        1
    } else {
        -1
    };
    let y_dir = if p2_1.1.cmp(&p2_2.1) == actual_p1_1.1.cmp(&actual_p1_2.1) {
        1
    } else {
        -1
    };
    let z_dir = if p2_1.2.cmp(&p2_2.2) == actual_p1_1.2.cmp(&actual_p1_2.2) {
        1
    } else {
        -1
    };
    let scanner_position = (
        actual_p1_1.0 - (x_dir * p2_1.0),
        actual_p1_1.1 - (y_dir * p2_1.1),
        actual_p1_1.2 - (z_dir * p2_1.2),
    );
    let info = ScannerInfo {
        position: scanner_position,
        axis_order,
        axis_direction: (x_dir, y_dir, z_dir),
    };
    Some(info)
}

fn parse_scanner(s: &str) -> Vec<Point> {
    s.lines()
        .skip(1)
        .filter_map(|l| l.split(',').filter_map(|n| n.parse().ok()).collect_tuple())
        .collect_vec()
}
