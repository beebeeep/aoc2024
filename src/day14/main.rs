use flate2::write::ZlibEncoder;
use flate2::Compression;
use regex::Regex;
use std::io::prelude::*;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn render(map: &Vec<Vec<i32>>) -> String {
    return map
        .iter()
        .map(|r| {
            r.iter()
                .map(|v| {
                    if *v == 0 {
                        '.'
                    } else {
                        v.to_string().chars().next().unwrap()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
}

fn load_input(txt: &str, size_x: i32, size_y: i32) -> (Vec<Robot>, Vec<Vec<i32>>) {
    let mut robots: Vec<_> = Vec::new();
    let mut map: Vec<Vec<i32>> = vec![vec![0; size_y as usize]; size_x as usize];
    let re = Regex::new(r"(\d+).+?(\d+).+?(-?\d+).+?(-?\d+)").unwrap();
    for line in txt.split("\n") {
        let m = re.captures(line).unwrap();
        let r = Robot {
            x: m[1].parse().unwrap(),
            y: m[2].parse().unwrap(),
            dx: m[3].parse().unwrap(),
            dy: m[4].parse().unwrap(),
        };
        map[r.x as usize][r.y as usize] += 1;
        robots.push(r);
    }
    return (robots, map);
}

fn patrol(
    robots: &mut Vec<Robot>,
    steps: usize,
    map: &mut Vec<Vec<i32>>,
    size_x: i32,
    size_y: i32,
) {
    for _step in 0..steps {
        for robot in robots.iter_mut() {
            map[robot.x as usize][robot.y as usize] -= 1;
            robot.x = robot.x + robot.dx;
            robot.y = robot.y + robot.dy;
            if robot.x < 0 {
                robot.x = size_x + robot.x;
            }
            if robot.y < 0 {
                robot.y = size_y + robot.y;
            }
            if robot.x > size_x - 1 {
                robot.x = robot.x - size_x;
            }
            if robot.y > size_y - 1 {
                robot.y = robot.y - size_y;
            }
            // assert!(robot.x >= 0 && robot.x < size_x);
            // assert!(robot.y >= 0 && robot.y < size_y);
            map[robot.x as usize][robot.y as usize] += 1
        }
    }
}

fn get_safety_factor(map: &Vec<Vec<i32>>) -> (i32, i32, i32, i32) {
    let mx = map.len() / 2;
    let my = map[0].len() / 2;
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            //    mx
            // q1 | q2
            // ---+---- my
            // q4 | q3
            if x < mx && y < my {
                q1 += map[x][y]
            }
            if x > mx && y < my {
                q2 += map[x][y]
            }
            if x > mx && y > my {
                q3 += map[x][y]
            }
            if x < mx && y > my {
                q4 += map[x][y]
            }
        }
    }
    return (q1, q2, q3, q4);
}

fn main() {
    let input = include_str!("input.txt");
    let (mut robots, mut map) = load_input(input, 101, 103);
    patrol(&mut robots, 100, &mut map, 101, 103);
    let (q1, q2, q3, q4) = get_safety_factor(&map);
    println!("part 1: {}", q1 * q2 * q3 * q4,);

    (robots, map) = load_input(input, 101, 103);
    for i in 0..10000000 {
        patrol(&mut robots, 1, &mut map, 101, 103);
        let (q1, q2, q3, q4) = get_safety_factor(&map);
        // render the pic into text compress it and see when resulting size is suspiciously low,
        // i.e. the pic has lower enthropy
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        let pic = render(&map);
        e.write_all(&pic.as_bytes()).unwrap();
        let b = e.finish().unwrap();
        if b.len() < 600 {
            println!("step {i} {q1} {q2} {q3} {q4} {}\n{}", b.len(), pic);
            break;
        }
    }
}
