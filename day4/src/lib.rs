use std::fs;


// const INPUT_FILE: &str = "test_input.txt";
pub const INPUT_FILE: &str = "input.txt";
pub const OUTPUT_FILE: &str = "output.txt";

pub fn read_input_lines() -> Vec<Box<str>> {
    let input = fs::read_to_string(INPUT_FILE).expect("Error reading file");
    input.trim_end().split("\n").map(|x| x.into()).collect()
}

pub fn write_output(output: impl ToString) {
    fs::write(OUTPUT_FILE, output.to_string()).expect("Error writing file");
}
