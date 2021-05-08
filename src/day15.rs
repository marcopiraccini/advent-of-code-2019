use crate::intcode;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

const NORTH: i128 = 1;
const SOUTH: i128 = 2;
const WEST: i128 = 3;
const EAST: i128 = 4;
const DIRECTIONS: [i128; 4] = [NORTH, SOUTH, EAST, WEST];

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

pub fn part1(input: String) {
    let program = intcode::get_program(input);
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited = Visited::init();
    let start = Node::init(0, 0, 0, program);
    visited.insert(&start);
    queue.push_back(start);
    let mut found: Option<Node> = None;
    // Breadth first, for each node in the queue:
    // - pop the node
    // - get the status in every direction
    // - if a node has already been visited, wkip
    // - if status is 2, found
    // - if status is 1, add the node to the end of the queue
    while found.is_none() {
        let current = queue.pop_front().unwrap();
        for dir in DIRECTIONS.iter() {
            let mut node = create_node(&current, *dir);
            let status = node.program.process(*dir, false);
            if visited.contains(&node) {
                continue;
            }
            if status == 2 {
                found = Some(node.clone());
            }
            if status == 1 {
                visited.insert(&node);
                queue.push_back(node);
            }
        }
    }

    println!("Solution: {:?}", found.unwrap().distance);
}
