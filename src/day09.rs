use std::convert::TryFrom;

#[derive(Clone, Debug)]
struct Program {
    insts: Vec<i128>,
    curr: usize,
    base: usize,
    done: bool,
}

impl Program {
    fn init(insts: Vec<i128>) -> Program {
        Program {
            insts,
            curr: 0,
            base: 0,
            done: false,
        }
    }

    fn check(&mut self, index: usize) {
        if index >= self.insts.len() {
            self.insts.resize(index + 1, 0);
        }
    }

    fn get(&mut self, index: usize) -> i128 {
        self.check(index);
        self.insts[index]
    }

    fn get_curr(&mut self) -> i128 {
        self.check(self.curr);
        self.insts[self.curr]
    }

    fn set(&mut self, index: usize, value: i128) {
        self.check(index);
        self.insts[index] = value;
    }

    fn inc(&mut self, value: usize) {
        self.curr += value;
    }

    fn get_index(&mut self, mode: i128, n: usize) -> isize {
        let curr_val = isize::try_from(self.get(self.curr + n)).unwrap();
        if mode == 1 {
            curr_val as isize
        } else if mode == 2 {
            self.base as isize + curr_val
        } else {
            curr_val as isize
        }
    }

    fn get_param(&mut self, mode: i128, n: usize) -> i128 {
        let index = self.get_index(mode, n);
        if mode == 1 {
            index as i128
        } else {
            self.get(index as usize)
        }
    }

    fn process(&mut self, input: i128, is_setup: bool) -> i128 {
        while self.get_curr() != 99 {
            let op = self.get_curr() % 100;
            let first_mode = (self.get_curr() / 100) % 10;
            let second_mode = (self.get_curr() / 1000) % 10;
            let third_mode = (self.get_curr() / 10000) % 10;
            match op {
                1 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    let target = self.get_index(third_mode, 3) as usize;
                    self.set(target, first + second);
                    self.inc(4);
                }
                2 => {
                    let first = self.get_param(first_mode, 1);
                    let second = self.get_param(second_mode, 2);
                    let target = self.get_index(third_mode, 3) as usize;
                    self.set(target, first * second);
                    self.inc(4);
                }
                3 => {
                    println!("INPUT: {}", input);
                    let target = self.get_index(first_mode, 1) as usize;
                    self.set(target, input);
                    self.inc(2);
                    if is_setup {
                        return 0;
                    }
                }
                4 => {
                    let first = self.get_param(first_mode, 1);
                    self.inc(2);
                    println!("OUTPUT: {}", first);
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
                    let target = self.get_index(third_mode, 3) as usize;
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
                    let target = self.get_index(third_mode, 3) as usize;
                    if first == second {
                        self.set(target, 1);
                    } else {
                        self.set(target, 0);
                    }
                    self.inc(4);
                }
                9 => {
                    let target = self.get_param(first_mode, 1);
                    let new_base = self.base as isize + isize::try_from(target).unwrap();
                    self.base = new_base as usize;
                    self.inc(2);
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

pub fn part1(mut input: String) {
    input.pop();
    let insts = input
        .split(',')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut program = Program::init(insts);
    program.process(1, false);
}

pub fn part2(mut input: String) {
    input.pop();
    let insts = input
        .split(',')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut program = Program::init(insts);
    program.process(2, false);
}
