use itertools::Itertools;
use shared::puzzle_input;

pub fn main() {
    let input = puzzle_input!();
    let _parsed_input = input
        .split('\n')
        .map(|line| {
            line.chars().fold(
                0u32,
                |acc, c| if c == '#' { (acc << 1) | 1 } else { acc << 1 },
            )
        })
        .collect_vec();
    let p1 = 0;
    let p2 = 0;

    println!("part 1: {}\npart 2: {}", p1, p2);
}
