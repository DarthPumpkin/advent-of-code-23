#![feature(test, slice_as_chunks)]
extern crate test;


use std::cmp::{min, max};
use std::ops::Range;
use std::path::Path;

use day5::{iter_input_lines, parse_input_iter, write_output, INPUT_FILE};
use day5::{PuzzleInput, Direction};


fn main() {
    let output = read_and_solve_part2(INPUT_FILE).to_string();
    write_output(output);
}


fn read_and_solve_part2(file_path: impl AsRef<Path>) -> u64 {
    let input = iter_input_lines(file_path)
        .unwrap()
        .flatten();
    let input = parse_input_iter(input);
    solve_part2(&input)
}

fn solve_part2(input: &PuzzleInput) -> u64 {
    let (seed_ranges, _remainder) = input.seeds.as_chunks();
    let seed_ranges: Vec<Range<u64>> = seed_ranges.iter().map(|[start, len]| 
        Range { start: *start, end: *start + *len }
    )
    .collect();
    let mut merged_ranges = RangeUnion::from(seed_ranges);
    for map in &input.maps {
        let mut unmapped = merged_ranges;
        let mut mapped = Vec::new();
        for direction in &map.directions {
            let (remainders, redirected) = unmapped.redirect(direction);
            unmapped = RangeUnion::from(remainders);
            mapped.extend(redirected);
        }
        mapped.extend(unmapped.ranges);
        merged_ranges = RangeUnion::from(mapped);
    }
    merged_ranges.ranges[0].start
}

#[derive(Debug)]
struct RangeUnion {
    /// A vector of non-overlapping ranges sorted by start
    ranges: Vec<Range<u64>>,
}

impl RangeUnion {
    /// Create a range union from a vector of possibly overlapping ranges
    fn from(mut unstructured_ranges: Vec<Range<u64>>) -> Self {
        unstructured_ranges.sort_by_key(|range| range.start);
        let merged_ranges = merge_ranges(&unstructured_ranges);
        RangeUnion { ranges: merged_ranges }
    }

    /// Redirect a range union by a direction
    /// 
    /// # Arguments
    /// 
    /// * `direction` - The direction to redirect by
    /// 
    /// # Returns
    /// 
    /// A tuple of the remainders and the redirected ranges.
    /// The remainders are a vector of non-overlapping ranges that were not redirected.
    /// The redirected ranges are a vector of non-overlapping ranges that were redirected.
    fn redirect(&self, direction: &Direction) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        let source_range = direction.source_start..(direction.source_start + direction.range);
        let mut remainders = Vec::new();
        let mut new_ranges = Vec::new();
        for range in &self.ranges {
            let intersection_result = intersect_ranges(range, &source_range);
            if let Some(l_remainder) = intersection_result.l_remainder {
                remainders.push(l_remainder);
            }
            if let Some(intersection) = intersection_result.intersection {
                let new_start = (intersection.start - direction.source_start) + direction.destination_start;
                let new_end = (intersection.end - direction.source_start) + direction.destination_start;
                let redirected = new_start..new_end;
                new_ranges.push(redirected);
            }
            if let Some(r_remainder) = intersection_result.r_remainder {
                remainders.push(r_remainder);
            }
        }
        (remainders, new_ranges)
    }
}

#[derive(Debug)]
struct RangeIntersection {
    l_remainder: Option<Range<u64>>,
    intersection: Option<Range<u64>>,
    r_remainder: Option<Range<u64>>,
}

/// Merge overlapping ranges
/// 
/// # Arguments
/// 
/// * `ranges` - A slice of ranges to merge, sorted by start
/// 
/// # Returns
/// 
/// A vector of merged ranges, sorted by start
/// 
/// # Example
/// 
/// ```
/// let ranges = [0..3, 2..5, 3..4, 6..9];
/// let merged = merge_ranges(&ranges);
/// assert_eq!(merged, [0..5, 6..9]);
/// ```
fn merge_ranges(sorted_ranges: &[Range<u64>]) -> Vec<Range<u64>> {
    sorted_ranges.iter().fold(Vec::new(), |mut merged, range| {
        if let Some(last) = merged.last_mut() {
            if last.end >= range.start {
                last.end = max(last.end, range.end);
                return merged;
            }
        }
        merged.push(range.clone());
        merged
    })
}

fn intersect_ranges(range: &Range<u64>, other: &Range<u64>) -> RangeIntersection {
    let clip_l = min(max(range.start, other.start), range.end);
    let clip_r = max(min(range.end, other.end), range.start);
    let other_clipped = clip_l..clip_r;
    let l_remainder = match range.start < other_clipped.start {
        true => Some(range.start..other_clipped.start),
        false => None,
    };
    let r_remainder = match range.end > other_clipped.end {
        true => Some(other_clipped.end..range.end),
        false => None,
    };
    let intersection = match other_clipped.is_empty() {
        true => None,
        false => Some(other_clipped),
    };
    RangeIntersection { l_remainder, intersection, r_remainder }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOLUTION: u64 = 46;

    #[test]
    fn test_part2() {
        let test_output = read_and_solve_part2("test_input.txt");
        assert_eq!(test_output, TEST_SOLUTION);
    }

    #[test]
    fn test_redirect() {
        let ranges = vec![0..3, 5..8];
        let union = RangeUnion { ranges };
        let direction = Direction { source_start: 2, destination_start: 10, range: 2 };
        let (mut remainders, mut redirected) = union.redirect(&direction);
        redirected.append(&mut remainders);
        redirected.sort_by_key(|range| range.start);
        assert_eq!(redirected, vec![0..2, 5..8, 10..11]);
    }

    #[test]
    fn test_range_union() {
        let ranges = vec![6..9, 2..5, 0..3, 3..4];
        let union = RangeUnion::from(ranges);
        assert_eq!(union.ranges, vec![0..5, 6..9]);

        let ranges: Vec<Range<u64>> = vec![46..50, 54..63, 74..77, 45..56];
        let union = RangeUnion::from(ranges);
        assert_eq!(union.ranges, vec![45..63, 74..77]);
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = [0..3, 2..5, 3..4, 3..5, 6..9];
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, [0..5, 6..9]);
    }

    #[test]
    fn test_range_intersection() {
        let range = 0..5;
        let other = 2..7;
        let intersection = intersect_ranges(&range, &other);
        assert_eq!(intersection.l_remainder, Some(0..2));
        assert_eq!(intersection.intersection, Some(2..5));
        assert_eq!(intersection.r_remainder, None);

        let range = 5..7;
        let other = 0..4;
        let intersection = intersect_ranges(&range, &other);
        assert_eq!(intersection.l_remainder, None);
        assert_eq!(intersection.intersection, None);
        assert_eq!(intersection.r_remainder, Some(5..7));

        let range = 0..5;
        let other = 6..7;
        let intersection = intersect_ranges(&range, &other);
        assert_eq!(intersection.l_remainder, Some(0..5));
        assert_eq!(intersection.intersection, None);
        assert_eq!(intersection.r_remainder, None);
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| read_and_solve_part2(INPUT_FILE));
    }
}
