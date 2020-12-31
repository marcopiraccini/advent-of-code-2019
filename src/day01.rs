fn fuel(n: f64) -> f64 {
    ((n / 3_f64).floor()) - 2_f64 
}

pub fn part1(input: String) {
    let masses = input.lines().map(|x| x.parse::<f64>().unwrap());   
    let sol = masses.fold(0_f64, |acc, x| acc + fuel(x));     
    println!("Solution part 1: {}", sol)
}

fn fuel2(n:f64) -> f64 {
    let mut f = fuel(n);
    let mut total_fuel = f;
    while f > 0_f64 {
        f = fuel(f);
        if f > 0_f64 { total_fuel += f }
    }
    total_fuel
}


pub fn part2(input: String) {
    let masses = input.lines().map(|x| x.parse::<f64>().unwrap());   
    let sol = masses.fold(0_f64, |acc, x| acc + fuel2(x));     
    println!("Solution part 2: {}", sol)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fuel2() {
        assert_eq!(fuel2(14_f64), 2_f64);
        assert_eq!(fuel2(1969_f64), 966_f64);
        assert_eq!(fuel2(100756_f64), 50346_f64);
    }
}
