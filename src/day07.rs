use itertools::Itertools;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
struct Program {
    insts: Vec<i128>,
    curr: usize,
    done: bool,
}

impl Program {
    fn init(insts: Vec<i128>) -> Program {
        Program {
            insts,
            curr: 0,
            done: false,
        }
    }

    fn get_rel(&mut self, rel: usize) -> i128 {
        self.insts[self.curr + rel]
    }

    fn get(&mut self, index: usize) -> i128 {
        self.insts[index]
    }

    fn get_curr(&mut self) -> i128 {
        self.insts[self.curr]
    }

    fn set(&mut self, index: usize, value: i128) {
        self.insts[index] = value;
    }

    fn inc(&mut self, value: usize) {
        self.curr += value;
    }

    fn get_param(&mut self, mode: i128, n: usize) -> i128 {
        let curr_val = self.get(self.curr + n);
        if mode == 1 {
            curr_val
        } else {
            self.get(curr_val as usize)
        }
    }

    fn process(&mut self, input: i128, is_setup: bool) -> i128 {
        while self.get_curr() != 99 {
            let op = self.get_curr() % 100;
            let first_mode = (self.get_curr() / 100) % 10;
            let second_mode = (self.get_curr() / 1000) % 10;
            match op {
                1 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    let target = self.get_rel(3) as usize;
                    self.set(target, first + second);
                    self.inc(4);
                }
                2 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    let target = self.get_rel(3) as usize;
                    self.set(target, first * second);
                    self.inc(4);
                }
                3 => {
                    let target = self.get_rel(1) as usize;
                    self.set(target, input);
                    self.inc(2);
                    if is_setup {
                        return 0;
                    }
                }
                4 => {
                    let first = self.get_param(first_mode, 1);
                    self.inc(2);
                    return first;
                }
                5 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    if first != 0 {
                        self.curr = usize::try_from(second).unwrap();
                    } else {
                        self.inc(3);
                    }
                }
                6 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    if first == 0 {
                        self.curr = usize::try_from(second).unwrap();
                    } else {
                        self.inc(3);
                    }
                }
                7 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    let target = self.get_rel(3) as usize;
                    if first < second {
                        self.set(target, 1);
                    } else {
                        self.set(target, 0);
                    }
                    self.inc(4);
                }
                8 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    let target = self.get_rel(3) as usize;
                    if first == second {
                        self.set(target, 1);
                    } else {
                        self.set(target, 0);
                    }
                    self.inc(4);
                }
                // do nothing on other cases
                _ => {
                    return 0;
                }
            }
        }
        self.done = true;
        0
    }
}

fn process_perm(program: &Program, perm: Vec<i128>) -> i128 {
    let mut program0 = program.clone();
    let mut program1 = program.clone();
    let mut program2 = program.clone();
    let mut program3 = program.clone();
    let mut program4 = program.clone();
    // phase setup
    program0.process(perm[0], true);
    program1.process(perm[1], true);
    program2.process(perm[2], true);
    program3.process(perm[3], true);
    program4.process(perm[4], true);
    let res0 = program0.process(0, false);
    let res1 = program1.process(res0, false);
    let res2 = program2.process(res1, false);
    let res3 = program3.process(res2, false);
    program4.process(res3, false)
}

pub fn part1(mut input: String) {
    let phases: Vec<i128> = vec![0, 1, 2, 3, 4];
    input.pop(); // removes the trailing \n
    let insts = input
        .split(',')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();
    let program = Program::init(insts);
    // permutations creates array of references.
    // So, we need to map and collect to array of values. Since .iter() also create references
    // (to references) we need to rereference twice if we want to obtain a <Vec<Vec<i128>>
    let perms = phases
        .iter()
        .permutations(phases.len())
        .map(|v| v.iter().map(|x| **x).collect::<Vec<i128>>())
        .collect::<Vec<Vec<i128>>>();
    let mut max = 0;
    for perm in perms {
        let res = process_perm(&program, perm);
        if res > max {
            max = res;
        }
    }
    println!("Solution part 1: {}", max);
}

fn process_perm2(program: &Program, perm: Vec<i128>) -> i128 {
    let mut program0 = program.clone();
    let mut program1 = program.clone();
    let mut program2 = program.clone();
    let mut program3 = program.clone();
    let mut program4 = program.clone();
    let mut input = 0;
    program0.process(perm[0], true);
    program1.process(perm[1], true);
    program2.process(perm[2], true);
    program3.process(perm[3], true);
    program4.process(perm[4], true);
    loop {
        let res0 = program0.process(input, false);
        let res1 = program1.process(res0, false);
        let res2 = program2.process(res1, false);
        let res3 = program3.process(res2, false);
        let res4 = program4.process(res3, false);
        if program4.done {
            return input;
        }
        input = res4;
    }
}

pub fn part2(mut input: String) {
    let phases: Vec<i128> = vec![5, 6, 7, 8, 9];
    input.pop(); // removes the trailing \n
    let insts = input
        .split(',')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();
    let program = Program::init(insts);
    let perms = phases
        .iter()
        .permutations(phases.len())
        .map(|v| v.iter().map(|x| **x).collect::<Vec<i128>>())
        .collect::<Vec<Vec<i128>>>();
    let mut max = 0;
    for perm in perms {
        let res = process_perm2(&program, perm);
        if res > max {
            max = res;
        }
    }
    println!("Solution part 2: {}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1() {
        let insts = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let prog = Program::init(insts);
        let perm = vec![4, 3, 2, 1, 0];
        let res = process_perm(&prog, perm);
        assert_eq!(res, 43210);
    }

    #[test]
    fn day07_part2() {
        let insts = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let prog = Program::init(insts);
        let perm = vec![9, 8, 7, 6, 5];
        let res = process_perm2(&prog, perm);
        assert_eq!(res, 139629729);
    }
}
