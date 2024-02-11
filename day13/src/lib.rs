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

pub fn parse_input(input: &str) -> Result<PuzzleInput, <PuzzleInput as FromStr>::Err> {
    input.parse()
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub lines: Box<[PuzzleLine]>
}

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<PuzzleInput, Self::Err> {
        let problem_strings = s.split("\n\n");
        let lines: Result<Vec<_>, _> = problem_strings.map(PuzzleLine::from_str).collect();
        Ok(PuzzleInput {
            lines: lines?.into_boxed_slice()
        })
    }
}

#[derive(Clone, Debug)]
pub struct PuzzleLine {
    pub height: usize,
    pub width: usize,
    pub entries: Box<[Entry]>
}

impl FromStr for PuzzleLine {
    type Err = String;

    fn from_str(s: &str) -> Result<PuzzleLine, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut entries: Vec<Entry> = Vec::with_capacity(height * width);
        for line in lines {
            let row_entries: Result<Vec<_>, _> = line.chars().map(Entry::from_char).collect();
            entries.extend(row_entries?);
        }
        Ok(PuzzleLine {
            height,
            width,
            entries: entries.into_boxed_slice()
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    ASH, ROCK
}

impl Entry {
    pub fn from_char(c: char) -> Result<Entry, String> {
        match c {
            '.' => Ok(Entry::ASH),
            '#' => Ok(Entry::ROCK),
            _ => Err(format!("Invalid character: {}", c))
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::ASH => write!(f, "."),
            Entry::ROCK => write!(f, "#")
        }
    }
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
/// let parsed = day13::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
