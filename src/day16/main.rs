use pathfinding::prelude::astar_bag;
use pathfinding::prelude::AstarSolution;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

impl Into<Cell> for char {
    fn into(self) -> Cell {
        match self {
            '#' => Cell::Wall,
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => Cell::Empty,
        }
    }
}

type Point = (i32, i32);
const UP: Point = (-1, 0);
const RIGHT: Point = (0, 1);
const DOWN: Point = (1, 0);
const LEFT: Point = (0, -1);

fn load_input(txt: &str) -> (Vec<Vec<Cell>>, Point) {
    let mut map = Vec::new();
    let mut parts = txt.split("\n\n");
    let mut start_pos = (0, 0);
    let mut row = 0;
    for line in parts.next().unwrap().split("\n") {
        if let Some(col) = line.find('S') {
            start_pos = (row, col as i32)
        }
        map.push(line.chars().map(|c| c.into()).collect());
        row += 1;
    }
    return (map, start_pos);
}

fn get_best(map: &Vec<Vec<Cell>>, p: Point) -> Option<(AstarSolution<(Point, Point)>, i32)> {
    let end_pos = (1, (map.len() - 2) as i32);
    astar_bag(
        &(p, RIGHT),
        |(p, dir)| {
            let mut s = Vec::new();
            for (d, cost) in get_moves(dir) {
                let n = (p.0 + d.0, p.1 + d.1);
                if map[n.0 as usize][n.1 as usize] != Cell::Wall {
                    s.push(((n, d), cost))
                }
            }
            return s;
        },
        |(p, _)| (end_pos.0 - p.0).abs() + (end_pos.1 - p.1).abs(),
        |(p, _)| map[p.0 as usize][p.1 as usize] == Cell::End,
    )
}

fn get_moves(d: &Point) -> Vec<(Point, i32)> {
    match *d {
        UP => vec![(UP, 1), (LEFT, 1001), (RIGHT, 1001)],
        DOWN => vec![(DOWN, 1), (RIGHT, 1001), (LEFT, 1001)],
        LEFT => vec![(LEFT, 1), (DOWN, 1001), (UP, 1001)],
        RIGHT => vec![(RIGHT, 1), (UP, 1001), (DOWN, 1001)],
        _ => Vec::new(),
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (map, start_pos) = load_input(input);
    // let deers = find_paths(&map, deer).unwrap();
    // let score = deers.iter().map(|x| x.get_score()).min().unwrap();
    // println!("routes {:?}, best {}", deers.len(), score);
    let (paths, score) = get_best(&map, start_pos).unwrap();
    println!("part1: {}", score);
    let mut tiles = HashSet::new();
    for path in paths {
        for point in path {
            tiles.insert(point.0);
        }
    }
    println!("part2: {}", tiles.len())

    // let paths = find_paths(&map, deer, best_path.1 as usize).unwrap();
    // println!("paths: {:?}", paths);
}
