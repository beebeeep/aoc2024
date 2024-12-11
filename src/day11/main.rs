use std::{collections::HashMap, fs, thread};

fn load_input(file: &str) -> Vec<i64> {
    fs::read_to_string(file)
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

// this one is correct, but brute-forces it
fn multicount(mut stones: Vec<i64>, times: usize) -> usize {
    let mut handles: Vec<_> = Vec::new();
    blink(&mut stones);
    blink(&mut stones);
    for s in stones {
        let h = thread::spawn(move || count_stones(vec![s], times - 2, 0));
        handles.push(h);
    }
    let mut count = 0;
    for h in handles {
        count += h.join().unwrap();
    }
    return count;
}
fn count_stones(mut stones: Vec<i64>, times: usize, depth: usize) -> usize {
    println!("depth {depth} len {}, times {times}", stones.len());
    for i in 0..times {
        blink(&mut stones);
        let l = stones.len();
        if l > 5000000 {
            return count_stones(stones[..l / 2].into(), times - i - 1, depth + 1)
                + count_stones(stones[l / 2..].into(), times - i - 1, depth + 1);
        }
    }
    return stones.len();
}

fn blink(stones: &mut Vec<i64>) {
    let l = stones.len();
    for i in 0..l {
        if stones[i] == 0 {
            stones[i] = 1;
            continue;
        }
        let pow = stones[i].ilog10() + 1;
        if pow % 2 == 0 {
            let d = 10i64.pow(pow / 2);
            let v = stones[i];
            stones[i] = v / d;
            stones.push(v % d);
            continue;
        }
        stones[i] *= 2024;
    }
}

fn do_stone_things(stone: i64, tips: &mut HashMap<(i64, usize), usize>, times: usize) -> usize {
    if let Some(r) = tips.get(&(stone, times)) {
        return *r;
    }
    let mut s = stone;

    let mut dups = 0;
    for i in 0..times {
        if s == 0 {
            s = 1;
            continue;
        }
        let pow = s.ilog10() + 1;
        if pow % 2 == 0 {
            let d = 10i64.pow(pow / 2);
            dups += 1 + do_stone_things(s % d, tips, times - i - 1);
            s = s / d;
            continue;
        }
        s *= 2024;
    }
    tips.insert((stone, times), dups);
    return dups;
}

fn main() {
    let stones = load_input("src/day11/input.txt");
    let mut tips: HashMap<_, _> = HashMap::new();

    println!(
        "part1: {}",
        stones.iter().fold(stones.len(), |acc, x| acc
            + do_stone_things(*x, &mut tips, 25))
    );
    println!(
        "part2: {}",
        stones.iter().fold(stones.len(), |acc, x| acc
            + do_stone_things(*x, &mut tips, 75))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut tips: HashMap<_, _> = HashMap::new();
        let s = vec![125, 17]
            .iter()
            .fold(2, |acc, x| acc + do_stone_things(*x, &mut tips, 25));
        assert_eq!(55312, s);
    }
}
