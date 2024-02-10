use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;

pub const INPUT_FILE: &str = "input.txt";


pub fn read_input(path: impl AsRef<Path>) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_output(output: impl ToString) {
    println!("{}", output.to_string());
}

pub fn parse_input(input: &str) -> PuzzleInput {
    let puzzle_lines = input.lines().map(|line| {
        let mut parts = line.split(' ');
        let (part1, part2) = (parts.next().unwrap(), parts.next().unwrap());
        let springs: Vec<_> = part1.chars().map(|c| Spring::from_char(&c)).collect();
        let group_lens: Vec<_> = part2.split(',').map(|s| s.parse().unwrap()).collect();
        PuzzleLine {
            springs: springs.into_boxed_slice(),
            group_lens: group_lens.into_boxed_slice(),
        }
    });
    PuzzleInput { lines: puzzle_lines.collect_vec().into_boxed_slice() }
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub lines: Box<[PuzzleLine]>,   
}

#[derive(Clone, Debug)]
pub struct PuzzleLine {
    pub springs: Box<[Spring]>,
    pub group_lens: Box<[usize]>,
}

impl std::fmt::Display for PuzzleLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let springs = self.springs.iter().map(|s| s.to_string()).collect::<String>();
        let group_lens = self.group_lens.iter().join(",");
        write!(f, "PuzzleLine {{ springs: {}, group_lens: {} }}", springs, group_lens)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spring {
    OK, DAMAGED, UNKNOWN
}

impl Spring {
    pub fn from_char(c: &char) -> Spring {
        match c {
            '.' => Spring::OK,
            '#' => Spring::DAMAGED,
            '?' => Spring::UNKNOWN,
            _ => panic!("Invalid spring character: {}", c),
        }
    }
}

impl std::fmt::Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Spring::OK => ".",
            Spring::DAMAGED => "#",
            Spring::UNKNOWN => "?",
        };
        write!(f, "{}", s)
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
/// let parsed = day11::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
