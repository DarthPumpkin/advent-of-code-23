#![feature(test, iter_map_windows)]
extern crate test;

use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

use day14::{read_input, parse_input, write_output, INPUT_FILE};
use day14::{PuzzleInput, Column};

const N_REPEATS: u64 = 1_000_000_000;


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
    let (preamble_len, history) = until_repeat(input, cycle);
    let preamble_len = preamble_len as u64;
    let cycle_len = history.len() as u64 - preamble_len;
    let i = preamble_len + (N_REPEATS - preamble_len) % cycle_len;
    let score = score(&history[i as usize]);
    score
}

fn until_repeat<T>(start: &T, f: fn(&T) -> T) -> (usize, Box<[T]>) where T: Clone + Eq + Hash {
    let mut seen_map: HashMap<T, usize> = HashMap::new();
    let mut seen_vec: Vec<T> = Vec::new();
    let mut current = start.clone();
    seen_map.insert(start.clone(), 0);
    seen_vec.push(start.clone());
    for t in 1.. {
        current = f(&current);
        if seen_map.get(&current).is_some() {
            break;
        }
        seen_map.insert(current.clone(), t);
        seen_vec.push(current.clone());
    }
    let rep_index = seen_map.get(&current).unwrap();
    (*rep_index, seen_vec.into_boxed_slice())
}

fn cycle(grid: &PuzzleInput) -> PuzzleInput {
    let mut grid = grid.clone();
    for _ in 0..4 {
        grid = tilt(&grid);
        grid = rotate90_clockwise(&grid);
    }
    grid
}

fn tilt(grid: &PuzzleInput) -> PuzzleInput {
    let columns = grid.columns.iter().map(tilt_column).collect();
    PuzzleInput { height: grid.height, columns }
}

// Like in part1, but now we actually need to construct the tilted column
fn tilt_column(column: &Column) -> Column {
    let mut cube_iter = column.cube_positions.iter().peekable();
    let mut cur_cube = 0;
    let mut n_stacked = 0;
    let mut new_round_positions = vec![];
    for old_round_position in &column.round_positions {
        while cube_iter.peek().unwrap_or(&&usize::MAX) < &old_round_position {
            cur_cube = cube_iter.next().unwrap() + 1;
            n_stacked = 0;
        }
        let new_pos = cur_cube + n_stacked;
        new_round_positions.push(new_pos);
        n_stacked += 1;
    }
    Column {
        cube_positions: column.cube_positions.clone(),
        round_positions: new_round_positions
    }
}

fn rotate90_clockwise(grid: &PuzzleInput) -> PuzzleInput {
    let new_height = grid.columns.len();
    let new_width = grid.height;
    let mut new_columns: Vec<_> = (0..new_width).map(|_| Column { cube_positions: vec![], round_positions: vec![] }).collect();
    for (j, column) in grid.columns.iter().enumerate() {
        for &cube_pos in &column.cube_positions {
            new_columns[new_width - 1 - cube_pos].cube_positions.push(j);
        }
        for &round_pos in &column.round_positions {
            new_columns[new_width - 1 - round_pos].round_positions.push(j);
        }
    }
    PuzzleInput { height: new_height, columns: new_columns }
}

fn score(grid: &PuzzleInput) -> u64 {
    let mut score = 0;
    for column in &grid.columns {
        for &round_pos in &column.round_positions {
            score += (grid.height - round_pos) as u64;
        }
    }
    score
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u64 = 64;

    #[test]
    fn test_part1() {
        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[test]
    fn test_tilt() {
        let test_input = parse_input(&read_input("test_input.txt").unwrap()).unwrap();
        let tilted = tilt(&test_input);
        let expected = "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....\n";
        assert_eq!(tilted.to_string(), expected);
    }

    #[test]
    fn test_rotate90_clockwise() {
        let test_input = parse_input(&read_input("test_input.txt").unwrap()).unwrap();
        let rotated = rotate90_clockwise(&test_input);
        println!("{}", rotated);
    }

    #[test]
    fn test_cycle() {
        let test_input = parse_input(&read_input("test_input.txt").unwrap()).unwrap();
        let cycled = cycle(&test_input);
        let expected = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....\n";
        assert_eq!(cycled.to_string(), expected);
    }

    #[test]
    fn test_until_repeat() {
        let fun = |x: &u64| (x + 1) % 7;
        let (preamble_len, history) = until_repeat(&3, fun);
        assert_eq!(preamble_len, 0);
        assert_eq!(history.len(), 7);

        let (preamble_len, history) = until_repeat(&7, fun);
        assert_eq!(preamble_len, 1);
        assert_eq!(history.len(), 8);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
