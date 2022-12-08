use itertools::Itertools;
use shared::puzzle_input;
use shared::string_windows::*;

fn detect(input: &str, marker_len: usize) -> usize {
    for (i, s) in input.windows(marker_len).enumerate() {
        if s.chars().counts().len() == marker_len {
            return i + marker_len;
        }
    }
    0
}

pub fn main() {
    let input = puzzle_input!();
    println!("part 1: {}", detect(&input, 4));
    println!("part 2: {}", detect(&input, 14));
}
