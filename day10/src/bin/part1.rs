#![feature(test)]
extern crate test;

use std::path::Path;

use day10::{read_input, parse_input, write_output, INPUT_FILE};
use day10::{PuzzleInput, Coordinate, find_start, transition};


fn main() {
    let output = read_and_solve(INPUT_FILE).to_string();
    write_output(output);
}


fn read_and_solve(file_path: impl AsRef<Path>) -> i64 {
    let input = read_input(file_path).unwrap();
    let input = parse_input(&input);
    solve(&input)
}

fn solve(input: &PuzzleInput) -> i64 {
    let start = find_start(&input);
    let mut current = start.clone();
    let mut previous: Option<Coordinate> = None;
    let mut steps = 0;
    while previous == None || current != start {
        let next = transition(&input, &current, previous.as_ref());
        (current, previous) = (next, Some(current));
        steps += 1;
    }
    steps / 2
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: i64 = 8;

    #[test]
    fn test_find_start() {
        let input = read_input("test_input.txt").unwrap();
        let input = parse_input(&input);
        let start = find_start(&input);
        assert_eq!(start, Coordinate { y: 2, x: 0 });
    }

    #[test]
    fn test_part1() {
        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
