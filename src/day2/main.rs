use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn part2(r: &Vec<i32>) -> bool {
    for i in 0..r.len() {
        let mut t = r.clone();
        t.remove(i);
        if part1(&t) {
            return true;
        }
    }
    return false;
}

fn part1(r: &Vec<i32>) -> bool {
    let mut diffs = r.iter().zip(r.iter().skip(1)).map(|(a, b)| a - b);
    let first = diffs.next().unwrap();
    if first.abs() == 0 || first.abs() > 3 {
        return false;
    }
    for v in diffs {
        if v.signum() != first.signum() || v.abs() == 0 || v.abs() > 3 {
            return false;
        }
    }
    return true;
}

fn main() {
    let r = BufReader::new(File::open("src/day2/input.txt").unwrap());
    let mut ok1: usize = 0;
    let mut ok2: usize = 0;
    for line in r.lines() {
        let readings: Vec<i32> = line
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if part1(&readings) {
            ok1 += 1;
        }
        if part2(&readings) {
            ok2 += 1;
        }
    }
    println!("part1: {}, part2: {}", ok1, ok2);
}
