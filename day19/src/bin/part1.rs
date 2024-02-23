#![feature(test)]
extern crate test;

use std::fs;
use std::path::Path;

use aoc::today::{PuzzleInput, Solution1, solve_part1};


const INPUT_FILE: &str = "input.txt";


fn main() {
    let output = read_and_solve(INPUT_FILE);
    println!("{:?}", output);
}


fn read_and_solve(file_path: impl AsRef<Path>) -> Solution1 {
    let input = fs::read_to_string(file_path).expect("Error reading input file");
    let input: PuzzleInput = input.parse().unwrap_or_else(|err| panic!("Error parsing input: {}", err));
    solve_part1(&input)
}


#[cfg(test)]
mod part1 {
    use super::*;

    const TEST_SOLUTION: Solution1 = 19114;

    #[test]
    fn test_solution() {
        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_solution(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
