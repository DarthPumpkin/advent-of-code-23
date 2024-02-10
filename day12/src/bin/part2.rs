#![feature(test)]
extern crate test;

use std::collections::HashMap;
use std::path::Path;

use rayon::prelude::*;

use day12::{read_input, parse_input, write_output, INPUT_FILE};
use day12::{PuzzleInput, PuzzleLine, Spring};

const REPETITIONS: usize = 5;


fn main() {
    let output = read_and_solve(INPUT_FILE).to_string();
    write_output(output);
}


fn read_and_solve(file_path: impl AsRef<Path>) -> u64 {
    let input = read_input(file_path).unwrap();
    let input = parse_input(&input);
    solve(&input)
}

fn solve(input: &PuzzleInput) -> u64 {
    input.lines.par_iter().map(solve_line).sum()
}

fn expand(line: &PuzzleLine) -> PuzzleLine {
    let springs = [line.springs.as_ref(); REPETITIONS].join(&Spring::UNKNOWN).into();
    let group_lens = line.group_lens.repeat(REPETITIONS).into();
    PuzzleLine { springs, group_lens }
}

fn solve_line(line: &PuzzleLine) -> u64 {
    let line = expand(line);
    let automaton = Automaton::from(line.group_lens.as_ref());
    let mut state_counts: HashMap<usize, u64> = HashMap::new();
    state_counts.insert(0, 1);
    for spring in line.springs.iter() {
        match spring {
            Spring::OK | Spring::DAMAGED => {
                state_counts = automaton.transition_all(&state_counts, spring).unwrap();
            },
            Spring::UNKNOWN => {
                let state_counts_ok = automaton.transition_all(&state_counts, &Spring::OK).unwrap();
                let mut state_counts_damaged = automaton.transition_all(&state_counts, &Spring::DAMAGED).unwrap();
                for (state, count) in state_counts_ok {
                    *state_counts_damaged.entry(state).or_insert(0) += count;
                }
                state_counts = state_counts_damaged;
            },
        }
    }
    *state_counts.get(&(automaton.transitions.len() - 1)).unwrap_or(&0)
}

#[derive(Clone, Debug)]
struct Automaton {
    transitions: Vec<Transition>
}

impl Automaton {
    fn from(group_lens: &[usize]) -> Automaton {
        let mut states = vec![
            Transition {
                next_ok: Some(0),
                next_damaged: Some(1),
            }
        ];
        for &group_len in &group_lens[..group_lens.len()-1] {
            for _ in 0..group_len-1 {
                states.push(Transition {
                    next_ok: None,
                    next_damaged: Some(states.len() + 1),
                });
            }
            states.push(Transition {
                next_ok: Some(states.len() + 1),
                next_damaged: None,
            });
            states.push(Transition {
                next_ok: Some(states.len()),
                next_damaged: Some(states.len() + 1),
            });
        }
        let last_group_len = group_lens.last().unwrap();
        for _ in 0..last_group_len-1 {
            states.push(Transition {
                next_ok: None,
                next_damaged: Some(states.len() + 1),
            });
        }
        states.push(Transition {
            next_ok: Some(states.len()),
            next_damaged: None,
        });
        Automaton { transitions: states }
    }

    fn transition_all(&self, states: &HashMap<usize, u64>, symbol: &Spring) -> Result<HashMap<usize, u64>, &str> {
        let mut new_states = HashMap::new();
        for (current, count) in states {
            let next = self.transitions[*current].transition(symbol)?;
            if let Some(next) = next {
                *new_states.entry(next).or_insert(0) += count;
            }
        }
        Ok(new_states)
    }
}

#[derive(Clone, Debug)]
struct Transition {
    next_ok: Option<usize>,
    next_damaged: Option<usize>,
}

impl Transition {
    fn transition(&self, spring: &Spring) -> Result<Option<usize>, &str> {
        match spring {
            Spring::OK => Ok(self.next_ok),
            Spring::DAMAGED => Ok(self.next_damaged),
            Spring::UNKNOWN => Err("Cannot transition UNKNOWN"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION_L1: u64 = 1;
    const TEST_SOLUTION_L2: u64 = 16384;
    const TEST_SOLUTION_L6: u64 = 506250;
    const TEST_SOLUTION: u64 = 525152;

    #[test]
    fn test_part2() {
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
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
