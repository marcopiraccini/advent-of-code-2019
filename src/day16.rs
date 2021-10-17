// step and pos are from 0
fn get_pattern_val(step: usize, pos: usize) -> isize {
    if pos < step {
        return 0;
    }
    let rel_pos = pos - step;
    let pat_length = 4 * (step + 1);
    let pat_pos = rel_pos % pat_length;
    let block = pat_pos / (pat_length / 4);
    match block {
        0 => 1,
        2 => -1,
        _ => 0,
    }
}

fn apply_pattern(input: &Vec<isize>, step: usize, start_from: usize) -> isize {
    let mut res: isize = 0;
    for i in start_from..input.len() {
        res += input[i] * get_pattern_val(step, i);
    }
    res.abs() % 10
}

fn apply_phase(input: Vec<isize>, start_from: usize) -> Vec<isize> {
    let size = input.len();
    let mut ret: Vec<isize> = Vec::new();
    for step in start_from..size {
        let val = apply_pattern(&input, step, start_from);
        ret.push(val);
    }
    ret
}

pub fn part1(mut input: String) {
    input.pop(); // removes the trailing \n
    let mut curr = input
        .chars()
        .map(|x| x.to_string().parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    for _ in 0..100 {
        curr = apply_phase(curr, 0)
    }

    curr.truncate(8);
    let sol = curr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("Solution part 1: {:?}", sol);
}

fn to_number(input: &[isize]) -> isize {
    input.iter().fold(0, |total, current| total * 10 + current)
}

// OK, I didn't get this by myself, this is adapterd from https://github.com/prscoelho/aoc2019/blob/master/src/aoc16/mod.rs
pub fn part2(mut input: String) {
    input.pop(); // removes the trailing \n
    let curr = input
        .chars()
        .map(|x| x.to_string().parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let start = to_number(&curr[0..7]) as usize;
    let end = curr.len() * 10_000;
    let mut current = Vec::new();
    for i in start..end {
        current.push(curr[i % curr.len()]);
    }

    for _ in 0..100 {
        let mut sums = Vec::new();
        let mut total = 0;
        sums.push(0);
        for i in 0..current.len() {
            total += current[i];
            sums.push(total);
        }
        for i in 0..current.len() {
            let value = sums.last().unwrap() - sums[i];
            current[i] = value % 10;
        }
    }
    let res = to_number(&current[0..8]);
    println!("Solution part 2: {:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1() {
        assert_eq!(get_pattern_val(0, 0), 1);
        assert_eq!(get_pattern_val(0, 1), 0);
        assert_eq!(get_pattern_val(0, 2), -1);
        assert_eq!(get_pattern_val(0, 3), 0);
        assert_eq!(get_pattern_val(0, 4), 1);
        assert_eq!(get_pattern_val(1, 0), 0);
        assert_eq!(get_pattern_val(1, 1), 1);
        assert_eq!(get_pattern_val(1, 2), 1);
        assert_eq!(get_pattern_val(1, 3), 0);
        assert_eq!(get_pattern_val(1, 4), 0);
        assert_eq!(get_pattern_val(1, 5), -1);
        assert_eq!(get_pattern_val(2, 0), 0);
        assert_eq!(get_pattern_val(2, 1), 0);
        assert_eq!(get_pattern_val(2, 2), 1);
        assert_eq!(get_pattern_val(2, 3), 1);
        assert_eq!(get_pattern_val(2, 4), 1);
        assert_eq!(get_pattern_val(2, 5), 0);
    }
}
