use std::cmp::{min, max};
use std::fmt;
use std::str::FromStr;


pub type Solution1 = u64;
pub type Solution2 = u64;


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    let coos = input.edges.iter().fold(vec![(0, 0)], |mut coos, dig| {
        let (y, x) = coos.last().unwrap();
        let y = y + dig.direction.dy() * dig.distance as isize;
        let x = x + dig.direction.dx() * dig.distance as isize;
        coos.push((y, x));
        coos
    });
    let ys = coos.iter().map(|(y, _)| *y);
    let (y_min, y_max) = ys.fold((0, 0), |(y_min, y_max), y| (min(y_min, y), max(y_max, y)));
    let xs = coos.iter().map(|(_, x)| *x);
    let x_min = xs.min().unwrap();

    let mut v_edges = vec![];
    let mut h_edges = vec![];
    for (dig, edge) in input.edges.iter().zip(coos.windows(2)) {
        match dig.direction {
            Direction::Up | Direction::Down => {
                let x = edge[0].1;
                let y1 = min(edge[0].0, edge[1].0);
                let y2 = max(edge[0].0, edge[1].0);
                v_edges.push((x, y1, y2));
            }
            Direction::Left | Direction::Right => {
                let y = edge[0].0;
                let x1 = min(edge[0].1, edge[1].1);
                let x2 = max(edge[0].1, edge[1].1);
                h_edges.push((y, x1, x2));
            }
        }
    }
    v_edges.sort();
    h_edges.sort();
    let mut area = input.edges.iter().map(|dig| dig.distance as Solution1).sum();
    dbg!(area);
    for y in y_min..y_max {
        let edges_on_y = v_edges.iter().filter(|(_, y1, y2)| {
            *y1 <= y && y <= *y2
        });
        let mut inside = false;
        let mut x = x_min;
        for edge in edges_on_y {
            let (x_next, y1, _y2) = edge;
            // don't count if we're on a horizontal edge. (They are already counted.)
            let on_h_edge = h_edges.binary_search_by_key(&(y, x), |(y, x1, _)| (*y, *x1)).is_ok();
            if inside && !on_h_edge {
                area += (x_next - x - 1) as Solution1;
            }
            x = *x_next;
            if y > *y1 {
                inside = !inside;
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
