use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const IN_SIZE: i32 = 130;
const DIRS: &'static [(i32, i32, char)] = &[(-1, 0, '^'), (0, 1, '>'), (1, 0, 'v'), (0, -1, '<')];

fn display(input: &Vec<Vec<char>>) {
    for row in input {
        println!("{}", row.iter().collect::<String>())
    }
}

fn analyze(input: &Vec<Vec<char>>, mut pos: (usize, usize)) -> Option<Vec<Vec<char>>> {
    let mut dir = 0;
    let mut input = input.clone();
    let mut steps = 0;
    loop {
        input[pos.0][pos.1] = DIRS[dir].2;
        let next_pos = (pos.0 as i32 + DIRS[dir].0, pos.1 as i32 + DIRS[dir].1);
        if next_pos.0 < 0 || next_pos.0 >= IN_SIZE || next_pos.1 < 0 || next_pos.1 >= IN_SIZE {
            break;
        }
        let next_pos: (usize, usize) = (next_pos.0 as usize, next_pos.1 as usize);
        let dv = DIRS[dir].2;
        match input[next_pos.0][next_pos.1] {
            '#' => dir = (dir + 1) % DIRS.len(),
            '^' if dv == '^' => return None,
            'v' if dv == 'v' => return None,
            '<' if dv == '<' => return None,
            '>' if dv == '>' => return None,
            _ => pos = next_pos,
        }
        steps += 1;
        if steps > 10000 {
            input[pos.0][pos.1] = 'X';
            display(&input);
            // panic!("i'm stuck, direction is {}", DIRS[dir].2);
            return None; // consider this loop lol
        }
    }
    // display(&input);
    return Some(input);
}

fn part1(input: &Vec<Vec<char>>, pos: (usize, usize)) -> i32 {
    if let Some(result) = analyze(input, pos) {
        display(&result);
        return result.iter().fold(0, |acc, col| {
            acc + col.iter().fold(0, |acc, x| {
                acc + if ['^', '<', '>', 'v'].contains(x) {
                    1
                } else {
                    0
                }
            })
        });
    }
    0
}

fn part2(input: &Vec<Vec<char>>, pos: (usize, usize)) -> i32 {
    let mut count = 0;
    for r in 0..IN_SIZE as usize {
        for c in 0..IN_SIZE as usize {
            if input[r][c] == '#' || (r, c) == pos {
                continue;
            }
            let mut new_input = input.clone();
            new_input[r][c] = '#';
            if None == analyze(&new_input, pos) {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let r = BufReader::new(File::open("src/day6/input.txt").unwrap());
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut lineno: usize = 0;
    let mut guard_pos: (usize, usize) = (0, 0);
    for line in r.lines() {
        let line = line.unwrap();
        input.push(line.chars().collect());
        if let Some(col) = line.find('^') {
            guard_pos.0 = lineno;
            guard_pos.1 = col;
        }
        lineno += 1;
    }
    println!("part1: {}", part1(&input, guard_pos));
    println!("part2: {}", part2(&input, guard_pos));
}
