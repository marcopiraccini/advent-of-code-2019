
fn process(mut insts: Vec<isize>, input: isize) -> isize {
    let mut curr: usize = 0;
    while insts[curr] != 99 {
        let op = insts[curr] % 100;
        let first_direct = ((insts[curr] / 100) % 10) == 1; 
        let second_direct = ((insts[curr] / 1000) % 10) == 1; 
        // let third_direct = ((insts[curr] / 10000) % 10) ==1; 
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
                insts[target] = input;
                curr += 2;
            },
            4 => {
                let first = if first_direct {insts[curr + 1]} else { insts[insts[curr + 1] as usize] };
                if first != 0 {return first};
                curr += 2;
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


pub fn part1(mut input: String) {
    input.pop(); // removes the trailing \n
    let insts = input.split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let res = process(insts, 1); 
    println!("Solution part 1: {:?}", res);
}

pub fn part2(mut input: String) {
    input.pop(); // removes the trailing \n
    let insts = input.split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let res = process(insts, 5); 
    println!("Solution part 2: {:?}", res);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day05_part2() {
        // vec! macro: ioc.rust-lang.org/std/macro.vec.html
        // Also, we cannot compare vectors directly, but we can compare slices:
        let mut res = process(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0);
        assert_eq!(res, 0);
        res = process(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 1);
        assert_eq!(res, 1);
    }
}
