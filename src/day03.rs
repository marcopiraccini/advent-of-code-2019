use std::fmt;
use std::collections::HashSet;

enum Dir {
    Right, Left, Up, Down
}

struct Mov {
    dir: Dir, 
    n: isize
}

struct Pos {
    x: isize,
    y: isize
}

// to print it we need this trait
// see: https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Debug for Mov {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let dir = match self.dir {
            Dir::Right => "Right",
            Dir::Left => "Left", 
            Dir::Up => "Up", 
            Dir::Down => "Down",
        };
        write!(f, "- {}: {}", dir, self.n)
    }
}

fn move_to(pos: Pos, movx: isize, movy: isize, visited: &mut HashSet<(isize, isize)>) -> Pos {
    let mut curr_x = pos.x;
    let mut curr_y = pos.y;
    if movx.abs() != 0 {
        for _ in 1..movx.abs() + 1 {
            if movx > 0 { curr_x += 1; } else { curr_x -= 1; }
            visited.insert((curr_x, curr_y));
        }
    }
    if movy != 0 {
        for _ in 1..movy.abs() + 1 {
            if movy > 0 { curr_y += 1; } else { curr_y -= 1; }
            visited.insert((curr_x, curr_y));
        }
    }
    Pos {x: curr_x, y:curr_y}
}

fn apply_path(path: &[Mov]) -> HashSet<(isize, isize)> {
    let mut pos = Pos { x: 0, y: 0};
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
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

fn manhattan(pos: (isize, isize)) -> usize {
    (pos.0.abs() + pos.1.abs()) as usize
}


pub fn _part1(mut input: String) -> usize {
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

    let line1 = &lines[0];
    let line2 = &lines[1];
    let visited1 = apply_path(&line1);
    let visited2 = apply_path(&line2);
    let visited_both = visited1.intersection(&visited2);
    let dists = visited_both.map(|point| manhattan(*point)); 
    dists.min().unwrap()
}

pub fn part1(input: String) {
    let min = _part1(input); 
    println!("Solution: {:?}", min); 
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
