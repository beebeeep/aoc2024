use core::time;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{Arc, RwLock},
    thread,
};

use rayon::prelude::*;

fn load_input(txt: &str) -> (Vec<String>, Vec<String>) {
    let mut para = txt.split("\n\n");
    let patterns = para
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_string)
        .collect();
    let designs = para
        .next()
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .collect();
    return (patterns, designs);
}

fn is_possible(patterns: &Vec<String>, design: &str) -> bool {
    if design.len() == 0 {
        return true;
    }
    for pattern in patterns {
        if design.starts_with(pattern) {
            if is_possible(patterns, &design[pattern.len()..]) {
                return true;
            }
        }
    }
    return false;
}

fn count_possible(
    patterns: &Vec<String>,
    design: &str,
    possibles: &mut Arc<RwLock<HashMap<String, usize>>>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }
    if let Some(count) = possibles.read().unwrap().get(design) {
        return *count;
    }
    let mut count = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            count += count_possible(patterns, &design[pattern.len()..], possibles);
        }
    }
    possibles.write().unwrap().insert(design.to_string(), count);
    return count;
}

fn main() {
    let input = include_str!("input.txt");
    let (patterns, designs) = load_input(input);
    let possible: Vec<String> = designs
        .into_par_iter()
        .filter(|x| is_possible(&patterns, x))
        .collect();
    println!("part1: {}", possible.len());

    let cache: Arc<RwLock<HashMap<String, usize>>> = Arc::new(RwLock::new(HashMap::new()));
    let total_possibles: usize = possible
        .into_par_iter()
        .map(|x| {
            let mut cache = Arc::clone(&cache);
            (x.clone(), count_possible(&patterns, &x, &mut cache))
        })
        .fold(|| 0usize, |acc, (_, count)| acc + count)
        .sum::<usize>();
    println!("part2: {total_possibles}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check1() {
        let input = include_str!("test_input.txt");
        let (patterns, designs) = load_input(input);
        let possible = designs
            .iter()
            .filter(|x| is_possible(&patterns, x))
            .inspect(|x| println!("{} is possible", x))
            .count();

        println!("part1: {possible}");
    }

    #[test]
    fn check2() {
        let input = include_str!("test_input.txt");
        let (patterns, designs) = load_input(input);
        let total_possibles: usize = designs
            .into_par_iter()
            .map(|x| (x.clone(), count_possible(&patterns, &x)))
            .inspect(|(design, count)| println!("for {design}: {count}"))
            .fold(|| 0usize, |acc, (_, count)| acc + count)
            .sum::<usize>();
        println!("part2: {total_possibles}");
    }
}
