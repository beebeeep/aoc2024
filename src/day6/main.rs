use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const IN_SIZE: i32 = 130;
const DIRS: &'static [(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone)]
struct Fld {
    obstacle: bool,
    passes: usize,
}

fn analyze(input: &Vec<Vec<Fld>>, mut pos: (usize, usize)) -> Option<Vec<Vec<Fld>>> {
    let mut dir = 0;
    let mut input = input.clone();
    loop {
        input[pos.0][pos.1].passes += 1;
        let next_pos = (pos.0 as i32 + DIRS[dir].0, pos.1 as i32 + DIRS[dir].1);
        if next_pos.0 < 0 || next_pos.0 >= IN_SIZE || next_pos.1 < 0 || next_pos.1 >= IN_SIZE {
            break;
        }
        let next_pos: (usize, usize) = (next_pos.0 as usize, next_pos.1 as usize);
        let next_fld = &mut input[next_pos.0][next_pos.1];
        if next_fld.obstacle {
            dir = (dir + 1) % DIRS.len();
        } else if next_fld.passes >= 4 {
            return None;
        } else {
            pos = next_pos;
        }
    }
    return Some(input);
}

fn part1(input: &Vec<Vec<Fld>>, pos: (usize, usize)) -> i32 {
    if let Some(result) = analyze(input, pos) {
        return result.iter().fold(0, |acc, col| {
            acc + col
                .iter()
                .fold(0, |acc, x| acc + if x.passes > 0 { 1 } else { 0 })
        });
    }
    0
}

fn part2(input: &Vec<Vec<Fld>>, pos: (usize, usize)) -> i32 {
    let mut count = 0;
    let mut input = input.clone();
    for r in 0..IN_SIZE as usize {
        for c in 0..IN_SIZE as usize {
            if input[r][c].obstacle || (r, c) == pos {
                continue;
            }
            input[r][c].obstacle = true;
            match analyze(&input, pos) {
                None => count += 1,
                _ => (),
            }
            input[r][c].obstacle = false;
        }
    }
    return count;
}

fn main() {
    let r = BufReader::new(File::open("src/day6/input.txt").unwrap());
    let mut input: Vec<Vec<Fld>> = Vec::new();
    let mut lineno: usize = 0;
    let mut guard_pos: (usize, usize) = (0, 0);
    for line in r.lines() {
        let line = line.unwrap();
        input.push(
            line.chars()
                .map(|x| Fld {
                    passes: 0,
                    obstacle: x == '#',
                })
                .collect(),
        );
        if let Some(col) = line.find('^') {
            guard_pos.0 = lineno;
            guard_pos.1 = col;
        }
        lineno += 1;
    }
    println!("part1: {}", part1(&input, guard_pos));
    println!("part2: {}", part2(&input, guard_pos));
}
