use std::collections::HashSet;
use std::fs;
use std::path::Path;
use itertools::Itertools;


// const INPUT_FILE: &str = "test_input.txt";
pub const INPUT_FILE: &str = "input.txt";
pub const OUTPUT_FILE: &str = "output.txt";

pub fn read_input_lines(path: impl AsRef<Path>) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Error reading file");
    input.lines().map(|x| x.to_string()).collect()
}

pub fn write_output(output: impl ToString) {
    fs::write(OUTPUT_FILE, output.to_string()).expect("Error writing file");
}



#[derive(Debug)]
pub struct Card {
    pub winning_numbers: Vec<i32>,
    pub own_numbers: Vec<i32>,
}

impl Card {
    pub fn overlap(&self) -> u32 {
        let winning_set: HashSet<_> = self.winning_numbers.iter().collect();
        let own_set: HashSet<_> = self.own_numbers.iter().collect();
        winning_set.intersection(&own_set).count().try_into().unwrap()
    }

    pub fn score(&self) -> u64 {
        match self.overlap() {
            0 => 0,
            n => (2 as u64).pow(n - 1)
        }
    }
}

pub fn parse_input(input: &[impl AsRef<str>]) -> Box<[Card]> {
    let cars = input.into_iter()
        .map(|x| parse_line(x.as_ref()))
        .collect();
    cars
}

pub fn parse_line(line: &str) -> Card {
    let (_prefix, numbers) = line.split(": ").collect_tuple().expect("Invalid input");
    let (winning, own) = numbers.split(" | ").collect_tuple().expect("Invalid input");
    let winning_numbers = parse_numbers(winning);
    let own_numbers = parse_numbers(own);
    Card { winning_numbers, own_numbers }
}

pub fn parse_numbers(numbers: &str) -> Vec<i32> {
    numbers.split_whitespace().map(|x| x.parse().expect("Invalid input")).collect()
}
