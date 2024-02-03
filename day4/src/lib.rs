use std::fs;


// const INPUT_FILE: &str = "test_input.txt";
pub const INPUT_FILE: &str = "input.txt";
pub const OUTPUT_FILE: &str = "output.txt";

pub fn read_input_lines() -> Vec<String> {
    let input = fs::read_to_string(INPUT_FILE).expect("Error reading file");
    input.lines().map(|x| x.to_string()).collect()
}

pub fn write_output(output: impl ToString) {
    fs::write(OUTPUT_FILE, output.to_string()).expect("Error writing file");
}
