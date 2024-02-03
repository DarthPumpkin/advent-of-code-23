use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;


pub const INPUT_FILE: &str = "input.txt";

pub fn iter_input_lines(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write_output(output: impl ToString) {
    println!("{}", output.to_string());
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

pub fn parse_input_iter(input: impl Iterator<Item = String>) -> Box<[Card]> {
    let cars = input
        .map(|x| parse_line(&x))
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
