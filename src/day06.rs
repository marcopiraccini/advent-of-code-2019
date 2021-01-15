use std::collections::{HashMap};

fn get_orbits(p: &str, orbs: &HashMap<&str, &str>) -> usize {
    let mut ret = 0;
    let mut parent = p;
    while parent != "COM" {
        parent = orbs.get(parent).unwrap();
        ret += 1;
    }
    ret    
}  

pub fn part1(input: String) {
    let orbs: HashMap<&str, &str> = input.lines().fold(HashMap::new(), |mut acc, p| { 
        let pp = p.split(')').collect::<Vec<&str>>();
        acc.insert(pp[1], pp[0]);
        acc
    });
    let res = orbs.keys().fold(0, |acc, p| acc + get_orbits(p, &orbs));
    println!("    Solution: {}", res); 
}

