use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

pub const INPUT_FILE: &str = "input.txt";


pub fn read_input(path: impl AsRef<Path>) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_output(output: impl ToString) {
    println!("{}", output.to_string());
}

pub fn parse_input(input: &str) -> PuzzleInput {
    let map = input.lines().map(|x| {
        x.as_bytes().iter().map(|b| Pipe::from_char(b).unwrap()).collect()
    }).collect();
    PuzzleInput { map }
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub map: Vec<Vec<Pipe>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pipe {
    START, GROUND, NS, WE, NE, NW, SE, SW
}

impl Pipe {
    pub fn from_char(c: &u8) -> Option<Pipe> {
        match c {
            b'S' => Some(Pipe::START),
            b'.' => Some(Pipe::GROUND),
            b'|' => Some(Pipe::NS),
            b'-' => Some(Pipe::WE),
            b'L' => Some(Pipe::NE),
            b'J' => Some(Pipe::NW),
            b'F' => Some(Pipe::SE),
            b'7' => Some(Pipe::SW),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub y: usize,
    pub x: usize,
}



pub fn find_start(input: &PuzzleInput) -> Coordinate {
    for (y, row) in input.map.iter().enumerate() {
        if let Some(x) = row.iter().position(|p| *p == Pipe::START) {
            return Coordinate {y, x};
        }
    }
    panic!("No start found");
}

pub fn transition(input: &PuzzleInput, current: &Coordinate, previous: Option<&Coordinate>) -> Coordinate {
    let (y, x) = (current.y, current.x);
    let current_pipe = &input.map[y][x];
    match current_pipe {
        Pipe::GROUND => panic!("Unexpected ground"),
        Pipe::START => {
            if previous.is_some() {
                panic!("Start should not have previous position")
            } else {
                let (n1, _n2) = find_neighbours(input, current);
                return n1.unwrap();
            }
        }
        _ => {
            let (n1, n2) = find_neighbours(input, current);
            let (n1, n2) = (n1.unwrap(), n2.unwrap());
            if let Some(previous) = previous {
                return if n1 == *previous { n2 } else if n2 == *previous { n1 } else { panic!("Previous position not found") };
            } else {
                panic!("Previous position not found")
            }
        }
    }
}

fn find_neighbours(input: &PuzzleInput, coo: &Coordinate) -> (Option<Coordinate>, Option<Coordinate>) {
    let (y, x) = (coo.y, coo.x);
    let height = input.map.len();
    let width = input.map[0].len();
    let pipe = &input.map[y][x];
    let candidates: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let candidates = candidates.iter().filter(|(dy, dx)| {
        let (other_y, other_x) = (y as isize + dy, x as isize + dx);
        (0 <= other_y) && (other_y < height as isize) && (0 <= other_x) && (other_x < width as isize)
    });
    let candidates = candidates.filter(|(dy, dx)| {
        let (dy, dx) = (*dy, *dx);
        match pipe {
            Pipe::NS => (dx == 0) && ((dy == -1) || (dy == 1)),
            Pipe::WE => (dy == 0) && ((dx == -1) || (dx == 1)),
            Pipe::NE => match (dy, dx) { (-1, 0) | (0, 1) => true, _ => false },
            Pipe::NW => match (dy, dx) { (-1, 0) | (0, -1) => true, _ => false },
            Pipe::SE => match (dy, dx) { (1, 0) | (0, 1) => true, _ => false },
            Pipe::SW => match (dy, dx) { (1, 0) | (0, -1) => true, _ => false },
            Pipe::START => {
                let other_coo = Coordinate { y: (y as isize + dy) as usize, x: (x as isize + dx) as usize };
                let (other_n1, other_n2) = find_neighbours(input, &other_coo);
                (other_n1 == Some(coo.clone())) || (other_n2 == Some(coo.clone()))
            }
            _ => false,
        }
    });
    let mut candidates = candidates
        .map(|(dy, dx)| ((y as isize + dy) as usize, (x as isize + dx) as usize))
        .map(|(y, x)| Coordinate { y, x });
    (candidates.next(), candidates.next())
}


/// Parse a whitespace-separated list of numbers
/// 
/// # Arguments
/// 
/// * `numbers` - A string containing a whitespace-separated list of numbers
/// 
/// # Returns
/// 
/// A vector of parsed numbers
/// 
/// # Example
/// 
/// ```
/// let numbers = "1 2 3 4 5";
/// let parsed = day10::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_neighbours() {
        let input = read_input("test_input.txt").unwrap();
        let input = parse_input(&input);

        let start = Coordinate { y: 2, x: 0 };
        let neighbours = find_neighbours(&input, &start);
        let correct = [
            Some(Coordinate { y: 2, x: 1 }),
            Some(Coordinate { y: 3, x: 0 })
        ];
        assert!(correct.contains(&neighbours.0) && correct.contains(&neighbours.1));

        let start = Coordinate { y: 2, x: 1 };
        let neighbours = find_neighbours(&input, &start);
        let correct = [
            Some(Coordinate { y: 2, x: 0 }),
            Some(Coordinate { y: 1, x: 1 })
        ];
        assert!(correct.contains(&neighbours.0) && correct.contains(&neighbours.1));

        let start = Coordinate { y: 1, x: 3 };
        let neighbours = find_neighbours(&input, &start);
        let correct = [
            Some(Coordinate { y: 2, x: 3 }),
            Some(Coordinate { y: 0, x: 3 })
        ];
        assert!(correct.contains(&neighbours.0) && correct.contains(&neighbours.1));
    }
}
