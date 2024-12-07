use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn check(test: &i64, lval: &i64, rvals: &[i64], ops: &[fn(i64, i64) -> i64]) -> bool {
    match rvals {
        [] => *test == *lval,
        [head, tail @ ..] => {
            for op in ops {
                if check(test, &(*op)(*lval, *head), tail, ops) {
                    return true;
                }
            }
            return false;
        }
    }
}

fn part1(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    let ops = [i64::wrapping_add, i64::wrapping_mul];
    input
        .iter()
        .filter(|(result, operands)| check(result, &operands[0], &operands[1..], &ops))
        .fold(0, |acc, (result, _)| acc + result)
}

fn part2(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    let ops = [i64::wrapping_add, i64::wrapping_mul, concat];
    input
        .iter()
        .filter(|(result, operands)| check(result, &operands[0], &operands[1..], &ops))
        .fold(0, |acc, (result, _)| acc + result)
}

fn main() {
    let r = BufReader::new(File::open("src/day7/input.txt").unwrap());
    let mut input: Vec<(i64, Vec<i64>)> = Vec::new();
    for line in r.lines() {
        let line = line.unwrap();
        if let [result, operands] = line.split(": ").collect::<Vec<&str>>()[..] {
            input.push((
                result.parse().unwrap(),
                operands.split(' ').map(|x| x.parse().unwrap()).collect(),
            ));
        }
    }
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
