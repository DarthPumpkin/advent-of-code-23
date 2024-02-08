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
/// let parsed = day9::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
