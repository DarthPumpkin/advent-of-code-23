#![feature(test)]
extern crate test;

use std::path::Path;

use day11::{read_input, parse_input, write_output, INPUT_FILE};
use day11::PuzzleInput;


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
    let mut empty_rows = vec![true; input.height];
    let mut empty_cols = vec![true; input.width];
    for galaxy in input.galaxies.iter() {
        empty_rows[galaxy.y] = false;
        empty_cols[galaxy.x] = false;
    }
    let empty_rows: Vec<_> = (0..input.height).filter(|&i| empty_rows[i]).collect();
    let empty_cols: Vec<_> = (0..input.width).filter(|&i| empty_cols[i]).collect();
    let adjusted_coos: Vec<_> = input.galaxies.iter().map(|galaxy| {
        (galaxy.y as i64 + empty_rows.binary_search(&galaxy.y).unwrap_or_else(|i| i) as i64,
         galaxy.x as i64 + empty_cols.binary_search(&galaxy.x).unwrap_or_else(|i| i) as i64)
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

    const TEST_SOLUTION: i64 = 374;

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
