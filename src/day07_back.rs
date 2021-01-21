use itertools::Itertools;

#[derive(Clone, Debug)]
struct Program {
    insts: Vec<isize>,
    curr: usize,
    done: bool
}

fn process(p: &mut Program, phase: isize, input: isize) -> isize {
    let mut phase_added = false;
    while p.insts[p.curr] != 99 {
        let op = p.insts[p.curr] % 100;
        let first_direct = ((p.insts[p.curr] / 100) % 10) == 1; 
        let second_direct = ((p.insts[p.curr] / 1000) % 10) == 1;
        match op {
            1 => {
                let first = if first_direct {p.insts[p.curr + 1]} else { p.insts[p.insts[p.curr + 1] as usize] };
                let second = if second_direct {p.insts[p.curr + 2]} else { p.insts[p.insts[p.curr + 2] as usize] };
                let target = p.insts[p.curr + 3] as usize; 
                p.insts[target] = first + second;
                p.curr += 4;
            },
            2 => {
                let first = if first_direct {p.insts[p.curr + 1]} else { p.insts[p.insts[p.curr + 1] as usize] };
                let second = if second_direct {p.insts[p.curr + 2]} else { p.insts[p.insts[p.curr + 2] as usize] };
                let target = p.insts[p.curr + 3] as usize; 
                p.insts[target] = first * second;
                p.curr += 4;
            },
            3 => {
                let target = p.insts[p.curr + 1] as usize; 
                if !phase_added {
                    p.insts[target] = phase;
                    phase_added = true;
                } else {
                    p.insts[target] = input;
                };
                p.curr += 2;
            },
            4 => {
                let first = if first_direct {p.insts[p.curr + 1]} else { p.insts[p.insts[p.curr + 1] as usize] };
                p.curr += 2;
                return first;
            },
            5 => {
                let first = if first_direct {p.insts[p.curr + 1]} else { p.insts[p.insts[p.curr + 1] as usize] };
                let second = if second_direct {p.insts[p.curr + 2]} else { p.insts[p.insts[p.curr + 2] as usize] };
                if first != 0 {
                    p.curr = second as usize;
                } else {
                    p.curr += 3;
                }
            },
            6 => {
                let first = if first_direct {p.insts[p.curr + 1]} else { p.insts[p.insts[p.curr + 1] as usize] };
                let second = if second_direct {p.insts[p.curr + 2]} else { p.insts[p.insts[p.curr + 2] as usize] };
                if first == 0 {
                    p.curr = second as usize;
                } else {
                    p.curr += 3;
                }
            },
            7 => {
                let first = if first_direct {p.insts[p.curr + 1]} else { p.insts[p.insts[p.curr + 1] as usize] };
                let second = if second_direct {p.insts[p.curr + 2]} else { p.insts[p.insts[p.curr + 2] as usize] };
                let target = p.insts[p.curr + 3] as usize; 
                if first < second {
                    p.insts[target] = 1;
                } else {
                    p.insts[target] = 0;
                }
                p.curr += 4;
            },
            8 => {
                let first = if first_direct {p.insts[p.curr + 1]} else { p.insts[p.insts[p.curr + 1] as usize] };
                let second = if second_direct {p.insts[p.curr + 2]} else { p.insts[p.insts[p.curr + 2] as usize] };
                let target = p.insts[p.curr + 3] as usize; 
                if first == second {
                    p.insts[target] = 1;
                } else {
                    p.insts[target] = 0;
                }
                p.curr += 4;
            },
            // do nothing on other cases
            _ => { return 0; } 
        }
    }
    p.done = true; 
    0
}

fn process_perm(program: &Program, perm:Vec<isize>) -> isize {
    let mut program0 = program.clone();
    let mut program1 = program.clone();
    let mut program2 = program.clone();
    let mut program3 = program.clone();
    let mut program4 = program.clone();
    let res0 = process(&mut program0, perm[0], 0);
    let res1 = process(&mut program1, perm[1], res0);
    let res2 = process(&mut program2, perm[2], res1);
    let res3 = process(&mut program3, perm[3], res2);
    process(&mut program4, perm[4], res3)
}

pub fn part1(mut input: String) {
    let phases: Vec<isize>= vec![0, 1, 2, 3, 4];
    input.pop(); // removes the trailing \n
    let insts = input.split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let program = Program { insts, curr: 0, done: false };
    // permutations creates array of references. 
    // So, we need to map and collect to array of values. Since .iter() also create references 
    // (to references) we need to rereference twice if we want to obtain a <Vec<Vec<isize>>
    let perms = phases.iter().permutations(phases.len())
        .map(|v| v.iter().map(|x| **x).collect::<Vec<isize>>()).collect::<Vec<Vec<isize>>>();
    let mut max = 0;
    for perm in perms  {
       let res = process_perm(&program, perm);
       if res > max { max = res; }
    }
    println!("Solution part 1: {}", max);
}

fn process_perm2(program: &Program, perm:Vec<isize>) -> isize {
       let mut program0 = program.clone();
       let mut program1 = program.clone();
       let mut program2 = program.clone();
       let mut program3 = program.clone();
       let mut program4 = program.clone();
       let mut input = 0;
       loop {
           let res0 = process(&mut program0, perm[0], input);
           let res1 = process(&mut program1, perm[1], res0);
           let res2 = process(&mut program2, perm[2], res1);
           let res3 = process(&mut program3, perm[3], res2);
           let res4 = process(&mut program4, perm[4], res3);
           if program4.done {
               println!("done!, {}, {}, {}, {}, {}", program4.done, program3.done, program2.done, program1.done, program0.done);
               return input;
           }
           input = res4;
       }
}

pub fn part2(mut input: String) {
    let phases: Vec<isize>= vec![5, 6, 7, 8, 9];
    input.pop(); // removes the trailing \n
    let insts = input.split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let program = Program { insts, curr: 0, done: false };
    let perms = phases.iter().permutations(phases.len())
        .map(|v| v.iter().map(|x| **x).collect::<Vec<isize>>()).collect::<Vec<Vec<isize>>>();
    let mut max = 0;
    for perm in perms  {
       let res = process_perm2(&program, perm);
       if res > max { max = res; }
    }
    println!("Solution part 2: {}", max);
}


// Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):
// 3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn day07_part1() {
        let insts = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let prog = Program {
            insts, done: false, curr: 0
        };
        let perm = vec![4,3,2,1,0];
        let res = process_perm(&prog, perm);
        assert_eq!(res, 43210);
    }
    #[test]
    fn day07_part2() {
        let insts = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let prog = Program {
            insts, done: false, curr: 0
        };
        let perm = vec![9,8,7,6,5];
        let res = process_perm2(&prog, perm);
        assert_eq!(res, 139629729);
    }
    
}
