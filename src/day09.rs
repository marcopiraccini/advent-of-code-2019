use crate::intcode;

pub fn part1(input: String) {
    intcode::process(input, 1, false);
}

pub fn part2(input: String) {
    intcode::process(input, 2, false);
}
