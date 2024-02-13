#![feature(ascii_char)]

use core::ascii;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    pub parts: Vec<Vec<ascii::Char>>,
}

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split(',').map(|substr| {
            substr.as_ascii().unwrap().to_vec()
        }).collect();
        Ok(PuzzleInput { parts })
    }
}

impl std::fmt::Display for PuzzleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in &self.parts {
            let str: String = part.iter().map(|c| c.to_char()).collect();
            write!(f, "{},", str)?;
        }
        Ok(())
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
/// let parsed = day15::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
