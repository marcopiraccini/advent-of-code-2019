use itertools::Itertools;

fn process(mut insts: Vec<isize>, phase: isize, input: isize) -> isize {
    let mut curr: usize = 0;
    let mut phase_added = false;
    while insts[curr] != 99 {
        let op = insts[curr] % 100;
        let first_direct = ((insts[curr] / 100) % 10) == 1; 
        let second_direct = ((insts[curr] / 1000) % 10) == 1; 
        match op {
            1 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                let second = if second_direct {insts[curr + 2]} else { insts[insts[curr + 2] as usize] };
                let target = insts[curr + 3] as usize; 
                insts[target] = first + second;
                curr += 4;
            },
            2 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                let second = if second_direct {insts[curr + 2]} else { insts[insts[curr + 2] as usize] };
                let target = insts[curr + 3] as usize; 
                insts[target] = first * second;
                curr += 4;
            },
            3 => {
                let target = insts[curr + 1] as usize; 
                if !phase_added {
                    insts[target] = phase;
                    phase_added = true;
                } else {
                    insts[target] = input;
                };
                curr += 2;
            },
            4 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                return first;
            },
            5 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                let second = if second_direct {insts[curr + 2]} else { insts[insts[curr + 2] as usize] };
                if first != 0 {
                    curr = second as usize;
                } else {
                    curr += 3;
                }
            },
            6 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                let second = if second_direct {insts[curr + 2]} else { insts[insts[curr + 2] as usize] };
                if first == 0 {
                    curr = second as usize;
                } else {
                    curr += 3;
                }
            },
            7 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                let second = if second_direct {insts[curr + 2]} else { insts[insts[curr + 2] as usize] };
                let target = insts[curr + 3] as usize; 
                if first < second {
                    insts[target] = 1;
                } else {
                    insts[target] = 0;
                }
                curr += 4;
            },
            8 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                let second = if second_direct {insts[curr + 2]} else { insts[insts[curr + 2] as usize] };
                let target = insts[curr + 3] as usize; 
                if first == second {
                    insts[target] = 1;
                } else {
                    insts[target] = 0;
                }
                curr += 4;
            },
            // do nothing on other cases
            _ => { return 0 } 
        }
    }
    0
}

fn process_perm(program: Vec<isize>, perm:Vec<isize>) -> isize {
       let res0 = process(program.clone(), perm[0], 0);
       let res1 = process(program.clone(), perm[1], res0);
       let res2 = process(program.clone(), perm[2], res1);
       let res3 = process(program.clone(), perm[3], res2);
       process(program, perm[4], res3)
}

pub fn part1(mut input: String) {
    let phases: Vec<isize>= vec![0, 1, 2, 3, 4];
    input.pop(); // removes the trailing \n
    let insts = input.split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    // permutations creates array of references. 
    // So, we need to map and collect to array of values. Since .iter() also create references 
    // (to references) we need to rereference twice if we want to obtain a <Vec<Vec<isize>>
    let perms = phases.iter().permutations(phases.len())
        .map(|v| v.iter().map(|x| **x).collect::<Vec<isize>>()).collect::<Vec<Vec<isize>>>();
    let mut max = 0;
    for perm in perms  {
       let res = process_perm(insts.clone(), perm);
       if res > max { max = res; }
    }
    println!("Solution part 1: {}", max);
}


// Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):
// 3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let perm = vec![4,3,2,1,0];
        let res = process_perm(program, perm);
        assert_eq!(res, 43210);
    }
}
