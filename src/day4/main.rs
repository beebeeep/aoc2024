use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const IN_SIZE: i32 = 140;

fn part1(input: &Vec<Vec<char>>) -> usize {
    let mut matches: usize = 0;
    for row in 0..IN_SIZE {
        for col in 0..IN_SIZE {
            for cd in -1..2 {
                for rd in -1..2 {
                    if row + rd * 3 < 0
                        || row + rd * 3 >= IN_SIZE
                        || col + cd * 3 < 0
                        || col + cd * 3 >= IN_SIZE
                    {
                        continue;
                    }
                    let w: String = (0..4)
                        .map(|i| {
                            input[usize::try_from(row + i * rd).unwrap()]
                                [usize::try_from(col + i * cd).unwrap()]
                        })
                        .collect();
                    if w == "XMAS" {
                        matches += 1;
                    }
                }
            }
        }
    }
    return matches;
}

fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut matches: usize = 0;
    let re = Regex::new(r"(M.S.A.M.S)|(S.S.A.M.M)|(M.M.A.S.S)|(S.M.A.S.M)").unwrap();
    for row in 1..IN_SIZE - 1 {
        for col in 1..IN_SIZE - 1 {
            let mut w = String::new();
            for rd in -1..2 {
                for cd in -1..2 {
                    w.push(
                        input[usize::try_from(row + rd).unwrap()]
                            [usize::try_from(col + cd).unwrap()],
                    );
                }
            }
            if re.is_match(&w) {
                matches += 1;
            }
        }
    }
    return matches;
}

fn main() {
    let r = BufReader::new(File::open("src/day4/input.txt").unwrap());
    let mut input =
        vec![vec![' '; usize::try_from(IN_SIZE).unwrap()]; usize::try_from(IN_SIZE).unwrap()];
    let mut lineno: usize = 0;
    for line in r.lines() {
        input[lineno] = line.unwrap().chars().collect();
        lineno += 1;
    }
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
