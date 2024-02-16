use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::str::FromStr;

use ndarray::Array2;


pub type Solution1 = Cost;
pub type Solution2 = u64;
type Cost = u64;


// Solution based on A* search. The heuristic is the Manhattan distance
// The nodes in the graph are the coordinates on the map, together with the (up to) three past steps.
// Note that this means that each coordinate corresponds to (up to) 12 different nodes in the graph.
// We don't need to store all possible nodes explicitly, because A* will never visit most of them if the heuristic is good enough.


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    let start = Node { y: 0, x: 0, prev_dir: None, len: 1 };
    let goal = (input.map.shape()[0] - 1, input.map.shape()[1] - 1);
    let mut fringe: BinaryHeap<HeapEntry> = [HeapEntry { node: start.clone(), f_val: heuristic(&start, &input) }].into();
    let mut came_from: HashMap<Node, Node> = HashMap::new();
    let mut cost_so_far: HashMap<Node, Cost> = [(start, 0)].into();
    while let Some(HeapEntry { node, f_val: _ }) = fringe.pop() {
        if (node.y, node.x) == goal {
            return cost_so_far[&node];
        }
        for neighbor in neighbors(&node, input) {
            let new_cost = cost_so_far[&node] + input.map[[neighbor.y, neighbor.x]] as Cost;
            let prev_cost = cost_so_far.get(&neighbor).copied().unwrap_or(Cost::MAX);
            if new_cost < prev_cost {
                cost_so_far.insert(neighbor.clone(), new_cost);
                let f_val = new_cost + heuristic(&neighbor, &input);
                fringe.push(HeapEntry { node: neighbor.clone(), f_val });
                came_from.insert(neighbor, node.clone());
            }
        }
    }
    panic!("No path to goal found");
}

pub fn solve_part2(input: &PuzzleInput) -> Solution2 {
    todo!()
}

fn heuristic(node: &Node, input: &PuzzleInput) -> Cost {
    let shape = input.map.shape();
    (shape[0] + shape[1] - (node.y + node.x)) as Cost
}

fn neighbors(node: &Node, input: &PuzzleInput) -> Vec<Node> {
    let mut neighbors = vec![];
    for new_dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter() {
        if node.prev_dir.is_some_and(|d| d == new_dir.opposite()) { continue; }
        let new_len = if node.prev_dir.is_some_and(|d| d == new_dir) { node.len + 1 } else { 1 };
        if new_len <= 3 {
            let new_y: Result<usize, _> = (node.y as isize + new_dir.dy()).try_into();
            let new_x: Result<usize, _> = (node.x as isize + new_dir.dx()).try_into();
            if let (Ok(new_y), Ok(new_x)) = (new_y, new_x) {
                if new_y < input.map.shape()[0] && new_x < input.map.shape()[1] {
                    neighbors.push(Node { y: new_y, x: new_x, prev_dir: Some(new_dir), len: new_len });
                }
            }
        }
    }
    neighbors
}

// This bundles a node together with its f value (guess of total cost from start to goal through this node) so that the priority queue can sort nodes by f value
#[derive(Clone, Debug, PartialEq, Eq)]
struct HeapEntry {
    node: Node,
    f_val: Cost,
}

// We sort by f value, plus some arbitrary tiebreaker. Doesn't matter much as long as all ties are broken consistently.
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.f_val.cmp(&self.f_val)
            .then(self.node.y.cmp(&other.node.y))
            .then(self.node.x.cmp(&other.node.x)))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    y: usize,
    x: usize,
    prev_dir: Option<Direction>,
    len: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dy(&self) -> isize {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }

    fn dx(&self) -> isize {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    map: Array2<u8>,
}

impl FromStr for PuzzleInput {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PuzzleInput, Self::Err> {
        let vecs: Result<Vec<Vec<u8>>, _> = s.lines().map(|line| {
            line.chars().map(|c| c.to_string().parse()).collect()
        }).collect();
        let vecs = vecs.map_err(ParseError::ParseIntError)?;
        let (height, width) = (vecs.len(), vecs[0].len());
        let flat_vec = vecs.into_iter().flatten().collect();
        let map = Array2::from_shape_vec((height, width), flat_vec).map_err(ParseError::ShapeError)?;
        Ok(PuzzleInput { map })
    }
}

pub enum ParseError {
    ParseIntError(std::num::ParseIntError),
    ShapeError(ndarray::ShapeError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ParseIntError(err) => write!(f, "Error parsing integer: {}", err),
            ParseError::ShapeError(err) => write!(f, "Error creating array from shape and data: {}", err),
        }
    }
}
