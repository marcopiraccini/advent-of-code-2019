use std::collections::HashMap;

fn get_orbits(p: &str, orbs: &HashMap<String, String>) -> usize {
    let mut ret = 0;
    let mut parent = p;
    while parent != "COM" {
        parent = orbs.get(parent).unwrap();
        ret += 1;
    }
    ret
}

fn get_orbs(input: String) -> HashMap<String, String> {
    let res = input.lines().fold(HashMap::new(), |mut acc, p| {
        let pp = p.split(')').collect::<Vec<&str>>();
        acc.insert(pp[1].to_string(), pp[0].to_string());
        acc
    });
    res
}

pub fn part1(input: String) {
    let orbs = get_orbs(input);
    let res = orbs.keys().fold(0, |acc, p| acc + get_orbits(p, &orbs));
    println!("    Solution: {}", res);
}

fn path_to(start: &str, end: &str, orbs: &HashMap<String, String>) -> Vec<String> {
    let mut path: Vec<String> = vec![];
    let mut current = start;
    while current != end {
        current = orbs.get(current).unwrap();
        path.push(current.to_string());
    }
    path
}

fn common_ancestor(a: &[String], b: &[String]) -> String {
    for el in a.iter() {
        if b.contains(el) {
            return el.to_string();
        }
    }
    "COM".to_string()
}

fn distance(end: &str, path: &[String]) -> usize {
    for (l, el) in path.iter().enumerate() {
        if *el == *end {
            return l;
        }
    }
    path.len()
}

pub fn part2(input: String) {
    let orbs = get_orbs(input);
    let path_you = path_to("YOU", "COM", &orbs);
    let path_san = path_to("SAN", "COM", &orbs);
    let ancestor = common_ancestor(&path_you, &path_san);
    let res = distance(&ancestor, &path_you) + distance(&ancestor, &path_san);
    println!("    Solution: {} ", res);
}
