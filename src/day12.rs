use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;

#[derive(Debug, Hash, Copy, Clone)]
struct Satellite {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

impl Satellite {
    fn init(pos: (isize, isize, isize)) -> Satellite {
        Satellite {
            x: pos.0,
            y: pos.1,
            z: pos.2,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn apply_velocity(&mut self) {
        self.apply_velocityx();
        self.apply_velocityy();
        self.apply_velocityz();
    }

    fn apply_velocityx(&mut self) {
        self.x = self.x + self.vx;
    }

    fn apply_velocityy(&mut self) {
        self.y = self.y + self.vy;
    }

    fn apply_velocityz(&mut self) {
        self.z = self.z + self.vz;
    }

    fn energy(&self) -> isize {
        let pot = self.x.abs() + self.y.abs() + self.z.abs();
        let kin = self.vx.abs() + self.vy.abs() + self.vz.abs();
        pot * kin
    }
}

fn parse_input_line(s: &str) -> (isize, isize, isize) {
    // r is to use the "raw string" feature, so no escape is necessary
    let rex = Regex::new(r"x=(\d+|-\d+)").unwrap();
    let rey = Regex::new(r"y=(\d+|-\d+)").unwrap();
    let rez = Regex::new(r"z=(\d+|-\d+)").unwrap();
    let xcap = rex.captures(s).unwrap();
    let ycap = rey.captures(s).unwrap();
    let zcap = rez.captures(s).unwrap();
    let x = &xcap[1].parse::<isize>().unwrap();
    let y = &ycap[1].parse::<isize>().unwrap();
    let z = &zcap[1].parse::<isize>().unwrap();
    (*x, *y, *z)
}

fn gravity(sats: &mut Vec<Satellite>) {
    // We cannot use iterator.combinations on the vector here, because it will copy the values (the Satellite).
    // So we get the index combinations, and apply the changes for each pari of combinations
    // let combinations: Vec<Vec<&Satellite>> = satellites.iter_mut().combinations(2).collect();
    for comb in (0..4).combinations(2) {
        let dx = sats[comb[0]].x - sats[comb[1]].x;
        let dy = sats[comb[0]].y - sats[comb[1]].y;
        let dz = sats[comb[0]].z - sats[comb[1]].z;
        if dx > 0 {
            sats[comb[0]].vx = sats[comb[0]].vx - 1;
            sats[comb[1]].vx = sats[comb[1]].vx + 1;
        }
        if dx < 0 {
            sats[comb[0]].vx = sats[comb[0]].vx + 1;
            sats[comb[1]].vx = sats[comb[1]].vx - 1;
        }
        if dy > 0 {
            sats[comb[0]].vy = sats[comb[0]].vy - 1;
            sats[comb[1]].vy = sats[comb[1]].vy + 1;
        }
        if dy < 0 {
            sats[comb[0]].vy = sats[comb[0]].vy + 1;
            sats[comb[1]].vy = sats[comb[1]].vy - 1;
        }
        if dz > 0 {
            sats[comb[0]].vz = sats[comb[0]].vz - 1;
            sats[comb[1]].vz = sats[comb[1]].vz + 1;
        }
        if dz < 0 {
            sats[comb[0]].vz = sats[comb[0]].vz + 1;
            sats[comb[1]].vz = sats[comb[1]].vz - 1;
        }
    }
}

fn velocity(satellites: &mut Vec<Satellite>) {
    for sat in satellites.iter_mut() {
        sat.apply_velocity();
    }
}

fn velocityx(satellites: &mut Vec<Satellite>) {
    for sat in satellites.iter_mut() {
        sat.apply_velocityx();
    }
}

fn velocityy(satellites: &mut Vec<Satellite>) {
    for sat in satellites.iter_mut() {
        sat.apply_velocityy();
    }
}

fn velocityz(satellites: &mut Vec<Satellite>) {
    for sat in satellites.iter_mut() {
        sat.apply_velocityz();
    }
}

pub fn part1(mut input: String) {
    input.pop(); // removes the trailing \n
    let data: Vec<(isize, isize, isize)> = input.lines().map(|x| parse_input_line(x)).collect();
    let mut sats: Vec<Satellite> = data.iter().map(|pos| Satellite::init(*pos)).collect();
    for _n in 0..1000 {
        gravity(&mut sats);
        velocity(&mut sats);
    }
    let energy = sats.iter().fold(0, |acc, s| acc + s.energy());
    println!("Solution: {:?}", energy);
}

fn gravityx(sats: &mut Vec<Satellite>) {
    for comb in (0..4).combinations(2) {
        let dx = sats[comb[0]].x - sats[comb[1]].x;
        if dx > 0 {
            sats[comb[0]].vx = sats[comb[0]].vx - 1;
            sats[comb[1]].vx = sats[comb[1]].vx + 1;
        }
        if dx < 0 {
            sats[comb[0]].vx = sats[comb[0]].vx + 1;
            sats[comb[1]].vx = sats[comb[1]].vx - 1;
        }
    }
}

fn gravityy(sats: &mut Vec<Satellite>) {
    for comb in (0..4).combinations(2) {
        let dy = sats[comb[0]].y - sats[comb[1]].y;
        if dy > 0 {
            sats[comb[0]].vy = sats[comb[0]].vy - 1;
            sats[comb[1]].vy = sats[comb[1]].vy + 1;
        }
        if dy < 0 {
            sats[comb[0]].vy = sats[comb[0]].vy + 1;
            sats[comb[1]].vy = sats[comb[1]].vy - 1;
        }
    }
}

fn gravityz(sats: &mut Vec<Satellite>) {
    for comb in (0..4).combinations(2) {
        let dz = sats[comb[0]].z - sats[comb[1]].z;
        if dz > 0 {
            sats[comb[0]].vz = sats[comb[0]].vz - 1;
            sats[comb[1]].vz = sats[comb[1]].vz + 1;
        }
        if dz < 0 {
            sats[comb[0]].vz = sats[comb[0]].vz + 1;
            sats[comb[1]].vz = sats[comb[1]].vz - 1;
        }
    }
}

pub fn part2(mut input: String) {
    input.pop(); // removes the trailing \n
    let data: Vec<(isize, isize, isize)> = input.lines().map(|x| parse_input_line(x)).collect();
    let mut sats: Vec<Satellite> = data.iter().map(|pos| Satellite::init(*pos)).collect();
    let initial = sats.clone();
    let mut nx = 0_u128;
    let mut ny = 0_u128;
    let mut nz = 0_u128;
    let mut foundx = false;
    let mut foundy = false;
    let mut foundz = false;
    while !foundx {
        nx = nx + 1;
        gravityx(&mut sats);
        velocityx(&mut sats);
        if sats[0].x == initial[0].x
            && sats[1].x == initial[1].x
            && sats[2].x == initial[2].x
            && sats[3].x == initial[3].x
            && sats[0].vx == 0
            && sats[1].vx == 0
            && sats[2].vx == 0
            && sats[3].vx == 0
        {
            foundx = true;
        }
    }

    while !foundy {
        ny = ny + 1;
        gravityy(&mut sats);
        velocityy(&mut sats);
        if sats[0].y == initial[0].y
            && sats[1].y == initial[1].y
            && sats[2].y == initial[2].y
            && sats[3].y == initial[3].y
            && sats[0].vy == 0
            && sats[1].vy == 0
            && sats[2].vy == 0
            && sats[3].vy == 0
        {
            foundy = true;
        }
    }

    while !foundz {
        nz = nz + 1;
        gravityz(&mut sats);
        velocityz(&mut sats);
        if sats[0].z == initial[0].z
            && sats[1].z == initial[1].z
            && sats[2].z == initial[2].z
            && sats[3].z == initial[3].z
            && sats[0].vz == 0
            && sats[1].vz == 0
            && sats[2].vz == 0
            && sats[3].vz == 0
        {
            foundz = true;
        }
    }

    println!("Solution: {:?}", lcm(lcm(nx, ny), nz));
}
