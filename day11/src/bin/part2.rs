#![feature(test)]
extern crate test;

use std::collections::HashSet;
use std::path::Path;

use day11::{read_input, parse_input, write_output, INPUT_FILE};
use day11::PuzzleInput;

const EMPTY_MULTIPLIER: i64 = 1_000_000;


fn main() {
    let output = read_and_solve(INPUT_FILE, EMPTY_MULTIPLIER).to_string();
    write_output(output);
}


fn read_and_solve(file_path: impl AsRef<Path>, empty_multiplier: i64) -> i64 {
    let input = read_input(file_path).unwrap();
    let input = parse_input(&input);
    solve(&input, empty_multiplier)
}

fn solve(input: &PuzzleInput, empty_multiplier: i64) -> i64 {
    let mut empty_rows: HashSet<_> = (0..input.height).collect();
    let mut empty_cols: HashSet<_> = (0..input.width).collect();
    for galaxy in input.galaxies.iter() {
        empty_rows.remove(&galaxy.y);
        empty_cols.remove(&galaxy.x);
    }
    let mut empty_rows: Vec<_> = empty_rows.into_iter().collect();
    let mut empty_cols: Vec<_> = empty_cols.into_iter().collect();
    empty_rows.sort();
    empty_cols.sort();
    let adjusted_coos: Vec<_> = input.galaxies.iter().map(|galaxy| {
        (galaxy.y as i64 + empty_rows.binary_search(&galaxy.y).unwrap_or_else(|i| i) as i64 * (empty_multiplier - 1),
         galaxy.x as i64 + empty_cols.binary_search(&galaxy.x).unwrap_or_else(|i| i) as i64 * (empty_multiplier - 1))
    }).collect();
    adjusted_coos[..(adjusted_coos.len()-1)].iter().enumerate().map(|(i, (y, x))| {
        adjusted_coos[i+1..].iter().map(|(other_y, other_x)| {
            (y - other_y).abs() + (x - other_x).abs()
        }).sum::<i64>()
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MULTIPLIER_0: i64 = 2;
    const TEST_SOLUTION_0: i64 = 374;

    const TEST_MULTIPLIER_1: i64 = 10;
    const TEST_SOLUTION_1: i64 = 1030;

    const TEST_MULTIPLIER_2: i64 = 100;
    const TEST_SOLUTION_2: i64 = 8410;

    #[test]
    fn test_part2() {
        let test_output = read_and_solve("test_input.txt", TEST_MULTIPLIER_0);
        assert_eq!(test_output, TEST_SOLUTION_0);
        
        let test_output = read_and_solve("test_input.txt", TEST_MULTIPLIER_1);
        assert_eq!(test_output, TEST_SOLUTION_1);

        let test_output = read_and_solve("test_input.txt", TEST_MULTIPLIER_2);
        assert_eq!(test_output, TEST_SOLUTION_2);
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE, EMPTY_MULTIPLIER));
    }
}
