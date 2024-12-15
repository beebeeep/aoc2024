use std::{
    collections::HashSet,
    fmt,
    hash::Hash,
    io::{stdin, Empty, Read},
    thread::sleep,
    time,
};

use console::Term;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Robot,
    Box,
    LCrate,
    RCrate,
}

impl Into<Cell> for char {
    fn into(self) -> Cell {
        match self {
            '#' => Cell::Wall,
            'O' => Cell::Box,
            '@' => Cell::Robot,
            '[' => Cell::LCrate,
            ']' => Cell::RCrate,
            _ => Cell::Empty,
        }
    }
}

impl From<Cell> for char {
    fn from(t: Cell) -> char {
        match t {
            Cell::Wall => '#',
            Cell::Box => 'O',
            Cell::Robot => '@',
            Cell::LCrate => '[',
            Cell::RCrate => ']',
            _ => '.',
        }
    }
}

type Dir = (i32, i32);
const UP: Dir = (-1, 0);
const RIGHT: Dir = (0, 1);
const DOWN: Dir = (1, 0);
const LEFT: Dir = (0, -1);

fn load_input(txt: &str) -> (Vec<Vec<Cell>>, Vec<Dir>, Dir) {
    let mut map = Vec::new();
    let mut parts = txt.split("\n\n");
    let mut row = 0;
    let mut robot_pos = (0, 0);
    for line in parts.next().unwrap().split("\n") {
        if let Some(col) = line.find('@') {
            robot_pos = (row, col as i32)
        }
        map.push(line.chars().map(|c| c.into()).collect());
        row += 1;
    }
    let moves = parts
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '>' => RIGHT,
            '<' => LEFT,
            'v' => DOWN,
            _ => UP,
        })
        .collect();

    return (map, moves, robot_pos);
}

fn print(term: &Term, map: &Vec<Vec<Cell>>, m: &Dir) {
    term.move_cursor_to(0, 0);
    for row in map {
        term.write_line(
            &format!(
                "{}",
                String::from(
                    row.iter()
                        .map(|x| char::from(x.clone()))
                        .collect::<String>(),
                )
            )
            .replace(
                "@",
                match *m {
                    UP => "^",
                    DOWN => "v",
                    LEFT => "<",
                    RIGHT => ">",
                    _ => "@",
                },
            ),
        );
    }
}

fn collect_movables(map: &Vec<Vec<Cell>>, cell: &Dir, m: &Dir) -> Option<Vec<Dir>> {
    let next = (cell.0 + m.0, cell.1 + m.1);
    match map[cell.0 as usize][cell.1 as usize] {
        Cell::Empty => Some(vec![]),
        Cell::Wall => None,
        Cell::Robot => {
            panic!("second robot?")
        }
        Cell::Box => match collect_movables(map, &next, m) {
            None => return None,
            Some(mut c) => {
                c.push(*cell);
                Some(c)
            }
        },
        Cell::LCrate if *m == LEFT || *m == RIGHT => match collect_movables(map, &next, m) {
            None => return None,
            Some(mut c) => {
                c.push(*cell);
                Some(c)
            }
        },
        Cell::RCrate if *m == LEFT || *m == RIGHT => match collect_movables(map, &next, m) {
            None => return None,
            Some(mut c) => {
                c.push(*cell);
                Some(c)
            }
        },
        Cell::LCrate => {
            match (
                collect_movables(map, &next, m),
                collect_movables(map, &(next.0, next.1 + 1), m),
            ) {
                (None, _) => return None,
                (_, None) => return None,
                (Some(mut a), Some(mut b)) => {
                    a.append(&mut b);
                    a.push(*cell);
                    a.push((cell.0, cell.1 + 1));
                    return Some(a);
                }
            }
        }
        Cell::RCrate => {
            match (
                collect_movables(map, &next, m),
                collect_movables(map, &(next.0, next.1 - 1), m),
            ) {
                (None, _) => return None,
                (_, None) => return None,
                (Some(mut a), Some(mut b)) => {
                    a.append(&mut b);
                    a.push(*cell);
                    a.push((cell.0, cell.1 - 1));
                    return Some(a);
                }
            }
        }
    }
}

fn move_robot(
    map: &mut Vec<Vec<Cell>>,
    moves: &Vec<Dir>,
    robot_pos: &mut Dir,
    mut interactive: bool,
) {
    let term = Term::stdout();
    term.clear_screen();
    for (i, m) in moves.iter().enumerate() {
        if interactive {
            // term.clear_screen();
            print(&term, &map, m);
            term.write_line(&format!(
                "step {i} robot at {:?}, moving {}    ",
                robot_pos,
                match *m {
                    UP => "up",
                    DOWN => "down",
                    LEFT => "left",
                    RIGHT => "right",
                    _ => "?",
                }
            ));
        }
        let next = (robot_pos.0 + m.0, robot_pos.1 + m.1);
        match collect_movables(map, &next, &m) {
            None => continue,
            Some(cells) => {
                let old_map = map.clone();
                let mut hs = HashSet::new();
                for cell in cells {
                    if hs.contains(&cell) {
                        continue;
                    }
                    hs.insert(cell);
                    map[(cell.0 + m.0) as usize][(cell.1 + m.1) as usize] =
                        old_map[cell.0 as usize][cell.1 as usize].clone();
                    map[cell.0 as usize][cell.1 as usize] = Cell::Empty;
                }
                map[robot_pos.0 as usize][robot_pos.1 as usize] = Cell::Empty;
                map[next.0 as usize][next.1 as usize] = Cell::Robot;
                *robot_pos = next;
            }
        }
        if interactive {
            // stdin().read(&mut [0]).unwrap();
            sleep(time::Duration::from_millis(100));
        }
    }
}

fn calc(map: &Vec<Vec<Cell>>) -> usize {
    let mut sum = 0;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            match map[r][c] {
                Cell::Box => sum += 100 * r + c,
                Cell::LCrate => sum += 100 * r + c,
                _ => {}
            }
        }
    }
    return sum;
}

fn get_wide_map(map: &Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, Dir) {
    let mut robot_pos: Dir = (0, 0);
    let mut wide_map: Vec<Vec<Cell>> = Vec::new();
    for (r, row) in map.iter().enumerate() {
        let mut wr = Vec::new();
        for cell in row.iter() {
            match cell {
                Cell::Box => wr.append(&mut vec![Cell::LCrate, Cell::RCrate]),
                Cell::Robot => {
                    wr.append(&mut vec![Cell::Robot, Cell::Empty]);
                    robot_pos = (r as i32, (wr.len() - 2) as i32);
                }
                x => wr.append(&mut vec![x.clone(), x.clone()]),
            }
        }
        wide_map.push(wr)
    }
    return (wide_map, robot_pos);
}

fn main() {
    let input = include_str!("input.txt");
    let (mut map, moves, mut robot_pos) = load_input(input);
    move_robot(&mut map, &moves, &mut robot_pos, false);
    println!("part1: {}", calc(&map));

    let (map, moves, _) = load_input(input);
    let (mut wide_map, mut robot_pos) = get_wide_map(&map);
    move_robot(&mut wide_map, &moves, &mut robot_pos, true);
    println!("part2: {}", calc(&wide_map));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check1() {
        let input = include_str!("test_input.txt");
        let term = Term::stdout();
        let (mut map, moves, mut robot_pos) = load_input(input);
        move_robot(&mut map, &moves, &mut robot_pos, false);
        print(&term, &map, &(0, 0));
        assert_eq!(10092, calc(&map))
    }

    #[test]
    fn check2() {
        let input = include_str!("test_input.txt");
        let term = Term::stdout();
        let (mut map, moves, mut robot_pos) = load_input(input);
        (map, robot_pos) = get_wide_map(&map);
        move_robot(&mut map, &moves, &mut robot_pos, true);
        assert_eq!(9021, calc(&map))
    }

    #[test]
    fn check3() {
        let input = include_str!("test_input2.txt");
        let (mut map, moves, mut robot_pos) = load_input(input);
        move_robot(&mut map, &moves, &mut robot_pos, true);
    }
}
