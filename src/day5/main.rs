use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn fix_manual(mut manual: Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    for i in (0..manual.len()).rev() {
        match rules.get(&manual[i]) {
            None => continue,
            Some(rule) => {
                for j in 0..i {
                    if rule.contains(&manual[j]) {
                        manual.swap(i, j);
                        return fix_manual(manual, rules);
                    }
                }
            }
        }
    }
    return manual;
}

fn is_ok(manual: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for i in (0..manual.len()).rev() {
        match rules.get(&manual[i]) {
            None => continue,
            Some(rule) => {
                for j in 0..i {
                    if rule.contains(&manual[j]) {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

fn part2(rules: &HashMap<i32, Vec<i32>>, manuals: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0i32;
    for manual in manuals {
        if !is_ok(manual, rules) {
            let fixed = fix_manual(manual.clone(), rules);
            sum += fixed[fixed.len() / 2];
        }
    }
    return sum;
}

fn part1(rules: &HashMap<i32, Vec<i32>>, manuals: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0i32;
    for manual in manuals {
        if !is_ok(manual, rules) {
            sum += manual[manual.len() / 2];
        }
    }
    return sum;
}

fn load_input(file: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let r = BufReader::new(File::open(file).unwrap());
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut manuals: Vec<Vec<i32>> = Vec::new();
    for line in r.lines() {
        let line = line.unwrap();
        if line.contains('|') {
            if let [page, not_after] = line
                .split('|')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()[..]
            {
                if !rules.contains_key(&page) {
                    rules.insert(page, vec![not_after]);
                } else {
                    rules.get_mut(&page).unwrap().push(not_after);
                }
            }
        }
        if line.contains(',') {
            manuals.push(
                line.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }
    return (rules, manuals);
}

fn main() {
    let (rules, manuals) = load_input("src/day5/input.txt");
    println!("part1: {}", part1(&rules, &manuals));
    println!("part2: {}", part2(&rules, &manuals));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_rule() {
        let (rules, manuals) = load_input("src/day5/test_input.txt");
        assert_eq!(is_ok(&manuals[0], &rules), true);
        assert_eq!(is_ok(&manuals[1], &rules), true);
        assert_eq!(is_ok(&manuals[2], &rules), true);
        assert_eq!(is_ok(&manuals[3], &rules), false);
        assert_eq!(is_ok(&manuals[4], &rules), false);
        assert_eq!(is_ok(&manuals[5], &rules), false);
    }
}
