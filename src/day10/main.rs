use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Point {
    r: i32,
    c: i32,
    h: i32,
}

fn get_paths(map: &Vec<Vec<i32>>, start: &Point) -> Vec<Vec<Point>> {
    let size = map.len() as i32;
    let mut paths: Vec<Vec<Point>> = Vec::new();
    if start.h == 9 {
        return vec![vec![start.clone()]];
    }
    for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let p = (start.r + dir.0, start.c + dir.1);
        if p.0 < 0 || p.0 >= size || p.1 < 0 || p.1 >= size {
            continue;
        }
        if map[p.0 as usize][p.1 as usize] - start.h != 1 {
            continue;
        }
        let next = Point {
            r: p.0,
            c: p.1,
            h: start.h + 1,
        };
        for path in &mut get_paths(map, &next) {
            let mut p = vec![start.clone()];
            p.append(path);
            paths.push(p);
        }
    }
    return paths;
}

fn load_input(file: &str) -> (Vec<Vec<i32>>, Vec<Point>) {
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut entrances: Vec<Point> = Vec::new();
    let r = BufReader::new(File::open(file).unwrap());
    let mut row: i32 = 0;
    for line in r.lines() {
        let line = line.unwrap();
        let columns: Vec<_> = line.chars().map(|c| c as i32 - '0' as i32).collect();
        for (col, h) in columns.iter().enumerate() {
            if *h == 0 {
                entrances.push(Point {
                    r: row,
                    c: col as i32,
                    h: 0,
                });
            }
        }
        map.push(columns);

        row += 1;
    }
    return (map, entrances);
}

fn part1(map: &Vec<Vec<i32>>, entrances: &Vec<Point>) -> usize {
    let mut score: usize = 0;
    for entrance in entrances {
        let mut peaks: HashSet<Point> = HashSet::new();
        get_paths(map, entrance).iter().for_each(|x| {
            peaks.insert(x.last().unwrap().clone());
        });
        score += peaks.len()
    }
    return score;
}

fn part2(map: &Vec<Vec<i32>>, entrances: &Vec<Point>) -> usize {
    entrances
        .iter()
        .fold(0, |acc, x| acc + get_paths(&map, x).len())
}

fn main() {
    let (map, entrances) = load_input("src/day10/input.txt");
    println!("part1: {}", part1(&map, &entrances));
    println!("part2: {}", part2(&map, &entrances));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check1() {
        let (map, entrances) = load_input("src/day10/test_input.txt");
        println!("{:?}\n{:?}", map, entrances);
        let paths = get_paths(&map, &entrances[0]);
        for path in paths {
            for r in 0..map.len() as i32 {
                for c in 0..map.len() as i32 {
                    if let Some(p) = path.iter().find(|x| x.c == c && x.r == r) {
                        print!("{}", p.h);
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }

            for p in path {
                print!("{} {} ({}) -> ", p.r, p.c, p.h);
            }
            println!("");
        }
        assert_eq!(36, part1(&map, &entrances));
    }
    #[test]
    fn check2() {
        let (map, entrances) = load_input("src/day10/test_input.txt");
        assert_eq!(81, part2(&map, &entrances))
    }
}
