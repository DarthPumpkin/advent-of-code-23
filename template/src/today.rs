use std::fmt;
use std::str::FromStr;

// use crate::aux;


pub type Solution1 = u64;
pub type Solution2 = u64;


pub fn solve_part1(input: &PuzzleInput) -> Solution1 {
    todo!()
}

pub fn solve_part2(input: &PuzzleInput) -> Solution2 {
    todo!()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    // ...
}

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<PuzzleInput, Self::Err> {
        todo!()
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Symbol {
    // ...
}

impl Symbol {
    fn from_char(c: char) -> Result<Symbol, String> {
        match c {
            _ => Err(format!("Invalid character: {}", c))
        }
    }
}


/////////////////////////////////////////////////////////////////////////////
// Pretty-printing
/////////////////////////////////////////////////////////////////////////////

impl fmt::Display for PuzzleInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => todo!()
        }
    }
}
