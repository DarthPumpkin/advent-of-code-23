use day4::{Card, read_input_lines, parse_input, write_output};


fn main() {
    let input = read_input_lines();
    let input = parse_input(&input);
    let output = solve_part1(&input).to_string();
    write_output(&output);
}


fn solve_part1(cards: &[Card]) -> u64 {
    cards.iter().map(Card::score).sum()
}
