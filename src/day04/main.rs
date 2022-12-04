use itertools::Itertools;
use shared::puzzle_input;
use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

type Elf = (RangeInclusive<usize>, RangeInclusive<usize>);

fn read_elf(line: &str) -> Elf {
    line.split(",")
        .map(|s| {
            let mut i = s.split('-').map(|s| s.parse::<usize>().unwrap());
            i.next().unwrap()..=i.next().unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn p1_filter((a, b): &Elf) -> bool {
    ((a.start() <= b.start()) && (a.end() >= b.end()))
        || ((a.start() >= b.start()) && (a.end() <= b.end()))
}

fn p2_filter((a, b): &Elf) -> bool {
    let lo = max(a.start(), b.start());
    let hi = min(a.end(), b.end());
    hi >= lo
}

pub fn main() {
    let input = puzzle_input!();
    let elves = input.split('\n').map(read_elf);
    let p1 = elves.clone().filter(p1_filter).count();
    let p2 = elves.filter(p2_filter).count();
    println!("part 1: {}\npart 2: {}", p1, p2);
}
