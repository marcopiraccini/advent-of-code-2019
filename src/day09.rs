use crate::intcode;

pub fn part1(input: String) {
    let res = intcode::process(input, 1, false);
    println!("Solution: {:?}", res);
}

pub fn part2(input: String) {
    let res = intcode::process(input, 2, false);
    println!("Solution: {:?}", res);
}
