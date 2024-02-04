use itertools::Itertools;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub const INPUT_FILE: &str = "input.txt";

pub fn iter_input_lines(
    filename: impl AsRef<Path>,
) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write_output(output: impl ToString) {
    println!("{}", output.to_string());
}

#[derive(Debug)]
pub struct PuzzleInput {
    pub seeds: Vec<u64>,
    pub maps: Vec<Map>,
}

#[derive(Debug)]
pub struct Map {
    pub directions: Vec<Direction>,
}

impl Map {
    pub fn apply(&self, source: u64) -> u64 {
        for direction in &self.directions {
            if (direction.source_start..direction.source_start + direction.range).contains(&source) {
                return direction.destination_start + (source - direction.source_start);
            }
        }
        source
    }
}

#[derive(Debug)]
pub struct Direction {
    pub source_start: u64,
    pub destination_start: u64,
    pub range: u64,
}

pub fn parse_input_iter<I>(input: I) -> PuzzleInput
where
    I: Iterator<Item = String>,
{
    let batches: Vec<Vec<String>> = input
        .batching(|it| {
            let mut lines: Vec<String> = Vec::new();
            for line in it {
                if line.is_empty() {
                    break;
                }
                lines.push(line);
            }
            match lines.is_empty() {
                true => None,
                false => Some(lines),
            }
        })
        .collect();

    let seeds_str = &batches[0][0];
    let (_prefix, numbers_str) = seeds_str.split(": ").collect_tuple().unwrap();
    let seeds = parse_numbers(numbers_str).unwrap();

    let maps = batches[1..]
        .iter()
        .map(parse_map)
        .collect();
    PuzzleInput { seeds, maps }
}

pub fn parse_map(batch: &Vec<String>) -> Map {
    let directions = batch[1..]
        .iter()
        .map(|line| {
            let numbers: Vec<u64> = parse_numbers(line).unwrap();
            let (destination_start, source_start, range) = numbers
                .into_iter()
                .collect_tuple()
                .unwrap();
            Direction {
                source_start,
                destination_start,
                range,
            }
        })
        .collect();
    Map { directions }
}

pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
