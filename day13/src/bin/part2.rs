#![feature(test)]
extern crate test;

use std::path::Path;

use itertools::iproduct;

use day13::{read_input, parse_input, write_output, INPUT_FILE};
use day13::{PuzzleInput, PuzzleLine};


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
    input.lines.iter().map(solve_line).sum()
}

fn solve_line(line: &PuzzleLine) -> u64 {
    if let Some(mirror_row) = find_mirror_row(line) {
        mirror_row as u64 * 100
    } else {
        find_mirror_col(line).unwrap() as u64
    }
}

fn find_mirror_row(line: &PuzzleLine) -> Option<usize> {
    let rows: Vec<_> = (0..line.height).map(|i| &line.entries[i * line.width..(i + 1) * line.width]).collect();
    find_smudge(rows.as_slice())
}

fn find_mirror_col(line: &PuzzleLine) -> Option<usize> {
    let transposed: Vec<_> = iproduct!(0..line.width, 0..line.height).map(|(i, j)| line.entries[j * line.width + i].clone()).collect();
    let cols: Vec<_> = (0..line.width).map(|i| &transposed[i * line.height..(i + 1) * line.height]).collect();
    find_smudge(cols.as_slice())
}

fn find_smudge<S, T>(line: &[S]) -> Option<usize> where T: Eq, S: AsRef<[T]> {
    for i in 1..line.len() {
        let l = line[..i].iter().rev();
        let r_rev = line[i..].iter();
        let mismatches: usize = l.zip(r_rev).map(|(a, b)| {
            let (a, b) = (a.as_ref().iter(), b.as_ref().iter());
            a.zip(b).filter(|(a, b)| a != b).count()
        }).sum();
        if mismatches == 1 {
            return Some(i);
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION_1: u64 = 300;
    const TEST_SOLUTION_2: u64 = 100;
    const TEST_SOLUTION: u64 = 400;

    #[test]
    fn test_find_smudge() {
        let line = [[1, 2], [1, 3], [4, 5]].as_slice();
        let output = find_smudge(line);
        assert_eq!(output, Some(1));

        let line = [
            [1, 2],
            [3, 4], 
            [5, 6],
            [5, 6],
            [2, 4],
        ].as_slice();
        let output = find_smudge(&line);
        assert_eq!(output, Some(3));
    }

    #[test]
    fn test_part1() {
        let test_input = parse_input(&read_input("test_input.txt").unwrap()).unwrap();
        let output_1 = solve_line(&test_input.lines[0]);
        let output_2 = solve_line(&test_input.lines[1]);
        assert_eq!(output_1, TEST_SOLUTION_1);
        assert_eq!(output_2, TEST_SOLUTION_2);

        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
