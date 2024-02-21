use std::cmp::{min, max};
use std::iter::Step;
use std::{fmt, vec};
use std::ops::RangeInclusive;
use std::str::FromStr;


pub type Solution1 = u64;
pub type Solution2 = u64;


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    let (v_inner, v_outer, h_edges) = preprocess_edges(&input.edges);
    let mut area = input.edges.iter().map(|dig| dig.distance as Solution1).sum();
    for (x_inner, y1_inner, y2_inner) in v_inner {
        let y_range = {
            // don't count if we're on a horizontal edge. (They are already counted.)
            let y1_on_h_edge = h_edges.binary_search_by_key(&(y1_inner, x_inner), |(y, x1, _)| (*y, *x1)).is_ok();
            let y2_on_h_edge = h_edges.binary_search_by_key(&(y2_inner, x_inner), |(y, x1, _)| (*y, *x1)).is_ok();
            let (mut y1, mut y2) = (y1_inner, y2_inner);
            if y1_on_h_edge {
                y1 += 1;
            }
            if y2_on_h_edge {
                y2 -= 1;
            }
            (y1, y2)
        };
        let mut covered = RangeUnion { ranges: vec![] };
        let outer_start = v_outer.binary_search_by_key(&x_inner, |(x, _, _)| *x)
            .unwrap_or_else(|i| i);
        for (x_outer, y1, y2) in v_outer[outer_start..].iter().copied() {
            let intersection = intersect_ranges((y1, y2), y_range);
            if let Some(intersection) = intersection {
                let n_covered = covered.update(&intersection) as Solution1;
                area += n_covered * (x_outer - x_inner - 1) as Solution1;
            }
        }
    }
    area
}

pub fn solve_part2(input: &PuzzleInput) -> Solution2 {
    let translated = input.edges.iter().map(|dig| {
        let color = dig.color.as_ref().unwrap();
        let distance = u64::from_str_radix(&color[..5], 16).unwrap();
        let direction = u8::from_str_radix(&color[5..], 16).unwrap();
        let direction = match direction {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        };
        Dig {
            direction: direction,
            distance: distance,
            color: None,
        }
    }).collect();
    dbg!(&translated);
    let new_input = PuzzleInput { edges: translated };
    solve_part1(&new_input)
}

/// Find the vertical and horizontal edges and detect on which side of the map they are.
fn preprocess_edges(edges: &[Dig]) -> (Vec<(isize, isize, isize)>, Vec<(isize, isize, isize)>, Vec<(isize, isize, isize)>) {
    let coos = edges.iter().fold(vec![(0, 0)], |mut coos, dig| {
        let (y, x) = coos.last().unwrap();
        let y = y + dig.direction.dy() * dig.distance as isize;
        let x = x + dig.direction.dx() * dig.distance as isize;
        coos.push((y, x));
        coos
    });

    let mut v_edges = vec![];
    let mut v_dirs = vec![];
    let mut h_edges = vec![];
    for (dig, edge) in edges.iter().zip(coos.windows(2)) {
        match dig.direction {
            Direction::Up | Direction::Down => {
                let x = edge[0].1;
                let y1 = min(edge[0].0, edge[1].0);
                let y2 = max(edge[0].0, edge[1].0);
                v_edges.push((x, y1, y2));
                v_dirs.push(dig.direction.clone());
            }
            Direction::Left | Direction::Right => {
                let y = edge[0].0;
                let x1 = min(edge[0].1, edge[1].1);
                let x2 = max(edge[0].1, edge[1].1);
                h_edges.push((y, x1, x2));
            }
        }
    }
    let v_argsort = {
        let mut v_argsort: Vec<_> = (0..v_edges.len()).collect();
        v_argsort.sort_by_key(|&i| v_edges[i].0);
        v_argsort
    };
    v_edges = v_argsort.iter().map(|&i| v_edges[i]).collect();
    v_dirs = v_argsort.iter().map(|&i| v_dirs[i].clone()).collect();
    h_edges.sort();
    let inner_dir = v_dirs[0].clone();
    let (v_inner, v_outer) = v_edges.iter().zip(v_dirs.iter())
        .fold((vec![], vec![]), |(mut inner, mut outer), (edge, dir)| {
            if dir == &inner_dir {
                inner.push(edge.clone());
            } else {
                outer.push(edge.clone());
            }
            (inner, outer)
        }
    );
    (v_inner, v_outer, h_edges)
}

fn intersect_ranges<T: Ord>((a1, a2): (T, T), (b1, b2): (T, T)) -> Option<RangeInclusive<T>> {
    let a1 = max(a1, b1);
    let a2 = min(a2, b2);
    if a1 <= a2 {
        Some(a1..=a2)
    } else {
        None
    }
}

fn merge_ranges<T: Ord + Step>(sorted_ranges: &[RangeInclusive<T>]) -> Vec<RangeInclusive<T>> {
    sorted_ranges.iter().fold(Vec::new(), |mut merged, range| {
        if let Some(last) = merged.last_mut() {
            if *last.end() >= T::backward(range.start().clone(), 1) {
                *last = last.start().clone()..=max(last.end(), range.end()).clone();
                return merged;
            }
        }
        merged.push(range.clone());
        merged
    })
}

#[derive(Debug)]
struct RangeUnion<T: Ord> {
    /// A vector of non-overlapping ranges sorted by start
    ranges: Vec<RangeInclusive<T>>,
}

impl RangeUnion<isize> {
    fn update(&mut self, other: &RangeInclusive<isize>) -> isize {
        let old_size: isize = self.ranges.iter().map(|range| range.end() - range.start() + 1).sum();
        let i = self.ranges.binary_search_by_key(&other.start(), |range| range.start()).unwrap_or_else(|i| i);
        self.ranges.insert(i, other.clone());
        self.ranges = merge_ranges(&self.ranges);
        let new_size: isize = self.ranges.iter().map(|range| range.end() - range.start() + 1).sum();
        new_size - old_size
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    edges: Vec<Dig>,
}

impl FromStr for PuzzleInput {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<PuzzleInput, Self::Err> {
        let digs: Result<Vec<Dig>, _> = s.lines().map(|line| {
            let mut parts = line.split(' ');
            let direction = parts.next().ok_or(InputParseError::StringParseError)?;
            let direction = direction.chars().next().ok_or(InputParseError::StringParseError)?;
            let direction = Direction::from_char(direction)?;
            let distance = parts.next().ok_or(InputParseError::StringParseError)?;
            let distance = distance.parse().map_err(InputParseError::ParseIntError)?;
            let color = parts.next().map(|s| s[2..s.len() - 1].to_string());
            Ok(Dig { direction, distance, color })
        }).collect();
        Ok(PuzzleInput { edges: digs? })
    }
}

pub enum InputParseError {
    StringParseError,
    ParseIntError(std::num::ParseIntError),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Dig {
    direction: Direction,
    distance: u64,
    color: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction, InputParseError> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(InputParseError::StringParseError),
        }
    }

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
}


/////////////////////////////////////////////////////////////////////////////
// Pretty-printing
/////////////////////////////////////////////////////////////////////////////
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "U"),
            Direction::Down => write!(f, "D"),
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

impl fmt::Display for InputParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputParseError::StringParseError => write!(f, "Error parsing string"),
            InputParseError::ParseIntError(err) => write!(f, "Error parsing integer: {}", err),
        }
    }
}
