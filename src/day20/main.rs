use pathfinding::prelude::astar;
use rayon::prelude::*;

type Point = (i32, i32);
const UP: Point = (-1, 0);
const RIGHT: Point = (0, 1);
const DOWN: Point = (1, 0);
const LEFT: Point = (0, -1);

#[derive(Debug, Clone, PartialEq, Eq)]
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

fn load_input(txt: &str) -> (Vec<Vec<Cell>>, Point, Point) {
    let mut map = Vec::new();
    let mut parts = txt.split("\n\n");
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let mut row = 0;
    for line in parts.next().unwrap().split("\n") {
        if let Some(col) = line.find('S') {
            start_pos = (row, col as i32)
        }
        if let Some(col) = line.find('E') {
            end_pos = (row, col as i32)
        }
        map.push(line.chars().map(|c| c.into()).collect());
        row += 1;
    }
    return (map, start_pos, end_pos);
}

fn get_points(
    map: &Vec<Vec<Cell>>,
    around: &Point,
    shortcut: &Option<(Point, Point)>,
) -> Vec<(Point, i32)> {
    let mut adj: Vec<(Point, i32)> = [UP, DOWN, RIGHT, LEFT]
        .iter()
        .map(|d| (around.0 + d.0, around.1 + d.1))
        .filter(|x| map[x.0 as usize][x.1 as usize] != Cell::Wall)
        .map(|p| (p, 1))
        .collect();
    if let Some((start, end)) = shortcut {
        if *start == *around && map[end.0 as usize][end.1 as usize] != Cell::Wall {
            adj.push((*end, get_dist(start, end)));
        }
    }
    return adj;
}

fn find_path(
    map: &Vec<Vec<Cell>>,
    start: &Point,
    end: &Point,
    shortcut: &Option<(Point, Point)>,
) -> Option<(Vec<Point>, i32)> {
    astar(
        start,
        |point| get_points(map, point, shortcut),
        |point| (end.0 - point.0).abs() + (end.1 - point.1).abs(),
        |point| *point == *end,
    )
}

fn get_dist(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_shortcuts(map: &Vec<Vec<Cell>>, max_dist: i32) -> Vec<(Point, Point)> {
    let mut shortcuts = Vec::new();
    for r in 1..map.len() - 1 {
        for c in 1..map[r].len() - 1 {
            let start = (r as i32, c as i32);
            if map[r][c] == Cell::Wall || map[r][c] == Cell::End {
                continue;
            }
            for rr in 1..map.len() - 1 {
                for cc in 1..map[rr].len() - 1 {
                    let end = (rr as i32, cc as i32);
                    if map[rr][cc] != Cell::Wall && get_dist(&start, &end) <= max_dist {
                        shortcuts.push((start, end));
                    }
                }
            }
        }
    }
    return shortcuts;
}

fn find_with_shortcuts(
    map: &Vec<Vec<Cell>>,
    start: &Point,
    end: &Point,
    max_shortcut: i32,
) -> Vec<(i32, (Point, Point))> {
    let shortcuts = get_shortcuts(map, max_shortcut);

    let total = shortcuts.len();
    let scores = shortcuts
        .into_par_iter()
        .enumerate()
        .map(|(i, shortcut)| {
            println!("doing shorcut {i} out of {total}");
            if let Some((_path, score)) = find_path(&map, start, end, &Some(shortcut)) {
                Some((score, shortcut))
            } else {
                None
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    return scores;
}

fn main() {
    let input = include_str!("input.txt");
    let (map, start, end) = load_input(input);
    let (_path, base_score) = find_path(&map, &start, &end, &None).unwrap();
    let scores = find_with_shortcuts(&map, &start, &end, 2);
    println!(
        "part1: {}",
        scores
            .iter()
            .filter(|(score, _)| base_score - *score >= 100)
            .count()
    );
    let scores = find_with_shortcuts(&map, &start, &end, 20);
    println!(
        "part1: {}",
        scores
            .iter()
            .filter(|(score, _)| base_score - *score >= 100)
            .count()
    );
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    #[test]
    fn check1() {
        let input = include_str!("test_input.txt");
        let (map, start, end) = load_input(input);
        let (_path, base_score) = find_path(&map, &start, &end, &None).unwrap();
        let scores = find_with_shortcuts(&map, &start, &end, 20);
        println!(
            "{:?}",
            scores.iter().counts_by(|(score, _)| base_score - score)
        );
    }
}
