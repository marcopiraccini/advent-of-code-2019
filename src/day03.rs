use std::collections::{HashSet, HashMap};

enum Dir {
    Right, Left, Up, Down
}

struct Mov {
    dir: Dir, 
    n: isize
}

struct Pos {
    x: isize,
    y: isize,
    step: usize    
}

fn move_to(pos: Pos, movx: isize, movy: isize, visited: &mut HashMap<(isize, isize), usize>) -> Pos {
    let mut curr_x = pos.x;
    let mut curr_y = pos.y;
    let mut curr_step = pos.step;
    if movx.abs() != 0 {
        for _ in 1..movx.abs() + 1 {
            if movx > 0 { curr_x += 1; } else { curr_x -= 1; }
            curr_step += 1;
            visited.insert( (curr_x,curr_y), curr_step);
        }
    }
    if movy != 0 {
        for _ in 1..movy.abs() + 1 {
            if movy > 0 { curr_y += 1; } else { curr_y -= 1; }
            curr_step += 1;
            visited.insert((curr_x, curr_y), curr_step);
        }
    }
    Pos {x: curr_x, y:curr_y, step: curr_step}
}

fn apply_path(path: &[Mov]) -> HashMap<(isize, isize), usize> {
    let mut pos = Pos { x: 0, y: 0, step: 0};
    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();
    for mov in path {
        match mov.dir {
            Dir::Right => pos = move_to(pos, mov.n, 0, &mut visited),
            Dir::Left =>  pos = move_to(pos, -mov.n, 0, &mut visited),
            Dir::Up => pos = move_to(pos, 0, mov.n, &mut visited),
            Dir::Down => pos = move_to(pos, 0, -mov.n, &mut visited)
        };
    }
    visited
}

fn manhattan(x: isize, y: isize) -> usize {
    (x.abs() + y.abs()) as usize
}

fn get_intersections(lines: Vec<Vec<Mov>>) -> Vec<(isize, isize)> {
    let line1 = &lines[0];
    let line2 = &lines[1];
    let visited1 = apply_path(&line1);
    let visited2 = apply_path(&line2);

    let set1: HashSet<(isize, isize)> = visited1.keys().copied().collect(); 
    let set2: HashSet<(isize, isize)> = visited2.keys().copied().collect();
    // https://stackoverflow.com/questions/31217518/why-does-cloned-allow-this-function-to-compile
    // Basically, the iterator is on references, with cloned we get the instances
    set1.intersection(&set2).cloned().collect()
}

fn get_lines(mut input: String) -> Vec<Vec<Mov>> {
    input.pop(); // removes the trailing \n
    let lines: Vec<Vec<Mov>> = input.lines().map(|l| {
        l.split(',').map(|x| {
            // chars is an interator
            let chars: Vec<char> = x.chars().collect();
            let d = chars[0];
            let rest = &chars[1..];
            let n = rest.iter().collect::<String>().parse::<isize>().unwrap();
            let dir = match d {
                'U' => Dir::Up,
                'D' => Dir::Down, 
                'L' => Dir::Left,
                'R' => Dir::Right,
                _ => Dir::Up
            };
            Mov {dir, n}
        }).collect::<Vec<Mov>>()
    }).collect();  
    lines
}

fn _part1(input: String) -> usize {
    let lines = get_lines(input);
    let visited_both = get_intersections(lines);
    let dists = visited_both.iter().map(|p| manhattan(p.0, p.1)); 
    dists.min().unwrap()
}

pub fn part1(input: String) {
    let min = _part1(input); 
    println!("    Solution: {:?}", min); 
}

pub fn  part2(input: String) {
    let lines = get_lines(input);
    let visited1 = apply_path(&lines[0]);
    let visited2 = apply_path(&lines[1]);
    let visited_both = get_intersections(lines);
    let min = visited_both.iter().map(|v| visited1.get(&v).unwrap() + visited2.get(&v).unwrap()).min();
    println!("    Solution: {:?}", min.unwrap()); 
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day03_part1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string();
        let min = _part1(input);        
        assert_eq!(min, 159);
    }
}
