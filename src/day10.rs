use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Seen {
    coord: Coord,
    seen: usize,
}

#[derive(Clone, Debug)]
struct CoordWithAngle {
    coord: Coord,
    angle: f64,
    dist: f64,
}

fn get_angle(start: &Coord, end: &Coord) -> f64 {
    let dx = start.x as f64 - end.x as f64;
    let dy = start.y as f64 - end.y as f64;
    dy.atan2(dx)
}

fn asteroids_seen(asteroid: &Coord, coords: &[Coord]) -> usize {
    // Cnnot use  HashSet because cannot hash a float
    let mut angles = Vec::new();
    for coord in coords.iter() {
        let angle = get_angle(&asteroid, &coord);
        if !angles.contains(&angle) {
            angles.push(angle)
        }
    }
    angles.len()
}

fn get_coords(mut input: String) -> Vec<Coord> {
    input.pop(); // removes the trailing \n
    let asteroids = input.lines().collect::<Vec<&str>>();
    let mut coords: Vec<Coord> = Vec::new();
    for (y, line) in asteroids.iter().enumerate() {
        for (x, ast) in line.chars().enumerate() {
            if ast == '#' {
                coords.push(Coord { x, y })
            }
        }
    }
    coords
}

fn get_seen(coords: &Vec<Coord>) -> Vec<Seen> {
    let seen = coords
        .iter()
        .map(|coord| {
            let seen = asteroids_seen(&coord, &coords);
            Seen {
                coord: (*coord).clone(),
                seen,
            }
        })
        .collect::<Vec<Seen>>();
    seen
}

pub fn part1(input: String) {
    let coords = get_coords(input);
    let seen = get_seen(&coords);
    let max = seen.iter().max_by_key(|x| x.seen);
    println!("Solution: {:?}", max.unwrap().seen);
}

fn distance(base: &Coord, asteroid: &Coord) -> f64 {
    ((base.x as f64 - asteroid.x as f64).powf(2.0) + (base.y as f64 - asteroid.y as f64).powf(2.0))
        .sqrt()
}

pub fn part2(input: String) {
    let coords = get_coords(input);
    let seen = get_seen(&coords);
    let max = seen.iter().max_by_key(|x| x.seen);
    let base = max.unwrap().coord;

    // For each asteroid we calculate the angle and the distance from the base
    let mut ast_ang: HashMap<String, Vec<CoordWithAngle>> =
        coords.iter().fold(HashMap::new(), |mut acc, a| {
            let angle = get_angle(&base, a);
            let dist = distance(&base, a);
            let c = CoordWithAngle {
                coord: a.clone(),
                angle,
                dist,
            };
            let curr = acc.get_mut(&angle.to_string());
            if curr.is_some() {
                curr.unwrap().push(c)
            } else {
                acc.insert(angle.to_string(), vec![c]);
            }
            return acc;
        });

    // We collect all the angles and order in a descending order
    let mut angles = coords.iter().fold(Vec::new(), |mut acc, a| {
        let angle = get_angle(&base, a);
        if !acc.contains(&angle) {
            acc.push(angle);
        }
        return acc;
    });
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // angles.reverse();

    // order all the asteroids for each angle by distance
    for angle in &angles {
        let asters = ast_ang.get_mut(&angle.to_string()).unwrap();
        asters.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
        asters.reverse();
    }

    let mut shoot = 1;
    let mut index = angles.iter().position(|&r| r >= (PI / 2_f64)).unwrap();
    let mut asteroid = base;
    while shoot <= 200 {
        let asteroids = ast_ang.get_mut(&angles[index].to_string()).unwrap();
        if asteroids.len() > 0 {
            asteroid = asteroids.pop().unwrap().coord;
            shoot = shoot + 1;
        }
        index = index + 1;
        if index == angles.len() {
            index = 0;
        }
    }
    println!("Solution: {:?}", asteroid.x * 100 + asteroid.y);
}
