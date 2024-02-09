#![feature(test)]
extern crate test;

use std::path::Path;

use day10::{read_input, parse_input, write_output, INPUT_FILE};
use day10::{PuzzleInput, Pipe, Coordinate, find_start, transition};


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
    let start = find_start(&input);
    let cycle = find_cycle(&input, &start);
    let mut simplified_map: Vec<Vec<Pipe>> = input.map.iter().map(|row| {
        vec![Pipe::GROUND; row.len()]
    }).collect();
    for coo in cycle.iter() {
        simplified_map[coo.y][coo.x] = input.map[coo.y][coo.x].clone();
    }
    simplified_map[start.y][start.x] = infer_start(&input, &start);
    let mut count = 0;
    for row in simplified_map.iter() {
        let mut inside = false;
        for pipe in row.iter() {
            if (*pipe == Pipe::GROUND) && inside {
                count += 1;
            } else if [Pipe::NS, Pipe::NW, Pipe::NE].contains(pipe) {
                inside = !inside;
            }
        }
    }
    count
}

fn find_cycle(input: &PuzzleInput, start: &Coordinate) -> Vec<Coordinate> {
    let mut cycle: Vec<Coordinate> = vec![start.clone()];
    loop {
        let current = cycle.last().unwrap();
        let previous = if cycle.len() >= 2 { Some(cycle[cycle.len() - 2].clone()) } else { None };
        let next = transition(&input, current, previous.as_ref());
        cycle.push(next.clone());
        if next == *start {
            return cycle;
        }
    }
}

fn infer_start(input: &PuzzleInput, start: &Coordinate) -> Pipe {
    let left = if start.x > 0 { Some(input.map[start.y][start.x - 1].clone()) } else { None };
    let left = (left == Some(Pipe::WE)) || (left == Some(Pipe::NE)) || (left == Some(Pipe::SE));
    let right = if start.x < input.map[start.y].len() - 1 { Some(input.map[start.y][start.x + 1].clone()) } else { None };
    let right = (right == Some(Pipe::WE)) || (right == Some(Pipe::NW)) || (right == Some(Pipe::SW));
    let up = if start.y > 0 { Some(input.map[start.y - 1][start.x].clone()) } else { None };
    let up = (up == Some(Pipe::NS)) || (up == Some(Pipe::SE)) || (up == Some(Pipe::SW));
    let down = if start.y < input.map.len() - 1 { Some(input.map[start.y + 1][start.x].clone()) } else { None };
    let down = (down == Some(Pipe::NS)) || (down == Some(Pipe::NE)) || (down == Some(Pipe::NW));
    if left && right { Pipe::WE }
    else if up && down { Pipe::NS }
    else if left && down { Pipe::SW }
    else if left && up { Pipe::NW }
    else if right && down { Pipe::SE }
    else if right && up { Pipe::NE }
    else { panic!("Invalid start pipe") }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION_2: i64 = 8;
    const TEST_SOLUTION_3: i64 = 10;

    #[test]
    fn test_part2() {
        let test_output = read_and_solve("test_input2.txt");
        assert_eq!(test_output, TEST_SOLUTION_2);

        let test_output = read_and_solve("test_input3.txt");
        assert_eq!(test_output, TEST_SOLUTION_3);
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
