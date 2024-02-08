#![feature(test)]
extern crate test;

use std::path::Path;

use day10::{read_input, parse_input, write_output, INPUT_FILE};
use day10::{PuzzleInput, Pipe};


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
    let mut current = start;
    let mut previous: Option<(usize, usize)> = None;
    let mut steps = 0;
    while previous == None || current != start {
        let next = transition(&input, &current, previous.as_ref());
        (current, previous) = (next, Some(current));
        steps += 1;
    }
    steps / 2
}

fn find_start(input: &PuzzleInput) -> (usize, usize) {
    for (y, row) in input.map.iter().enumerate() {
        if let Some(x) = row.iter().position(|p| *p == Pipe::START) {
            return (y, x);
        }
    }
    panic!("No start found");
}

fn transition(input: &PuzzleInput, current: &(usize, usize), previous: Option<&(usize, usize)>) -> (usize, usize) {
    let (y, x) = *current;
    let current_pipe = &input.map[y][x];
    match current_pipe {
        Pipe::GROUND => panic!("Unexpected ground"),
        Pipe::START => {
            if previous.is_some() {
                panic!("Start should not have previous position")
            } else {
                let (n1, _n2) = find_neighbours(&input, y, x);
                return n1.unwrap();
            }
        }
        _ => {
            let (n1, n2) = find_neighbours(&input, y, x);
            let (n1, n2) = (n1.unwrap(), n2.unwrap());
            if let Some(previous) = previous {
                return if n1 == *previous { n2 } else if n2 == *previous { n1 } else { panic!("Previous position not found") };
            } else {
                panic!("Previous position not found")
            }
        }
    }
}

fn find_neighbours(input: &PuzzleInput, y: usize, x: usize) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let height = input.map.len();
    let width = input.map[0].len();
    let pipe = &input.map[y][x];
    let candidates: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let candidates = candidates.iter().filter(|(dy, dx)| {
        let (other_y, other_x) = (y as isize + dy, x as isize + dx);
        (0 <= other_y) && (other_y < height as isize) && (0 <= other_x) && (other_x < width as isize)
    });
    let candidates = candidates.filter(|(dy, dx)| {
        let (dy, dx) = (*dy, *dx);
        match pipe {
            Pipe::NS => (dx == 0) && ((dy == -1) || (dy == 1)),
            Pipe::WE => (dy == 0) && ((dx == -1) || (dx == 1)),
            Pipe::NE => match (dy, dx) { (-1, 0) | (0, 1) => true, _ => false },
            Pipe::NW => match (dy, dx) { (-1, 0) | (0, -1) => true, _ => false },
            Pipe::SE => match (dy, dx) { (1, 0) | (0, 1) => true, _ => false },
            Pipe::SW => match (dy, dx) { (1, 0) | (0, -1) => true, _ => false },
            Pipe::START => {
                let (y_other, x_other) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
                let (other_n1, other_n2) = find_neighbours(input, y_other, x_other);
                (other_n1 == Some((y, x))) || (other_n2 == Some((y, x)))
            }
            _ => false,
        }
    });
    let mut candidates = candidates.map(|(dy, dx)| ((y as isize + dy) as usize, (x as isize + dx) as usize));
    (candidates.next(), candidates.next())
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: i64 = 8;

    #[test]
    fn test_find_neighbours() {
        let input = read_input("test_input.txt").unwrap();
        let input = parse_input(&input);

        let neighbours = find_neighbours(&input, 2, 0);
        let correct = [Some((2, 1)), Some((3, 0))];
        assert!(correct.contains(&neighbours.0) && correct.contains(&neighbours.1));

        let neighbours = find_neighbours(&input, 2, 1);
        let correct = [Some((2, 0)), Some((1, 1))];
        assert!(correct.contains(&neighbours.0) && correct.contains(&neighbours.1));

        let neighbours = find_neighbours(&input, 1, 3);
        let correct = [Some((2, 3)), Some((0, 3))];
        assert!(correct.contains(&neighbours.0) && correct.contains(&neighbours.1));
    }

    #[test]
    fn test_find_start() {
        let input = read_input("test_input.txt").unwrap();
        let input = parse_input(&input);
        let start = find_start(&input);
        assert_eq!(start, (2, 0));
    }

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
