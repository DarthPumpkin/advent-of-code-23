use core::panic;
use std::collections::HashSet;
use std::{env, vec};
use itertools::Itertools;

use day4::{read_input_lines, write_output};


fn main() {
    let input = read_input_lines();
    let input = parse_input(&input);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments");
    }
    let output = match args[1].as_ref() {
        "1" => solve_part1(&input).to_string(),
        "2" => solve_part2(&input).to_string(),
        _ => "Invalid argument".to_string()
    };
    write_output(&output);
}


#[derive(Debug)]
struct Card {
    pub winning_numbers: Vec<i32>,
    pub own_numbers: Vec<i32>,
}

impl Card {
    fn overlap(&self) -> u32 {
        let winning_set: HashSet<_> = self.winning_numbers.iter().collect();
        let own_set: HashSet<_> = self.own_numbers.iter().collect();
        winning_set.intersection(&own_set).count().try_into().unwrap()
    }

    fn score(&self) -> u64 {
        match self.overlap() {
            0 => 0,
            n => (2 as u64).pow(n - 1)
        }
    }
}


fn parse_input(input: &[impl AsRef<str>]) -> Box<[Card]> {
    let cars = input.into_iter()
        .map(|x| parse_line(x.as_ref()))
        .collect();
    cars
}

fn parse_line(line: &str) -> Card {
    let (_prefix, numbers) = line.split(": ").collect_tuple().expect("Invalid input");
    let (winning, own) = numbers.split(" | ").collect_tuple().expect("Invalid input");
    let winning_numbers = parse_numbers(winning);
    let own_numbers = parse_numbers(own);
    Card { winning_numbers, own_numbers }
}

fn parse_numbers(numbers: &str) -> Vec<i32> {
    numbers.split_whitespace().map(|x| x.parse().expect("Invalid input")).collect()
}

fn solve_part1(input: &[Card]) -> u64 {
    input.iter().map(Card::score).sum()
}

fn solve_part2(input: &[Card]) -> u64 {
    let size = input.len();
    let overlaps: Vec<u32> = input.iter().map(Card::overlap).collect();
    let mut n_copies = vec![1; size];
    for (i, &n) in overlaps.iter().enumerate() {
        for j in 1..n+1 {
            n_copies[i + (j as usize)] += n_copies[i];
        }
    }
    n_copies.iter().sum()
}
