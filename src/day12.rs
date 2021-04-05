use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
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
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
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
