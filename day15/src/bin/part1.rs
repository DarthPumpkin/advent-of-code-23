#![feature(test, ascii_char)]
extern crate test;

use core::ascii;
use std::path::Path;

use day15::{read_input, parse_input, write_output, INPUT_FILE};
use day15::PuzzleInput;


fn main() {
    let output = read_and_solve(INPUT_FILE).to_string();
    write_output(output);
}


fn read_and_solve(file_path: impl AsRef<Path>) -> u64 {
    let input = read_input(file_path).unwrap();
    let input = parse_input(&input).unwrap();
    solve(&input)
}

fn solve(input: &PuzzleInput) -> u64 {
    input.parts.iter()
    .map(|part| hash_single(part.as_slice()) as u64)
    .sum()
}

fn hash_single(string: &[ascii::Char]) -> u8 {
    let mut hash = 0u16;
    for c in string {
        hash += c.to_u8() as u16;
        hash *= 17;
        hash %= 256;
    }
    hash as u8
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u64 = 1320;
    const TEST_SOL_1: u8 = 30;
    const TEST_SOL_2: u8 = 253;

    #[test]
    fn test_part1() {
        let test_input = parse_input(&read_input("test_input.txt").unwrap()).unwrap();
        let output_1 = hash_single(&test_input.parts[0]);
        let output_2 = hash_single(&test_input.parts[1]);
        assert_eq!(output_1, TEST_SOL_1);
        assert_eq!(output_2, TEST_SOL_2);

        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
