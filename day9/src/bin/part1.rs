#![feature(test)]
extern crate test;

use std::path::Path;

use day9::{read_input, parse_input, write_output, INPUT_FILE};
use day9::PuzzleInput;


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
    let histories = &input.histories;
    histories.iter().map(solve_single).sum()
}

fn solve_single(history: &Vec<i64>) -> i64 {
    let len = history.len();
    let mut pyramid: Vec<Vec<i64>> = vec![history.clone()];
    for level in 1..len {
        let diff: Vec<i64> = pyramid[level - 1].windows(2).map(|x| x[1] - x[0]).collect();
        let all_eq = diff.iter().all(|&x| x == diff[0]);
        pyramid.push(diff);
        if all_eq {
            break;
        }
    }
    pyramid.iter().map(|level| level.last().unwrap()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: i64 = 114;

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
