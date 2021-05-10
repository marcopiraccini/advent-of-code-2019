use std::collections::VecDeque;

fn fill_pattern(p: &mut Vec<isize>, val: isize, base: usize) -> &mut Vec<isize> {
    for _ in 0..base {
        p.push(val);
    }
    p
}

fn get_pattern(step: usize, n: usize) -> Vec<isize> {
    let base = 1 + step; // assume that 0 is the first step
    let mut ret: VecDeque<isize> = VecDeque::new();
    let mut pattern: Vec<isize> = vec![];
    fill_pattern(&mut pattern, 0, base);
    fill_pattern(&mut pattern, 1, base);
    fill_pattern(&mut pattern, 0, base);
    fill_pattern(&mut pattern, -1, base);
    while ret.len() <= n {
        ret.extend(&pattern);
    }
    ret.pop_front();
    ret.truncate(n);
    ret.iter().map(|x| *x).collect()
}

fn apply_pattern(input: &Vec<isize>, pattern: &Vec<isize>) -> isize {
    let mut res: isize = 0;
    for i in 0..input.len() {
        res += input[i] * pattern[i]
    }
    res.abs() % 10
}

fn apply_phase(input: Vec<isize>) -> Vec<isize> {
    let size = input.len();
    let mut ret: Vec<isize> = Vec::new();
    for step in 0..size {
        let pattern = get_pattern(step, size);
        let val = apply_pattern(&input, &pattern);
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
        curr = apply_phase(curr)
    }

    curr.truncate(8);
    let sol = curr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("Solution part 1: {:?}", sol);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1() {
        let p0 = get_pattern(0, 8);
        assert_eq!(p0, vec![1, 0, -1, 0, 1, 0, -1, 0]);

        let p1 = get_pattern(0, 10);
        assert_eq!(p1, vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0]);

        let p2 = get_pattern(1, 10);
        assert_eq!(p2, vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1]);

        let r1 = apply_pattern(
            &vec![1, 2, 3, 4, 5, 6, 7, 8],
            &vec![1, 0, -1, 0, 1, 0, -1, 0],
        );
        assert_eq!(r1, 4);
    }
}
