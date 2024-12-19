use regex::Regex;

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    x: i64,
    y: i64,
}
fn calculate(machine: &Machine) -> Option<i64> {
    let (ax, ay, bx, by, x, y) = (
        machine.ax as f64,
        machine.ay as f64,
        machine.bx as f64,
        machine.by as f64,
        machine.x as f64,
        machine.y as f64,
    );
    // doing math wasn't fun as it used to be in school :/
    let apushes = (x * by - y * bx) / (ax * by - bx * ay);
    let bpushes = (y - apushes * ay) / by;
    if apushes == apushes.round() && bpushes == bpushes.round() {
        return Some(3 * (apushes as i64) + bpushes as i64);
    }
    return None;
}

fn play(machine: &Machine) -> Option<i64> {
    let mut tokens = std::i64::MAX;

    for pushb in 0..100 {
        for pusha in 0..100 {
            if machine.ax * pusha + machine.bx * pushb == machine.x
                && machine.ay * pusha + machine.by * pushb == machine.y
            {
                let price = pusha * 3 + pushb;
                if price < tokens {
                    tokens = price;
                }
            }
        }
    }
    if tokens == std::i64::MAX {
        return None;
    }
    return Some(tokens);
}

fn load_input(txt: &str) -> Vec<Machine> {
    let mut machines: Vec<_> = Vec::new();
    let re = Regex::new(r"(\d+).+?(\d+)").unwrap();
    for para in txt.split("\n\n") {
        let mut l = para.split("\n");
        let l1 = re.captures(l.next().unwrap()).unwrap();
        let l2 = re.captures(l.next().unwrap()).unwrap();
        let l3 = re.captures(l.next().unwrap()).unwrap();
        machines.push(Machine {
            ax: l1[1].parse().unwrap(),
            ay: l1[2].parse().unwrap(),
            bx: l2[1].parse().unwrap(),
            by: l2[2].parse().unwrap(),
            x: l3[1].parse().unwrap(),
            y: l3[2].parse().unwrap(),
        });
    }

    return machines;
}

fn main() {
    let input = include_str!("input.txt");
    let mut machines = load_input(input);
    let mut spent = 0;
    for m in &machines {
        if let Some(price) = calculate(&m) {
            spent += price;
        }
    }
    println!("part 1: {spent}");
    for i in 0..machines.len() {
        machines[i].x += 10000000000000;
        machines[i].y += 10000000000000;
    }
    spent = 0;
    for m in &machines {
        if let Some(price) = calculate(&m) {
            spent += price;
        }
    }
    println!("part 2: {spent}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        let input = include_str!("test_input.txt");
        let machines = load_input(input);
        println!("{:?}", machines);
        for m in machines {
            println!("{:?}", calculate(&m));
        }
    }
}
