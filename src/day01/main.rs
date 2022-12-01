use itertools::Itertools;
use shared::puzzle_input;

fn elf_sum(elf: &str) -> usize {
    elf.split('\n')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .sum()
}
pub fn main() {
    let sorted_totals = puzzle_input!()
        .trim()
        .split("\n\n")
        .map(elf_sum)
        .sorted()
        .rev();
    let p1 = sorted_totals.clone().max().unwrap();
    let p2: usize = sorted_totals.take(3).sum();
    println!("part 1: {}\npart 2: {}", p1, p2);
}
