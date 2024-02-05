#![feature(test)]
extern crate test;


use std::path::Path;

use day8::{read_input, parse_input, write_output, INPUT_FILE};
use day8::{PuzzleInput, Instruction};


// You have to see by manual inspection that each starting node leads into a cycle after 277 steps (the number of L/R instructions),
// each cycle length is of the form p * 277 where p is a prime number, each cycle contains exactly one node with a Z,
// and this node is located 277 steps before the end of the cycle. Therefore, the first time all Z-nodes are reached
// simultaneously if after P steps, where P is the product of all the primes times 277.


fn main() {
    let input = read_input(INPUT_FILE).unwrap();
    let mut input = parse_input(&input);
    input.map.sort_by_key(|(s1, _, _)| s1.to_string());
    println!("Number of instructions: {}", input.instructions.len());
    println!("Number of nodes: {}", input.map.len());
    println!();
    let (start_nodes, cycles) = find_cycles(input.clone());
    let start_names: Vec<String> = start_nodes.iter().map(|i| input.map[*i].0.clone()).collect();
    println!("Cycles:");
    for (start_name, cycle_len) in start_names.iter().zip(cycles.iter()) {
        println!("{}: {}", start_name, cycle_len);
    }
    println!();
    let cycles_prod: u64 = cycles.iter().map(|&c| c as u64).product();
    let solution = cycles_prod * input.instructions.len() as u64;
    println!("Solution:");
    write_output(solution);
}

fn read_and_solve_part2(file_path: impl AsRef<Path>) -> u64 {
    let input = read_input(file_path).unwrap();
    let input = parse_input(&input);
    solve_part2(input)
}

fn solve_part2(input: PuzzleInput) -> u64 {
    let (_, cycles) = find_cycles(input.clone());
    let cycles_prod: u64 = cycles.iter().map(|&c| c as u64).product();
    cycles_prod * input.instructions.len() as u64
}

fn find_cycles(mut input: PuzzleInput) -> (Vec<usize>, Vec<usize>) {
    input.map.sort_by_key(|(s1, _, _)| s1.to_string());
    let nodes: Vec<Node> = input.map.iter().map(|(_, l, r)| {
        Node {
            left: input.map.binary_search_by_key(l, |(s, _, _)| s.to_string()).unwrap(),
            right: input.map.binary_search_by_key(r, |(s, _, _)| s.to_string()).unwrap(),
        }
    }).collect();
    let start_nodes: Vec<usize> = (0..nodes.len()).filter(|i| input.map[*i].0.ends_with('A')).collect();
    let mut cycles: Vec<usize> = vec![];
    for start_node in start_nodes.iter() {
        let mut cycle_ends: Vec<usize> = vec![];
        let mut current_node = start_node.clone();
        loop {
            for dir in &input.instructions {
                current_node = nodes[current_node].child(dir);
            }
            cycle_ends.push(current_node);
            if cycle_ends[..cycle_ends.len()-1].contains(&current_node) {
                break;
            }
        }
        cycles.push(cycle_ends.len() - 1);
    }
    (start_nodes, cycles)
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

    const SOLUTION: u64 = 10151663816849;

    #[test]
    fn test_solve_part2() {
        let solution = read_and_solve_part2(INPUT_FILE);
        assert_eq!(solution, SOLUTION);
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| read_and_solve_part2(INPUT_FILE));
    }
}
