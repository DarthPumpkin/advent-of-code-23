use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;
use std::vec;

pub const INPUT_FILE: &str = "input.txt";


pub fn read_input(path: impl AsRef<Path>) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_output(output: impl ToString) {
    println!("{}", output.to_string());
}

pub fn parse_input(input: &str) -> PuzzleInput {
    let mut galaxies = vec![];
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Coordinate { y, x });
            }
        }
    }
    PuzzleInput {
        galaxies: galaxies.into_boxed_slice(),
        height,
        width,
    }
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub galaxies: Box<[Coordinate]>,
    pub height: usize,
    pub width: usize,
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
/// let parsed = day11::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
