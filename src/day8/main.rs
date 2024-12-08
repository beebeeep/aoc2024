use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type Pos = (i32, i32);

fn is_ok(v: Pos) -> bool {
    return v.0 >= 0 && v.1 >= 0 && v.0 < 50 && v.1 < 50;
}

fn part2(anntenae: &HashMap<char, Vec<Pos>>) -> usize {
    let mut antinodes: HashSet<Pos> = HashSet::new();
    for (_, annt) in anntenae.iter() {
        for i in 0..annt.len() {
            antinodes.insert(annt[i]);
            for j in i + 1..annt.len() {
                let r = (annt[i].0 - annt[j].0, annt[i].1 - annt[j].1);
                for hop in 1.. {
                    let node1 = (annt[i].0 + hop * r.0, annt[i].1 + hop * r.1);
                    let node2 = (annt[j].0 - hop * r.0, annt[j].1 - hop * r.1);
                    if is_ok(node1) {
                        antinodes.insert(node1);
                    }
                    if is_ok(node2) {
                        antinodes.insert(node2);
                    }
                    if !is_ok(node1) && !is_ok(node2) {
                        break;
                    }
                }
            }
        }
    }
    return antinodes.len();
}

fn part1(anntenae: &HashMap<char, Vec<Pos>>) -> usize {
    let mut antinodes: HashSet<Pos> = HashSet::new();
    for (_, annt) in anntenae.iter() {
        for i in 0..annt.len() {
            for j in i + 1..annt.len() {
                let r = (annt[i].0 - annt[j].0, annt[i].1 - annt[j].1);
                let node1 = (annt[i].0 + r.0, annt[i].1 + r.1);
                let node2 = (annt[j].0 - r.0, annt[j].1 - r.1);
                if is_ok(node1) {
                    antinodes.insert(node1);
                }
                if is_ok(node2) {
                    antinodes.insert(node2);
                }
            }
        }
    }
    return antinodes.len();
}

fn main() {
    let r = BufReader::new(File::open("src/day8/input.txt").unwrap());
    let mut anntenae: HashMap<char, Vec<Pos>> = HashMap::new();
    let mut row: i32 = 0;
    for line in r.lines() {
        let line = line.unwrap();
        for (col, c) in line.chars().enumerate() {
            let col = col as i32;
            if c != '.' {
                if !anntenae.contains_key(&c) {
                    anntenae.insert(c, vec![(row, col)]);
                } else {
                    anntenae.get_mut(&c).unwrap().push((row, col));
                }
            }
        }
        row += 1;
    }
    println!("part1: {}", part1(&anntenae));
    println!("part2: {}", part2(&anntenae));
}
