#![feature(test, iter_map_windows)]
extern crate test;

use std::path::Path;

use day14::{read_input, parse_input, write_output, INPUT_FILE};
use day14::{PuzzleInput, Column};


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
    input.columns.iter().map(|col| solve_column(input.height, col)).sum()
}

fn solve_column(height: usize, column: &Column) -> u64 {
    let mut total = 0;
    // cur_cube is the index of the current cube plus 1, i.e., the first position where a round rock can be placed
    // n_rounds_on_cur_cube counts the rocks already stacked on the current cube
    let (mut cur_cube, mut n_rounds_on_cur_cube) = (0, 0);
    let mut round_iter = column.round_positions.iter().peekable();
    let mut cube_iter = column.cube_positions.iter().peekable();
    while let Some(next_round) = round_iter.peek() {
        if let Some(next_cube) = cube_iter.peek() {
            if next_cube < next_round {
                cur_cube = **next_cube + 1;
                n_rounds_on_cur_cube = 0;
                cube_iter.next();
                continue;
            }
        }
        let height = height as u64;
        let score = height - cur_cube as u64 - n_rounds_on_cur_cube;
        total += score;
        n_rounds_on_cur_cube += 1;
        round_iter.next();
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u64 = 136;
    const TEST_COL_1: u64 = 34;
    const TEST_COL_2: u64 = 27;

    #[test]
    fn test_part1() {
        let test_input = parse_input(&read_input("test_input.txt").unwrap()).unwrap();
        let output_1 = solve_column(test_input.height, &test_input.columns[0]);
        let output_2 = solve_column(test_input.height, &test_input.columns[1]);
        assert_eq!(output_1, TEST_COL_1);
        assert_eq!(output_2, TEST_COL_2);

        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
