use regex::Regex;
use std::fs;

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();
    match re.find(input) {
        None => return part1(input),
        Some(m) => {
            let mut new_input = input[..m.start()].to_owned();
            new_input.push_str(&input[m.end()..]);
            return part2(&new_input);
        }
    }
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&input).fold(0, |acc, x| {
        let (_, [a, b]) = x.extract();
        acc + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    })
}

fn main() {
    let input = fs::read_to_string("src/day3/input.txt")
        .unwrap()
        .replace("\n", "");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
