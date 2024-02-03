use std::path::Path;

use day4::{Card, iter_input_lines, parse_input_iter, write_output, INPUT_FILE};


fn main() {
    let output = read_and_solve_part1(INPUT_FILE).to_string();
    write_output(output);
}


fn read_and_solve_part1(file_path: impl AsRef<Path>) -> u64 {
    let input = iter_input_lines(file_path)
        .unwrap()
        .flatten();
    let input = parse_input_iter(input);
    solve_part1(&input)
}

fn solve_part1(cards: &[Card]) -> u64 {
    cards.iter().map(Card::score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u64 = 13;

    #[test]
    fn test_part1() {
        let test_output = read_and_solve_part1("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }
}
