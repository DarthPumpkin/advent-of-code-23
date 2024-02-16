#![feature(test)]
extern crate test;

use std::fs;
use std::path::Path;

use aoc::today::{PuzzleInput, Solution2, solve_part2};


const INPUT_FILE: &str = "input.txt";


fn main() {
    let output = read_and_solve(INPUT_FILE);
    println!("{:?}", output);
}


fn read_and_solve(file_path: impl AsRef<Path>) -> Solution2 {
    let input = fs::read_to_string(file_path).expect("Error reading input file");
    let input: PuzzleInput = input.parse().unwrap_or_else(|err| panic!("Error parsing input: {}", err));
    solve_part2(&input)
}


#[cfg(test)]
mod part2 {
    use super::*;

    const TEST_SOLUTION: Solution2 = 94;

    #[test]
    fn test_solution() {
        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    // const TEST_1 = todo!();
    // const TEST_2 = todo!();

    // #[test]
    // fn test_cases() {
    //     let test_input = parse_input(&read_input("test_input.txt").unwrap()).unwrap();
    //     let output_1 = solve_single(&test_input.cases[0]);
    //     let output_2 = solve_single(&test_input.cases[1]);
    //     assert_eq!(output_1, TEST_1);
    //     assert_eq!(output_2, TEST_2);
    // }

    #[bench]
    fn bench_solution(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
