use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::str::FromStr;

use itertools::iproduct;
use ndarray::Array2;

use crate::aux::aux;


pub type Solution1 = Cost;
pub type Solution2 = u64;
type Cost = u64;


// Solution based on A* search. The heuristic is the Manhattan distance
// The nodes in the graph are the coordinates on the map, together with the orientation (horizontal or vertical) of the previous step.
// A transition from one node to another is a move of 1, 2 or 3 steps in a direction orthogonal to the previous step.
// Note that this means that each coordinate corresponds to 2 different nodes in the graph.
// We don't need to store all possible nodes explicitly, because A* will never visit some of them if the heuristic is good enough.


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    solve_for_arbitrary_len(input, &[1, 2, 3])
}

pub fn solve_part2(input: &PuzzleInput) -> Solution2 {
    solve_for_arbitrary_len(input, &[4, 5, 6, 7, 8, 9, 10])
}

fn solve_for_arbitrary_len(input: &PuzzleInput, lengths: &[usize]) -> Solution1 {
    let start_v = Node { y: 0, x: 0, prev_orientation: Orientation::Vertical };
    let start_h = Node { y: 0, x: 0, prev_orientation: Orientation::Horizontal };
    let goal = (input.map.shape()[0] - 1, input.map.shape()[1] - 1);
    let mut fringe: BinaryHeap<HeapEntry> = [
        HeapEntry { node: start_v.clone(), f_val: heuristic(&start_v, &input) },
        HeapEntry { node: start_h.clone(), f_val: heuristic(&start_h, &input) },
    ].into();
    let mut cost_so_far: HashMap<Node, Cost> = [(start_v, 0), (start_h, 0)].into();
    while let Some(HeapEntry { node, f_val: _ }) = fringe.pop() {
        if (node.y, node.x) == goal {
            return cost_so_far[&node];
        }
        let neighbors = neighbors(&node, &input, lengths.iter().copied());
        for neighbor in neighbors {
            let intermed = intermediaries((&node.y, &node.x), (&neighbor.y, &neighbor.x)).expect("Failed to find intermediaries");
            let add_cost: Cost = intermed.into_iter().map(|(y, x)| input.map[[y, x]] as Cost).sum();
            let new_cost = cost_so_far[&node] + add_cost;
            let prev_cost = cost_so_far.get(&neighbor).copied().unwrap_or(Cost::MAX);
            if new_cost < prev_cost {
                cost_so_far.insert(neighbor.clone(), new_cost);
                let f_val = new_cost + heuristic(&neighbor, &input);
                fringe.push(HeapEntry { node: neighbor.clone(), f_val });
            }
        }
    }
    panic!("No path to goal found");
}

fn heuristic(node: &Node, input: &PuzzleInput) -> Cost {
    let shape = input.map.shape();
    (shape[0] + shape[1] - (node.y + node.x)) as Cost
}

fn neighbors(node: &Node, input: &PuzzleInput, len_candidates: impl Iterator<Item = usize>) -> Vec<Node> {
    let dir_candidates = node.prev_orientation.orthogonals();
    iproduct!(len_candidates, dir_candidates).filter_map(|(new_len, new_dir)| {
        let new_y = node.y as isize + new_dir.dy() * new_len as isize;
        let new_x = node.x as isize + new_dir.dx() * new_len as isize;
        let shape: [usize; 2] = input.map.shape().try_into().unwrap();
        if let Some([new_y, new_x]) = aux::within_bounds([new_y, new_x], shape) {
            Some(Node { y: new_y, x: new_x, prev_orientation: new_dir.orientation() })
        } else { None }
    }).collect()
}

fn intermediaries((y1, x1): (&usize, &usize), (y2, x2): (&usize, &usize)) -> Result<Vec<(usize, usize)>, ()> {
    if y1 == y2 && x1 != x2 {
        let x_min = *min(x1, x2);
        let x_max = *max(x1, x2);
        let coos = (x_min..=x_max).map(|x| (*y1, x));
        if x1 > x2 {
            return Ok(coos.rev().skip(1).collect());
        }
        return Ok(coos.skip(1).collect());
    } else if x1 == x2 && y1 != y2 {
        let y_min = *y1.min(y2);
        let y_max = *y1.max(y2);
        let coos = (y_min..=y_max).map(|y| (y, *x1));
        if y1 > y2 {
            return Ok(coos.rev().skip(1).collect());
        }
        return Ok(coos.skip(1).collect());
    }
    Err(())
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
        Some(
            other.f_val.cmp(&self.f_val)
                .then(self.node.y.cmp(&other.node.y))
                .then(self.node.x.cmp(&other.node.x))
                .then(self.node.prev_orientation.cmp(&other.node.prev_orientation))
        )
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
    prev_orientation: Orientation,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn orthogonals(&self) -> [Direction; 2] {
        match self {
            Self::Vertical => [Direction::Left, Direction::Right],
            Self::Horizontal => [Direction::Up, Direction::Down],
        }
    }
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

    fn orientation(&self) -> Orientation {
        match self {
            Direction::Up | Direction::Down => Orientation::Vertical,
            Direction::Left | Direction::Right => Orientation::Horizontal,
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
