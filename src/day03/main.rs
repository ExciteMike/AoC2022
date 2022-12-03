use itertools::Itertools;
use shared::puzzle_input;
use std::{collections::HashSet, iter::FromIterator};

fn priority(c: char) -> usize {
    let adjust = if c.is_uppercase() { 17 } else { -9 };
    (c.to_digit(36).unwrap() as i32 + adjust) as usize
}

fn score_p1(line: &str) -> usize {
    let compartment_size = line.len() / 2;
    let a = HashSet::<_>::from_iter(line[0..compartment_size].chars());
    let b = HashSet::<_>::from_iter(line[compartment_size..].chars());
    priority(*a.intersection(&b).next().unwrap())
}

fn common_badge<I: Iterator<Item = HashSet<usize>>>(chunk: I) -> usize {
    *chunk
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

pub fn main() {
    let input = puzzle_input!();
    let p1: usize = input.lines().map(score_p1).sum();
    let p2: usize = input
        .lines()
        .map(|l| l.chars().map(priority).collect::<HashSet<usize>>())
        .chunks(3)
        .into_iter()
        .map(common_badge)
        .sum();

    println!("part 1: {}\npart 2: {}", p1, p2);
}
