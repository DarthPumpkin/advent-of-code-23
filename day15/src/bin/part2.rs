#![feature(test, ascii_char)]
extern crate test;

use core::ascii;
use std::array::from_fn;
use std::path::Path;

use day15::{read_input, parse_input, write_output, INPUT_FILE};
use day15::PuzzleInput;


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
    let mut hashmap = Hashmap { boxes: from_fn(|_| vec![]) };
    for part in &input.parts {
        let instruction = Instruction::from_ascii(part).unwrap();
        hashmap.perform(&instruction);
    }
    let mut score = 0;
    for (box_index, r#box) in hashmap.boxes.iter().enumerate() {
        for (slot_index, lens) in r#box.iter().enumerate() {
            score += focusing_power(box_index as u8, slot_index, lens.focal_length);
        }
    }
    score
}

fn hash_label(string: &[ascii::Char]) -> u8 {
    let mut hash = 0u16;
    for c in string {
        hash += c.to_u8() as u16;
        hash *= 17;
        hash %= 256;
    }
    hash as u8
}

fn focusing_power(box_index: u8, slot_index: usize, focal_length: u8) -> u64 {
    (box_index as u64 + 1) * (slot_index as u64 + 1) * focal_length as u64
}

#[derive(Clone, Debug)]
struct Hashmap {
    boxes: [Vec<Lens>; 256],
}

impl Hashmap {
    fn perform(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::SET(lens) => {
                let hash = hash_label(&lens.label);
                let r#box = &mut self.boxes[hash as usize];
                let index = r#box.iter().position(|l| l.label == lens.label);
                match index {
                    Some(index) => { r#box[index] = lens.clone(); },
                    None => { r#box.push(lens.clone()); },
                }
            },
            Instruction::REMOVE(label) => {
                let hash = hash_label(&label);
                let index = self.boxes[hash as usize].iter().position(|l| l.label == *label);
                if let Some(index) = index {
                    self.boxes[hash as usize].remove(index);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lens {
    focal_length: u8,
    label: Box<[ascii::Char]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
    SET(Lens),
    REMOVE(Box<[ascii::Char]>),
}

impl Instruction {
    fn from_ascii(slice: &[ascii::Char]) -> Option<Self> {
        let eq_ascii = '='.as_ascii().unwrap();
        let dash_ascii = '-'.as_ascii().unwrap();
        let instruction_chars: [ascii::Char; 2] = [eq_ascii, dash_ascii];
        let parts: Vec<_> = slice.split(|c| instruction_chars.contains(c)).collect();
        let label = parts[0].to_vec().into_boxed_slice();
        let instruction = match slice[label.len()].to_char() {
            '=' => {
                let focal_length: u8 = parts[1].as_str().parse().unwrap();
                Instruction::SET(Lens { focal_length, label })
            }
            '-' => Instruction::REMOVE(label),
            _ => return None,
        };
        Some(instruction)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u64 = 145;

    #[test]
    fn test_part2() {
        let test_output = read_and_solve("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| read_and_solve(INPUT_FILE));
    }
}
