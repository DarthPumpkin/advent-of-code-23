use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

pub const INPUT_FILE: &str = "input.txt";


pub fn read_input(path: impl AsRef<Path>) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_output(output: impl ToString) {
    println!("{}", output.to_string());
}

pub fn parse_input(input: &str) -> Result<PuzzleInput, <PuzzleInput as FromStr>::Err> {
    input.parse()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PuzzleInput {
    pub height: usize,
    pub columns: Vec<Column>
}

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<PuzzleInput, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let (height, width) = (lines.len(), lines[0].len());
        let mut columns: Vec<_> = (0..width).map(|_| Column { cube_positions: vec![], round_positions: vec![] }).collect();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match Entry::from_char(c) {
                    Ok(Entry::CUBE) => columns[j].cube_positions.push(i),
                    Ok(Entry::ROUND) => columns[j].round_positions.push(i),
                    Ok(Entry::GROUND) => (),
                    Err(e) => return Err(e)
                }
            }
        }
        Ok(PuzzleInput { height, columns })
    }
}

impl std::fmt::Display for PuzzleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for col in &self.columns {
                let entry = if col.cube_positions.contains(&i) {
                    Entry::CUBE
                } else if col.round_positions.contains(&i) {
                    Entry::ROUND
                } else {
                    Entry::GROUND
                };
                write!(f, "{}", entry.to_string())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Column {
    pub cube_positions: Vec<usize>,
    pub round_positions: Vec<usize>
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Entry {
    CUBE, ROUND, GROUND
}

impl Entry {
    pub fn from_char(c: char) -> Result<Entry, String> {
        match c {
            '.' => Ok(Entry::GROUND),
            '#' => Ok(Entry::CUBE),
            'O' => Ok(Entry::ROUND),
            _ => Err(format!("Invalid character: {}", c))
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::GROUND => write!(f, "."),
            Entry::CUBE => write!(f, "#"),
            Entry::ROUND => write!(f, "O")
        }
    }
}


/// Parse a whitespace-separated list of numbers
/// 
/// # Arguments
/// 
/// * `numbers` - A string containing a whitespace-separated list of numbers
/// 
/// # Returns
/// 
/// A vector of parsed numbers
/// 
/// # Example
/// 
/// ```
/// let numbers = "1 2 3 4 5";
/// let parsed = day14::parse_numbers(numbers);
/// assert_eq!(parsed, Ok(vec![1, 2, 3, 4, 5]));
/// ```
pub fn parse_numbers<T: FromStr>(numbers: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    numbers
        .split_whitespace()
        .map(|x| x.parse())
        .collect()
}
