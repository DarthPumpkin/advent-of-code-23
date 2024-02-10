#![feature(test)]
extern crate test;

use std::path::Path;

use day12::{read_input, parse_input, write_output, INPUT_FILE};
use day12::{PuzzleInput, PuzzleLine, Spring};


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
    input.lines.iter().map(solve_line).sum()
}

fn solve_line(line: &PuzzleLine) -> i64 {
    let count = count_possibilities(line);
    count
}

fn count_possibilities(line: &PuzzleLine) -> i64 {
    // Return 0 if the line is impossible.
    // Return 1 if it is possible and there are no more choices left.
    // Otherwise, recurse.
    if line.group_lens.is_empty() {
        if line.springs.contains(&Spring::DAMAGED) {
            return 0;
        } else {
            return 1;
        }
        
    }
    
    let min_space = line.group_lens.iter().sum::<usize>() + line.group_lens.len() - 1;
    if line.springs.len() < min_space {
        return 0;
    }

    match line.springs.first().unwrap() {
        Spring::OK => {
            // If the first spring is OK, we can just skip it.
            let new_line = PuzzleLine {
                springs: line.springs[1..].to_vec().into_boxed_slice(),
                group_lens: line.group_lens.clone(),
            };
            let count = count_possibilities(&new_line);
            return count;
        },
        Spring::DAMAGED => {
            // If the first spring is damaged, we must match it with the first group
            let first_group_len = line.group_lens[0];
            let first_ok = (0..line.springs.len()).filter(|s| line.springs[*s] == Spring::OK).next();
            if let Some(first_ok) = first_ok {
                if first_ok < first_group_len {
                    // If the first group is too long, we can't match it with the first spring
                    return 0;
                }
            }
            if line.springs.len() > first_group_len && line.springs[first_group_len] == Spring::DAMAGED {
                // If the group is followed by a damaged spring, then we can't match it
                return 0;
            }
            // Match the first group
            let n_skip = std::cmp::min(first_group_len + 1, line.springs.len());
            let new_line = PuzzleLine {
                springs: line.springs[n_skip..].to_vec().into_boxed_slice(),
                group_lens: line.group_lens[1..].to_vec().into_boxed_slice(),
            };
            let count = count_possibilities(&new_line);
            return count;
        },
        Spring::UNKNOWN => {
            // Try both possibilities
            let mut line_1 = line.clone();
            line_1.springs[0] = Spring::OK;
            let count_1 = count_possibilities(&line_1);

            let mut line_2 = line.clone();
            line_2.springs[0] = Spring::DAMAGED;
            let count_2 = count_possibilities(&line_2);

            return count_1 + count_2;
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION_L1: i64 = 1;
    const TEST_SOLUTION_L2: i64 = 4;
    const TEST_SOLUTION_L6: i64 = 10;
    const TEST_SOLUTION: i64 = 21;

    #[test]
    fn test_part1() {
        let test_input = parse_input(&read_input("test_input.txt").unwrap());
        let output_1 = solve_line(&test_input.lines[0]);
        let output_2 = solve_line(&test_input.lines[1]);
        let output_6 = solve_line(&test_input.lines[5]);
        assert_eq!(output_1, TEST_SOLUTION_L1);
        assert_eq!(output_2, TEST_SOLUTION_L2);
        assert_eq!(output_6, TEST_SOLUTION_L6);

        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
