#![feature(test)]
extern crate test;


use std::path::Path;

use day4::{Card, iter_input_lines, parse_input_iter, write_output, INPUT_FILE};


fn main() {
    let output = read_and_solve_part2(INPUT_FILE).to_string();
    write_output(output);
}


fn read_and_solve_part2(file_path: impl AsRef<Path>) -> u64 {
    let input = iter_input_lines(file_path)
        .unwrap()
        .flatten();
    let input = parse_input_iter(input);
    solve_part2(&input)
}

fn solve_part2(cards: &[Card]) -> u64 {
    let size = cards.len();
    let overlaps: Vec<u32> = cards.iter().map(Card::overlap).collect();
    let mut n_copies = vec![1; size];
    for (i, &n) in overlaps.iter().enumerate() {
        for j in 1..n+1 {
            n_copies[i + (j as usize)] += n_copies[i];
        }
    }
    n_copies.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u64 = 30;

    #[test]
    fn test_part2() {
        let test_output = read_and_solve_part2("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| read_and_solve_part2(INPUT_FILE));
    }
}
