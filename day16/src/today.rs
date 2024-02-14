use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;


pub type Solution1 = usize;
pub type Solution2 = usize;


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    let starting_cursor = Cursor { y: 0, x: 0, direction: Direction::Right };
    let starting_state = State::new(starting_cursor);
    solve_single(starting_state, &input.map)
}

pub fn solve_part2(input: &PuzzleInput) -> Solution2 {
    let candidates = [
        (Direction::Right, (0..input.map.nrows()), (0..1)),
        (Direction::Left, (0..input.map.nrows()), (input.map.ncols() - 1..input.map.ncols())),
        (Direction::Down, (0..1), (0..input.map.ncols())),
        (Direction::Up, (input.map.nrows() - 1..input.map.nrows()), (0..input.map.ncols())),
    ];
    let starting_cursors = candidates.map(|(direction, y_range, x_range)| {
        y_range.cartesian_product(x_range).map(move |(y, x)| {
            Cursor { y, x, direction: direction.clone() }
        })
    }).into_iter().flatten();
    starting_cursors.map(|cursor| {
        let starting_state = State::new(cursor);
        solve_single(starting_state, &input.map)
    }).max().unwrap()
}

fn solve_single(mut state: State, map: &ndarray::Array2<Symbol>) -> usize {
    while !state.current_cursors.is_empty() {
        step(&mut state, map);
    }
    let energized: HashSet<_> = state.history.into_iter().map(|cursor| (cursor.y, cursor.x)).collect();
    energized.len()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    map: ndarray::Array2<Symbol>,
}

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<PuzzleInput, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let symbols: Result<Vec<_>, _> = lines.join("").chars().map(Symbol::from_char).collect();
        let symbols = symbols?;
        let map = ndarray::Array2::from_shape_vec((height, width), symbols).unwrap();
        Ok(PuzzleInput { map })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    current_cursors: Vec<Cursor>,
    history: HashSet<Cursor>
}

impl State {
    fn new(starting_cursor: Cursor) -> State {
        State {
            current_cursors: vec![starting_cursor.clone()],
            history: HashSet::from([starting_cursor]),
        }
    }
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current_cursors.hash(state);
        let history_vec: Vec<Cursor> = self.history.iter().cloned().collect();
        history_vec.hash(state);
    }
}

fn step(state: &mut State, map: &ndarray::Array2<Symbol>) {
    let mut next_cursors: Vec<Cursor> = Vec::new();
    for cursor in &state.current_cursors {
        let next = advance_cursor(map, cursor);
        for n in next.iter() {
            if state.history.contains(&n) {
                continue;
            }
            state.history.insert(n.clone());
            next_cursors.push(n.clone());
        }
    }
    state.current_cursors = next_cursors;
}

fn advance_cursor(map: &ndarray::Array2<Symbol>, cursor: &Cursor) -> Vec<Cursor> {
    let (dy, dx) = cursor.direction.as_vector();
    let (y, x) = (cursor.y as isize + dy, cursor.x as isize + dx);
    let (y, x) = (usize::try_from(y), usize::try_from(x));
    let (y, x) = match (y, x) {
        (Ok(y), Ok(x)) => (y, x),
        _ => return vec![],  // out of bounds
    };
    let symbol = map.get((y, x));
    let directions = match symbol {
        None => vec![],  // out of bounds
        Some(Symbol::Empty) => vec![cursor.direction.clone()],
        Some(Symbol::SplitV) => match cursor.direction {
            Direction::Left | Direction::Right => {
                vec![
                    Direction::Up,
                    Direction::Down
                ]
            } 
            Direction::Up | Direction::Down => vec![cursor.direction.clone()],
        },
        Some(Symbol::SplitH) => match cursor.direction {
            Direction::Up | Direction::Down => {
                vec![
                    Direction::Left,
                    Direction::Right
                ]
            } 
            Direction::Left | Direction::Right => vec![cursor.direction.clone()],
        },
        Some(Symbol::MirrorUlDr) => match cursor.direction {
            Direction::Up => vec![Direction::Left],
            Direction::Right => vec![Direction::Down],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
        },
        Some(Symbol::MirrorDlUr) => match cursor.direction {
            Direction::Up => vec![Direction::Right],
            Direction::Right => vec![Direction::Up],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
        },
    };
    directions.into_iter().map(|direction| Cursor { y, x, direction }).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cursor {
    y: usize,
    x: usize,
    direction: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_vector(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Symbol {
    Empty, SplitV, SplitH, MirrorUlDr, MirrorDlUr,
}

impl Symbol {
    fn from_char(c: char) -> Result<Symbol, String> {
        match c {
            '.' => Ok(Symbol::Empty),
            '|' => Ok(Symbol::SplitV),
            '-' => Ok(Symbol::SplitH),
            '/' => Ok(Symbol::MirrorDlUr),
            '\\' => Ok(Symbol::MirrorUlDr),
            _ => Err(format!("Invalid character: {}", c)),
        }
    }
}
