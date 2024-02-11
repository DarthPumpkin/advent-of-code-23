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
    let rows: Vec<_> = (0..line.height).map(|i| line.entries[i * line.width..(i + 1) * line.width].to_vec()).collect();
    find_symmetry(rows.as_slice())
}

fn find_mirror_col(line: &PuzzleLine) -> Option<usize> {
    let transposed: Vec<_> = iproduct!(0..line.width, 0..line.height).map(|(i, j)| line.entries[j * line.width + i].clone()).collect();
    let cols: Vec<_> = (0..line.width).map(|i| transposed[i * line.height..(i + 1) * line.height].to_vec()).collect();
    find_symmetry(cols.as_slice())
}

fn find_symmetry<T>(line: &[T]) -> Option<usize> where T: Eq {
    for i in 1..line.len() {
        let l = line[..i].iter().rev();
        let r_rev = line[i..].iter();
        if l.zip(r_rev).all(|(a, b)| {
            a == b
        }) {
            return Some(i)
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION_1: u64 = 5;
    const TEST_SOLUTION_2: u64 = 400;
    const TEST_SOLUTION: u64 = 405;

    #[test]
    fn test_find_symmetry() {
        let line = [1, 2, 3, 4, 4, 3];
        let output = find_symmetry(&line);
        assert_eq!(output, Some(4));

        let line = [1, 2, 2];
        let output = find_symmetry(&line);
        assert_eq!(output, Some(2));
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
