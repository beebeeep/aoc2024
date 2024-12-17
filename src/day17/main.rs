use pathfinding::prelude::astar_bag;
use pathfinding::prelude::AstarSolution;
use std::ops::BitAndAssign;
use std::ops::Range;
use std::ops::RangeInclusive;
use std::thread;
use std::{collections::HashSet, hash::Hash};

const OPCODES: [&'static str; 8] = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];
const OPERANDS: [&'static str; 8] = ["0", "1", "2", "3", "A", "B", "C", "7"];

#[derive(Debug, Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    prog: Vec<u64>,
    out: Vec<u64>,
}
fn get_val(v: &str) -> &str {
    v.split(": ").last().unwrap()
}
impl Computer {
    fn new(input: &str) -> Self {
        let mut lines = input.split("\n");
        Self {
            a: get_val(lines.next().unwrap()).parse().unwrap(),
            b: get_val(lines.next().unwrap()).parse().unwrap(),
            c: get_val(lines.next().unwrap()).parse().unwrap(),
            ip: 0,
            prog: get_val(lines.skip(1).next().unwrap())
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect(),
            out: Vec::new(),
        }
    }
    fn combo_operand(&self, v: u64) -> Result<u64, &'static str> {
        match v {
            0..=3 => Ok(v as u64),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err("invalid opcode"),
        }
    }
    fn dump_prog(&self) {
        let mut i = 0;
        while i < self.prog.len() - 1 {
            println!(
                "{} {};",
                OPCODES[self.prog[i] as usize],
                OPERANDS[self.prog[i + 1] as usize]
            );
            i += 2;
        }
    }

    fn step(&mut self, opcode: u64, operand: u64) -> Result<(), &'static str> {
        let mut inc_ip = true;
        match opcode {
            /* adv */ 0 => self.a = self.a / 2u64.pow(self.combo_operand(operand)? as u32),
            /* bxl */ 1 => self.b = self.b ^ operand,
            /* bst */ 2 => self.b = self.combo_operand(operand)? % 8,
            /* jnz */
            3 => {
                if self.a != 0 {
                    inc_ip = false;
                    self.ip = operand as usize;
                }
            }
            /* bxc */ 4 => self.b = self.b ^ self.c,
            /* out */ 5 => self.out.push(self.combo_operand(operand)? % 8),
            /* bdv */ 6 => self.b = self.a / 2u64.pow(self.combo_operand(operand)? as u32),
            /* cdv */ 7 => self.c = self.a / 2u64.pow(self.combo_operand(operand)? as u32),
            /* nop */ _ => return Err("invalid opcode"),
        };
        if inc_ip {
            self.ip += 2;
        }
        Ok(())
    }

    fn run(&mut self, limit: usize) -> Option<Vec<u64>> {
        for _step in 0..limit {
            if self.ip >= self.prog.len() - 1 {
                return Some(self.out.clone());
            }
            let opcode = self.prog[self.ip];
            let operand = self.prog[self.ip + 1];
            if let Err(_) = self.step(opcode, operand) {
                return None;
            }
        }
        return None;
    }

    fn check_quine(&mut self, limit: usize) -> bool {
        for _step in 0..limit {
            if self.ip >= self.prog.len() - 1 {
                break;
            }
            let opcode = self.prog[self.ip];
            let operand = self.prog[self.ip + 1];
            if let Err(_) = self.step(opcode, operand) {
                return false;
            }
        }
        if self.prog == self.out {
            return true;
        }
        return false;
    }
}

fn try_find(mut comp: Computer, r: RangeInclusive<u64>) -> Option<u64> {
    let rs = format!("{:?}", r);
    println!("range {rs} started");
    for a in r {
        // reset the state
        comp.a = a;
        comp.b = 0;
        comp.c = 0;
        comp.ip = 0;
        comp.out.truncate(0);

        if comp.check_quine(comp.prog.len() * comp.prog.len()) {
            println!("found!!! {a}");
            return Some(a);
        }
    }
    return None;
}

fn find_a(orig: &Computer) -> u64 {
    let mut triplets_matched = 0;
    let mut a = 0;
    let mut steps = 0;
    loop {
        let mut comp = orig.clone();
        comp.a = a;
        if let Some(out) = comp.run(256) {
            if out.len() >= triplets_matched + 1
                && out[..triplets_matched + 1] == comp.prog[..triplets_matched + 1]
            {
                triplets_matched += 1;
                println!("matched {triplets_matched}");
                if triplets_matched == 16 {
                    break;
                }
                continue;
            }
        }
        a += 2u64.pow((triplets_matched) as u32);
        steps += 1;
    }
    println!("i did {steps} steps");
    return a;
}

// that'll take around 16 thousands years
fn brute_force(comp: &Computer) {
    let mut handles = Vec::new();
    let step = (u64::MAX / 16) as u64;
    for i in 0..16 {
        let c = comp.clone();
        let r = step * i..=step * (i + 1);
        handles.push(thread::spawn(move || try_find(c, r)));
    }
    for h in handles {
        if let Some(v) = h.join().unwrap() {
            println!("part2: {v}");
            return;
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut comp = Computer::new(input);

    let out = comp.clone().run(1000).unwrap();
    println!(
        "part1: {:?}",
        out.iter()
            .map(u64::to_string)
            .collect::<Vec<String>>()
            .join(",")
    );
    // brute_force(&comp);
    println!("part2: {}", find_a(&comp));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check1() {
        let input = include_str!("test_input.txt");
        let mut comp = Computer::new(input);
        assert_eq!(Some(117440), try_find(comp, 0..=1000000));
    }

    #[test]
    fn check2() {
        let input = include_str!("input.txt");
        let mut comp = Computer::new(input);
        comp.dump_prog();
    }
}
