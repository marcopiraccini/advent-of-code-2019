
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
                println!("output: {}", first);
                if first != 0 {return first};
                curr += 2;
            },
            // do nothing on other cases
            _ => {} 
        }
    }
    0
}


pub fn part1(mut input: String) {
    input.pop(); // removes the trailing \n
    let insts = input.split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let res = process(insts, 1); 
    println!("Solution part 1: {:?}", res)
}


//#[cfg(test)]
//mod tests {
    //// Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    //#[test]
    //fn test_day05_part1() {
        //// vec! macro: ioc.rust-lang.org/std/macro.vec.html
        //// Also, we cannot compare vectors directly, but we can compare slices:
        //let mut res = process(vec![1,0,0,0,99], 0);
        //let mut expected = vec![2,0,0,0,99];
        //assert_eq!(&res[0..res.len()], &expected[0..expected.len()]);
        
        //res =  process(vec![1,1,1,4,99,5,6,0,99], 0);
        //expected = vec![30,1,1,4,2,5,6,0,99];
        //assert_eq!(&res[0..res.len()], &expected[0..expected.len()]);
    //}
//}
