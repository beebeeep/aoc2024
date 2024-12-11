use std::{fs, thread};

fn load_input(file: &str) -> Vec<i64> {
    fs::read_to_string(file)
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

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

fn main() {
    let stones = load_input("src/day11/input.txt");

    println!("part1: {}", multicount(stones.clone(), 20));
    println!("part2: {}", multicount(stones.clone(), 75));
}
