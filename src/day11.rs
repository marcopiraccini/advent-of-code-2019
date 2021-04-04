use crate::intcode;

#[derive(Clone, Copy, Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
struct El {
    color: i128,
    painted: bool,
}

struct Map {
    _width: usize,
    _heigth: usize,
    _maxrow: usize,
    _maxcol: usize,
    _origin: Origin,
    map: Vec<Vec<El>>,
}

struct Origin {
    x: usize,
    y: usize,
}

impl Map {
    fn init(width: usize, heigth: usize) -> Map {
        let map = vec![
            vec![
                El {
                    color: 0,
                    painted: false
                };
                width
            ];
            heigth
        ];

        Map {
            _width: width,
            _heigth: heigth,
            _maxrow: 0,
            _maxcol: 0,
            _origin: Origin {
                x: width / 2,
                y: heigth / 2,
            },
            map,
        }
    }

    fn get(&mut self, pos: Pos) -> i128 {
        let x = self._origin.x as isize + pos.x;
        let y = self._origin.y as isize + pos.y;
        self.map[x as usize][y as usize].color
    }

    fn set(&mut self, pos: Pos, val: i128) {
        let x = (self._origin.x as isize + pos.x) as usize;
        let y = (self._origin.y as isize + pos.y) as usize;
        self.map[x][y] = El {
            color: val,
            painted: true,
        };
        if x > self._maxcol {
            self._maxcol = x
        }
        if y > self._maxrow {
            self._maxrow = y
        }
    }

    fn sum(&mut self) -> i128 {
        self.map.iter().fold(0, |acc, col| {
            acc + col
                .iter()
                .fold(0, |acc2, el| if el.painted { acc2 + 1 } else { acc2 })
        })
    }

    fn print(&self) {
        let printable_map = self
            .map
            .iter()
            .map(|row| {
                let row_str = row.iter().fold(String::from(""), |mut acc, el| {
                    if el.color == 1 {
                        acc.push('#')
                    } else {
                        acc.push(' ')
                    };
                    return acc;
                });

                println!("{:?}", row_str);
                return row_str;
            })
            .collect::<Vec<String>>();

        for line in printable_map {
            println!("{:?}", line);
        }
    }
}

fn next_dir(rotation: i128, dir: Dir) -> Dir {
    if rotation == 0 {
        match dir {
            Dir::Up => return Dir::Left,
            Dir::Left => return Dir::Down,
            Dir::Down => return Dir::Right,
            Dir::Right => return Dir::Up,
        };
    } else {
        match dir {
            Dir::Up => return Dir::Right,
            Dir::Right => return Dir::Down,
            Dir::Down => return Dir::Left,
            Dir::Left => return Dir::Up,
        };
    }
}

fn next_pos(current: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::Up => Pos {
            x: current.x,
            y: current.y + 1,
        },
        Dir::Left => Pos {
            x: current.x - 1,
            y: current.y,
        },
        Dir::Right => Pos {
            x: current.x + 1,
            y: current.y,
        },
        Dir::Down => Pos {
            x: current.x,
            y: current.y - 1,
        },
    }
}

pub fn part1(input: String) {
    let mut map = Map::init(160, 160);
    let mut program = intcode::get_program(input);
    let mut dir = Dir::Up;
    let mut current_pos = Pos { x: 0, y: 0 };
    while !program.done {
        let current_input = map.get(current_pos);
        let color = program.process(current_input, false);
        if !program.done {
            let rotation = program.process(current_input, false);
            map.set(current_pos, color);
            dir = next_dir(rotation, dir);
            current_pos = next_pos(current_pos, dir);
        }
    }
    println!("Solution {:?}", map.sum());
}

pub fn part2(input: String) {
    let mut map = Map::init(100, 150);
    let mut program = intcode::get_program(input);
    let mut dir = Dir::Up;
    let mut current_pos = Pos { x: 0, y: 0 };
    map.set(current_pos, 1);
    while !program.done {
        let current_input = map.get(current_pos);
        let color = program.process(current_input, false);
        if !program.done {
            let rotation = program.process(current_input, false);
            map.set(current_pos, color);
            dir = next_dir(rotation, dir);
            current_pos = next_pos(current_pos, dir);
        }
    }
    map.print();
}
