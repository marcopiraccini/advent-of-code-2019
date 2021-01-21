fn process(mut insts: Vec<usize>) -> Vec<usize> {
    let mut curr: usize = 0;
    while insts[curr] != 99 {
        match insts[curr] {
            1 => {
                let first = insts[insts[curr + 1]];
                let second = insts[insts[curr + 2]];
                let target = insts[curr + 3];
                insts[target] = first + second;
                curr += 4;
            }
            2 => {
                let first = insts[insts[curr + 1]];
                let second = insts[insts[curr + 2]];
                let target = insts[curr + 3];
                insts[target] = first * second;
                curr += 4
            }
            // do nothing on other cases
            _ => {}
        }
    }
    insts
}

pub fn part1(mut input: String) {
    input.pop(); // removes the trailing \n
    let mut insts = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    insts[1] = 12;
    insts[2] = 2;
    let res = process(insts);
    println!("Solution part 1: {:?}", res[0])
}

pub fn part2(mut input: String) {
    input.pop(); // removes the trailing \n
    let insts = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let output = 19690720;

    for first in 0..99 {
        for second in 0..99 {
            let mut mem = insts.to_vec();
            mem[1] = first;
            mem[2] = second;
            let res = process(mem);
            if res[0] == output {
                println!("{}, {}", first, second);
                let solution = 100 * first + second;
                println!("Solution part 2: {}", solution);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day02_part1() {
        // vec! macro: ioc.rust-lang.org/std/macro.vec.html
        // Also, we cannot compare vectors directly, but we can compare slices:
        let mut res = process(vec![1, 0, 0, 0, 99]);
        let mut expected = vec![2, 0, 0, 0, 99];
        assert_eq!(&res[0..res.len()], &expected[0..expected.len()]);

        res = process(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(&res[0..res.len()], &expected[0..expected.len()]);
    }
}
