use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;
use shared::puzzle_input;

fn detect(input: &str, marker_len: usize) -> usize {
    input
        .chars()
        .collect_vec()
        .windows(marker_len)
        .enumerate()
        .find_map(|(i, x)| {
            if HashSet::<&char>::from_iter(x).len() == marker_len {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()
        + marker_len
}

pub fn main() {
    let input = puzzle_input!();
    println!("part 1: {}", detect(&input, 4));
    println!("part 2: {}", detect(&input, 14));
}
