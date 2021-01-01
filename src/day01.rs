use std::cmp;


fn fuel(n: i32) -> i32 {
    (n / 3) - 2 
}

pub fn part1(input: String) {
    let masses = input.lines().map(|x| x.parse::<i32>().unwrap());   
    let sol = masses.fold(0, |acc, x| acc + fuel(x));     
    println!("Solution part 1: {}", sol)
}

fn fuel2(n:i32) -> i32 {
    let f = cmp::max(fuel(n), 0);
    f + if f > 0 { fuel2(f) } else { 0 }
}


pub fn part2(input: String) {
    let masses = input.lines().map(|x| x.parse::<i32>().unwrap());   
    let sol = masses.fold(0_i32, |acc, x| acc + fuel2(x));     
    println!("Solution part 2: {}", sol)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fuel2() {
        assert_eq!(fuel2(14), 2);
        assert_eq!(fuel2(1969), 966);
        assert_eq!(fuel2(100756), 50346);
    }
}
