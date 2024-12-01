use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    left.iter().zip(right.iter()).fold(0, |acc, x| acc + (x.0 - x.1).abs())
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut histogram: HashMap<i32, i32> = HashMap::new();
    right.iter().for_each(|x| {
        histogram.insert(*x, histogram.get(x).unwrap_or(&0) + 1);
    });
    left.iter().fold(0,|acc, x| acc + x*histogram.get(x).unwrap_or(&0))
}

fn main() {
    let r = BufReader::new(File::open("src/day1/input1.txt").unwrap());
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in r.lines() {
        match line.unwrap().split_ascii_whitespace().take(2).collect::<Vec<&str>>()[..]   {
            [a, b] => {
                left.push(a.parse().unwrap());
                right.push(b.parse().unwrap());
        }
        _ => panic!("invalid input")
        }
    }
    left.sort(); right.sort();
    print!("part1: {}\npart2: {}\n", part1(&left, &right), part2(&left, &right));
}
