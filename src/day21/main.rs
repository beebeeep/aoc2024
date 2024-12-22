use std::iter::repeat;

use itertools::Itertools;

type Point = (i32, i32);

const NUMPAD: &'static [&'static [char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &['X', '0', 'A'],
];

fn coord(x: &char, kb: &'static [&'static [char]]) -> Point {
    for r in 0..kb.len() {
        for c in 0..kb[r].len() {
            if kb[r][c] == *x {
                return (r as i32, c as i32);
            }
        }
    }
    panic!("unkonwn coord for {x}");
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn route_arr(source: &char, target: &char) -> Vec<Vec<char>> {
    match (source, target) {
        ('A', '^') => vec![vec!['<']],
        ('A', '>') => vec![vec!['v']],
        ('A', 'v') => vec![vec!['<', 'v'], vec!['v', '<']],
        ('A', '<') => vec![vec!['v', '<', '<']],

        ('^', 'A') => vec![vec!['>']],
        ('^', '>') => vec![vec!['v', '>'], vec!['>', 'v']],
        ('^', 'v') => vec![vec!['v']],
        ('^', '<') => vec![vec!['v', '<']],

        ('<', '^') => vec![vec!['>', '^']],
        ('<', '>') => vec![vec!['>', '>']],
        ('<', 'v') => vec![vec!['>']],
        ('<', 'A') => vec![vec!['>', '>', '^']],

        ('>', '^') => vec![vec!['<', '^'], vec!['^', '<']],
        ('>', 'A') => vec![vec!['^']],
        ('>', 'v') => vec![vec!['<']],
        ('>', '<') => vec![vec!['<', '<']],

        ('v', '^') => vec![vec!['^']],
        ('v', '>') => vec![vec!['>']],
        ('v', 'A') => vec![vec!['^', '>'], vec!['>', '^']],
        ('v', '<') => vec![vec!['<']],

        (x, y) if x == y => vec![],

        (_, _) => panic!("unknown arrows: {source} -> {target}"),
    }
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
fn route_numpad(source: &char, target: &char) -> Vec<Vec<char>> {
    let mut routes = Vec::new();
    let (src, tgt) = (coord(source, NUMPAD), coord(target, NUMPAD));
    let (dr, dc) = (tgt.0 - src.0, tgt.1 - src.1);

    let hor: Vec<char> = repeat(if dc < 0 { '<' } else { '>' })
        .take(dc.abs() as usize)
        .collect();
    let vert: Vec<char> = repeat(if dr < 0 { '^' } else { 'v' })
        .take(dr.abs() as usize)
        .collect();

    let only_hor_first = ['7', '4', '1'].contains(source) && ['0', 'A'].contains(target);
    let only_vert_first = ['0', 'A'].contains(source) && ['7', '4', '1'].contains(target);

    if !only_vert_first {
        let mut hor_first = hor.clone();
        hor_first.append(&mut vert.clone());
        routes.push(hor_first);
    }
    if !only_hor_first {
        let mut vert_first = vert.clone();
        vert_first.append(&mut hor.clone());
        if !routes.contains(&vert_first) {
            routes.push(vert_first);
        }
    }

    return routes;
}

// dial_num returns input for the directional control attached to numpad robot
fn dial_num(num: char, start: char) -> Vec<String> {
    let mut numpad_inputs = route_numpad(&start, &num);
    if numpad_inputs.len() == 0 {
        return vec![String::from("A")];
    }
    for input in &mut numpad_inputs {
        input.push('A');
    }
    return numpad_inputs
        .into_iter()
        .map(|x| x.into_iter().collect::<String>())
        .collect();
}

// dial_arr returns input for the directional control attached to another directional control
fn dial_arr(arr: char, start: char) -> Vec<String> {
    let mut inputs = route_arr(&start, &arr);
    if inputs.len() == 0 {
        return vec![String::from("A")];
    }
    for input in &mut inputs {
        input.push('A');
    }
    return inputs
        .into_iter()
        .map(|x| x.into_iter().collect::<String>())
        .collect();
}

// takes vector of possible inputs for each element of some sequence
// and returns vector of all possible combinations of dial the sequence
fn get_input_combinations(possible_inputs: Vec<Vec<String>>) -> impl Iterator<Item = Vec<String>> {
    possible_inputs
        .into_iter()
        .multi_cartesian_product()
        .take(1)
    // .collect()
}

// add_proxy returns all possible inputs for adding one level of indirection to dial specified arrow input
fn add_proxy(input: String) -> impl Iterator<Item = String> {
    let mut pos = 'A';
    let mut possible_inputs = Vec::new();

    for ch in input.chars() {
        possible_inputs.push(dial_arr(ch, pos));
        pos = ch;
    }

    let combinations = get_input_combinations(possible_inputs);
    combinations.map(|input| input.join(""))
}

fn add_proxies(input: String, proxies: usize) -> Vec<String> {
    if proxies == 0 {
        return vec![input.chars().collect()];
    }
    let mut result = Vec::new();
    for option in add_proxy(input) {
        result.append(&mut add_proxies(option, proxies - 1));
    }
    return result;
}

fn dial_num_via_proxies(num: char, start: char, proxies: usize) -> usize {
    let inputs = dial_num(num, start);
    let min_size = inputs
        .into_iter()
        .map(|input| {
            add_proxies(input, proxies)
                .into_iter()
                .min_by_key(|x| x.len())
                .unwrap()
        })
        .min_by_key(|x| x.len())
        .unwrap()
        .len();
    return min_size;
}

fn dial_code_via_proxies(code: &str, proxies: usize) -> usize {
    let mut pos = 'A';
    let mut size = 0;
    for num in code.chars() {
        size += dial_num_via_proxies(num, pos, proxies);
        pos = num;
    }
    return size;
}

fn load_input(txt: &str) -> Vec<String> {
    txt.split("\n").map(String::from).collect()
}

fn main() {
    let input = load_input(include_str!("input.txt"));
    let codes: Vec<_> = input.iter().map(|x| dial_code_via_proxies(x, 2)).collect();
    let mut complexity = 0;
    for i in 0..input.len() {
        complexity += input[i]
            .strip_suffix("A")
            .unwrap()
            .parse::<usize>()
            .unwrap()
            * codes[i]
    }
    println!("part1: {complexity}");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn check_route() {
        for tc in [
            ('0', 'A', vec![">"]),
            ('A', '7', vec!["^^^<<"]),
            ('7', '0', vec![">vvv"]),
            ('1', '9', vec![">>^^", "^^>>"]),
            ('7', '5', vec![">v", "v>"]),
            ('3', '7', vec!["<<^^", "^^<<"]),
            ('4', '5', vec![">"]),
            ('5', '0', vec!["vv"]),
            ('5', '5', vec![""]),
        ] {
            let opts =
                tc.2.iter()
                    .map(|x| x.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>();
            assert_eq!(
                route_numpad(&tc.0, &tc.1),
                opts,
                "from {} to {}",
                tc.0,
                tc.1
            );
        }
    }
}
