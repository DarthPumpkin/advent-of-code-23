#![feature(test)]
extern crate test;


use std::path::Path;

use day8::{read_input, parse_input, write_output, INPUT_FILE};
use day8::{PuzzleInput, Instruction};


fn main() {
    let output = read_and_solve_part1(INPUT_FILE).to_string();
    write_output(output);
}


fn read_and_solve_part1(file_path: impl AsRef<Path>) -> u32 {
    let input = read_input(file_path).unwrap();
    let input = parse_input(&input);
    solve_part1(input)
}

fn solve_part1(mut input: PuzzleInput) -> u32 {
    input.map.sort_by_key(|(s1, _, _)| s1.to_string());
    let nodes: Vec<Node> = input.map.iter().map(|(_, l, r)| {
        Node {
            left: input.map.binary_search_by_key(l, |(s, _, _)| s.to_string()).unwrap(),
            right: input.map.binary_search_by_key(r, |(s, _, _)| s.to_string()).unwrap(),
        }
    }).collect();
    let target_node = input.map.binary_search_by_key(&"ZZZ", |(s, _, _)| s).unwrap();
    let mut steps: u32 = 0;
    let mut current_node = 0;
    'outer: loop {
        for dir in &input.instructions {
            steps += 1;
            current_node = nodes[current_node].child(dir);
            if current_node == target_node {
                break 'outer;
            }
        }
    }
    steps
}

struct Node {
    left: usize,
    right: usize,
}

impl Node {
    fn child(&self, dir: &Instruction) -> usize {
        match dir {
            Instruction::Left => self.left,
            Instruction::Right => self.right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u32 = 2;

    #[test]
    fn test_part1() {
        let test_output = read_and_solve_part1("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| read_and_solve_part1(INPUT_FILE));
    }
}
