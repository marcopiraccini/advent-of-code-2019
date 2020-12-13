#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<&str> {
    // collect transforms an iterator in a collection
    input.lines().collect()
}


#[aoc(day1, part1)]
pub fn solve_part1(input: Vec<&str>) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&input_generator("2x3x4")), 58);
    }
}
