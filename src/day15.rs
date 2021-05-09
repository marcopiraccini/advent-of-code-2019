use crate::intcode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

const NORTH: i128 = 1;
const SOUTH: i128 = 2;
const WEST: i128 = 3;
const EAST: i128 = 4;
const DIRECTIONS: [i128; 4] = [NORTH, SOUTH, EAST, WEST];

#[derive(Clone, Debug)]
struct Visited {
    visited: HashSet<(isize, isize)>,
}
impl Visited {
    fn init() -> Visited {
        Visited {
            visited: HashSet::new(),
        }
    }

    fn insert(&mut self, node: &Node) {
        self.visited.insert((node.x, node.y));
    }

    fn contains(&self, node: &Node) -> bool {
        self.visited.contains(&(node.x, node.y))
    }
}

#[derive(Clone, Debug)]
struct Node {
    x: isize,
    y: isize,
    distance: usize,
    program: intcode::Program,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Node {
    fn init(x: isize, y: isize, distance: usize, program: intcode::Program) -> Node {
        Node {
            x,
            y,
            distance,
            program,
        }
    }
}

fn create_node(prev: &Node, dir: i128) -> Node {
    let copy = prev.program.clone();
    match dir {
        NORTH => Node::init(prev.x, prev.y + 1, prev.distance + 1, copy),
        WEST => Node::init(prev.x - 1, prev.y, prev.distance + 1, copy),
        EAST => Node::init(prev.x + 1, prev.y, prev.distance + 1, copy),
        _ => Node::init(prev.x, prev.y - 1, prev.distance + 1, copy),
    }
}

// Explore the from (x,y) and return a map with (x, y) => status and the node with oxygen
fn explore_map(input: String) -> (HashMap<(isize, isize), i128>, Node) {
    let mut found: HashMap<(isize, isize), i128> = HashMap::new();
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited = Visited::init();
    let mut oxygen: Option<Node> = None;
    let start = Node::init(0, 0, 0, intcode::get_program(input));
    visited.insert(&start);
    queue.push_back(start);
    while queue.len() != 0 {
        let current = queue.pop_front().unwrap();
        for dir in DIRECTIONS.iter() {
            let mut node = create_node(&current, *dir);
            if visited.contains(&node) {
                continue;
            }
            let status = node.program.process(*dir, false);
            if status != 0 {
                visited.insert(&node);
                found.insert((node.x, node.y), status);
                if status == 2 {
                    oxygen = Some(node.clone());
                }
                queue.push_back(node);
            }
        }
    }
    (found, oxygen.unwrap())
}

pub fn part1(input: String) {
    let (_, oxygen) = explore_map(input);
    println!("Solution: {:?}", oxygen.distance);
}

pub fn part2(input: String) {
    // From the oxygen, run the same algorithm until all the map is done.
    // The solution is the distance of the last node found.
    let (map, oxygen) = explore_map(input.clone());
    let program = intcode::get_program(input); // We are not using this
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited = Visited::init();
    let start = Node::init(oxygen.x, oxygen.y, 0, program);
    let mut last_node = start.clone();
    visited.insert(&start);
    queue.push_back(start);
    while queue.len() != 0 {
        let current = queue.pop_front().unwrap();
        for dir in DIRECTIONS.iter() {
            let node = create_node(&current, *dir);
            let status = map.get(&(node.x, node.y));
            if visited.contains(&node) {
                continue;
            }
            if status.is_some() && *status.unwrap() != 0 {
                last_node = node.clone();
                visited.insert(&node);
                queue.push_back(node);
            }
        }
    }
    println!("Solution {:?}", last_node.distance);
}
