use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Dir = (i32, i32);
const UP: Dir = (-1, 0);
const RIGHT: Dir = (0, 1);
const DOWN: Dir = (1, 0);
const LEFT: Dir = (0, -1);
const DIRS: &'static [Dir] = &[UP, RIGHT, DOWN, LEFT];

#[derive(Debug, Clone)]
struct Plot {
    region_id: i32,
    plant: char,
    r: i32,
    c: i32,
    fences: Vec<Dir>,
}

fn bound_sum((r, c): (usize, usize), (dr, dc): Dir, sz: usize) -> Option<(usize, usize)> {
    if (r as i32) + dr < 0
        || (c as i32) + dc < 0
        || (r as i32) + dr >= sz as i32
        || (c as i32) + dc >= sz as i32
    {
        return None;
    }
    return Some(((r as i32 + dr) as usize, (c as i32 + dc) as usize));
}

fn get_regions(map: &Vec<Vec<Plot>>) -> HashMap<i32, Vec<&Plot>> {
    let mut regions: HashMap<i32, Vec<&Plot>> = HashMap::new();
    for row in map {
        for plot in row {
            match regions.get_mut(&plot.region_id) {
                None => {
                    regions.insert(plot.region_id, vec![plot]);
                }
                Some(plots) => {
                    plots.push(plot);
                }
            };
        }
    }
    return regions;
}

fn find_fence(p: &Plot, region: &Vec<&Plot>, d: &Dir, fd: &Dir) -> bool {
    region
        .iter()
        .find(|x| x.r == p.r + d.0 && x.c == p.c + d.1 && x.fences.contains(fd))
        .is_some()
}

fn count_sides(region: &Vec<&Plot>) -> usize {
    let mut angles = 0;
    for p in region {
        if p.fences.contains(&UP) && p.fences.contains(&RIGHT) {
            angles += 1;
        }
        if p.fences.contains(&UP) && p.fences.contains(&LEFT) {
            angles += 1;
        }
        if p.fences.contains(&DOWN) && p.fences.contains(&RIGHT) {
            angles += 1;
        }
        if p.fences.contains(&DOWN) && p.fences.contains(&LEFT) {
            angles += 1;
        }
        if find_fence(p, region, &UP, &RIGHT) && find_fence(p, region, &RIGHT, &UP) {
            angles += 1;
        }
        if find_fence(p, region, &UP, &LEFT) && find_fence(p, region, &LEFT, &UP) {
            angles += 1
        }
        if find_fence(p, region, &DOWN, &RIGHT) && find_fence(p, region, &RIGHT, &DOWN) {
            angles += 1
        }
        if find_fence(p, region, &DOWN, &LEFT) && find_fence(p, region, &LEFT, &DOWN) {
            angles += 1
        }
    }
    return angles;
}

fn get_price(map: &Vec<Vec<Plot>>) -> usize {
    let regions = get_regions(map);
    regions.iter().fold(0, |acc, (_id, region)| {
        acc + region.len() * region.iter().fold(0, |acc, plot| acc + plot.fences.len())
    })
}

fn get_discounted_price(map: &Vec<Vec<Plot>>) -> usize {
    let regions = get_regions(map);
    regions.iter().fold(0, |acc, (_, region)| {
        acc + region.len() * count_sides(region)
    })
}

fn load_input(file: &str) -> Vec<Vec<Plot>> {
    let mut map: Vec<_> = Vec::new();
    let r = BufReader::new(File::open(file).unwrap());
    let mut row: i32 = 0;
    for line in r.lines() {
        let line = line.unwrap();
        let columns: Vec<_> = line
            .chars()
            .enumerate()
            .map(|(c, x)| Plot {
                region_id: -1,
                plant: x,
                r: row,
                c: c as i32,
                fences: Vec::new(),
            })
            .collect();
        map.push(columns);
        row += 1;
    }
    return map;
}

fn lookup(map: &mut Vec<Vec<Plot>>, p: (usize, usize)) {
    for d in DIRS {
        match bound_sum((p.0, p.1), *d, map.len()) {
            None => map[p.0][p.1].fences.push(*d),
            Some((r, c)) => {
                if map[r][c].plant == map[p.0][p.1].plant {
                    if map[r][c].region_id == -1 {
                        map[r][c].region_id = map[p.0][p.1].region_id;
                        lookup(map, (r, c));
                    }
                } else {
                    map[p.0][p.1].fences.push(*d);
                }
            }
        }
    }
}

fn analyze(map: &mut Vec<Vec<Plot>>) {
    let mut next_id = 0;

    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column].region_id != -1 {
                continue;
            }
            map[row][column].region_id = next_id;
            next_id += 1;
            lookup(map, (row, column));
        }
    }
}

fn main() {
    let mut map = load_input("src/day12/input.txt");
    analyze(&mut map);
    println!("part1: {}", get_price(&map));
    println!("part2: {}", get_discounted_price(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dump(map: &Vec<Vec<Plot>>) {
        let regions = get_regions(map);
        for (region, plots) in regions.iter() {
            println!(
                "region {} of {} with {} plots and {} sides, starts at ({} {})",
                region,
                plots[0].plant,
                plots.len(),
                count_sides(plots),
                plots[0].r,
                plots[0].c
            );
        }
    }
    #[test]
    fn check_bound_sum() {
        assert_eq!(None, bound_sum((2, 2), (1, 0), 2));
        assert_eq!(None, bound_sum((2, 2), (1, 0), 3));
        assert_eq!(Some((3, 2)), bound_sum((2, 2), (1, 0), 4));
    }

    #[test]
    fn check() {
        let mut map = load_input("src/day12/test_input.txt");
        analyze(&mut map);
        dump(&map);
        assert_eq!(140, get_price(&map));
    }

    #[test]
    fn check2() {
        let mut map = load_input("src/day12/test_input2.txt");
        analyze(&mut map);
        dump(&map);
        assert_eq!(1930, get_price(&map));
        assert_eq!(1206, get_discounted_price(&map));
    }

    #[test]
    fn check3() {
        let mut map = load_input("src/day12/test_input3.txt");
        analyze(&mut map);
        dump(&map);
        assert_eq!(368, get_discounted_price(&map));
    }
}
