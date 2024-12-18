use itertools::Itertools;
use pathfinding::prelude::astar;

type Point = (i32, i32);
const UP: Point = (-1, 0);
const RIGHT: Point = (0, 1);
const DOWN: Point = (1, 0);
const LEFT: Point = (0, -1);

fn load_input(txt: &str) -> Vec<Point> {
    txt.split("\n")
        .map(|l| {
            l.split(",")
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn get_points(around: &Point) -> Vec<Point> {
    [UP, DOWN, RIGHT, LEFT]
        .iter()
        .map(|d| (around.0 + d.0, around.1 + d.1))
        .filter(|p| p.0 >= 0 && p.0 <= 70 && p.1 >= 0 && p.1 <= 70)
        .collect()
}

fn find_path(bytes: &[Point]) -> Option<(Vec<Point>, i32)> {
    astar(
        &(0, 0),
        |point| {
            get_points(point)
                .iter()
                .filter(|p| !bytes.contains(p))
                .map(|x| (*x, 1))
                .collect::<Vec<(Point, i32)>>()
        },
        |point| 70 - point.0 + 70 - point.1,
        |point| *point == (70, 70),
    )
}

fn main() {
    let input = include_str!("input.txt");
    let bytes = load_input(input);

    let (steps, score) = find_path(&bytes[..1024]).unwrap();
    println!("part1: {} score {score}", steps.len());
    for cutoff in 0..bytes.len() {
        match find_path(&bytes[..=cutoff]) {
            Some(_) => {
                continue;
            }
            None => {
                println!("ok: {:?}", bytes[cutoff]);
                return;
            }
        }
    }
}
