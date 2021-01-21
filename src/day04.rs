fn is_pwd(n: usize) -> bool {
    let s: String = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut has_duplicate = false;
    for i in 1..chars.len() {
        if chars[i] < chars[i - 1] {
            return false;
        }
        if chars[i] == chars[i - 1] {
            has_duplicate = true
        }
    }
    has_duplicate
}

pub fn part1(input: String) {
    let numbers = input
        .trim()
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let res = (numbers[0]..numbers[1]).fold(0, |acc, n| if is_pwd(n) { acc + 1 } else { acc });
    println!("Solution: {}", res);
}

fn _to_st(c: char, length: usize) -> String {
    (0..length).map(|_| c).collect::<String>()
}

fn to_st(n: usize, s: usize) -> String {
    match n {
        1 => _to_st('1', s),
        2 => _to_st('2', s),
        3 => _to_st('3', s),
        4 => _to_st('4', s),
        5 => _to_st('5', s),
        6 => _to_st('6', s),
        7 => _to_st('7', s),
        8 => _to_st('8', s),
        9 => _to_st('9', s),
        0 => _to_st('0', s),
        _ => String::from(""),
    }
}

fn is_pwd2(n: usize) -> bool {
    if !is_pwd(n) {
        return false;
    }

    let s: String = n.to_string();
    let mut has_duplicate = false;
    for n in 0..10 {
        let s2 = to_st(n, 2);
        let s3 = to_st(n, 3);
        if s.contains(&s2) && !s.contains(&s3) {
            has_duplicate = true
        }
    }
    has_duplicate
}

pub fn part2(input: String) {
    let numbers = input
        .trim()
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let res = (numbers[0]..numbers[1]).fold(0, |acc, n| if is_pwd2(n) { acc + 1 } else { acc });
    println!("Solution: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1() {
        assert_eq!(is_pwd(111111), true);
        assert_eq!(is_pwd(223450), false);
        assert_eq!(is_pwd(123789), false);
    }

    #[test]
    fn day04_part2() {
        // assert_eq!(is_pwd2(112233), true);
        assert_eq!(is_pwd2(123444), false);
        // assert_eq!(is_pwd2(111122), true);
    }
}
