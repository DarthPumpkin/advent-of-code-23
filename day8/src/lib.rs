use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub const INPUT_FILE: &str = "input.txt";

pub fn read_input(path: impl AsRef<Path>) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_output(output: impl ToString) {
    println!("{}", output.to_string());
}

#[derive(Debug)]
pub struct PuzzleInput {
    pub instructions: Vec<Instruction>,
    pub map: Vec<(String, String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Left, Right
}

pub fn parse_input(input: &str) -> PuzzleInput {
    lazy_static! {
        static ref INSTRUCTIONS_REGEX: Regex = Regex::new(r"^([L|R]+)").unwrap();
        static ref MAP_REGEX: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    }
    let Some(captures) = INSTRUCTIONS_REGEX.captures(input) else {
        panic!("Invalid input")
    };
    let instructions = captures[0].chars().map(|c| {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction")
        }
    }).collect();

    let captures = MAP_REGEX.captures_iter(input);
    let map = captures.map(|capture| {
        let (_, [s1, s2, s3]) = capture.extract();
        (s1.to_owned(), s2.to_owned(), s3.to_owned())
    }).collect();

    PuzzleInput {instructions, map}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let test_input = read_input("test_input.txt").unwrap();
        let parsed = parse_input(&test_input);
        assert_eq!(parsed.instructions, vec![Instruction::Right, Instruction::Left]);
        assert_eq!(parsed.map.len(), 7);
        assert_eq!(parsed.map[0], (
            String::from("AAA"),
            String::from("BBB"),
            String::from("CCC"),
        ));
        assert_eq!(parsed.map[6], (
            String::from("ZZZ"),
            String::from("ZZZ"),
            String::from("ZZZ"),
        ));
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
/// let parsed = day5::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
