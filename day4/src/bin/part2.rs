use day4::{Card, read_input_lines, parse_input, write_output};


fn main() {
    let input = read_input_lines();
    let input = parse_input(&input);
    let output = solve_part2(&input).to_string();
    write_output(&output);
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
