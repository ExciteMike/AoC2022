use itertools::Itertools;
use shared::{map_words, puzzle_input};

fn parse_stacks(s: &str) -> Vec<String> {
    let mut stacks = vec![String::new(); 9];
    for line in s.split('\n') {
        for (i, mut cs) in line.chars().chunks(4).into_iter().enumerate() {
            if let Some(c) = cs.find(|c| c.is_alphabetic()) {
                stacks.get_mut(i).unwrap().insert(0, c);
            }
        }
    }
    stacks
}

fn run(stacks: &mut [String], code: &str, reverse: bool) -> String {
    for line in code.split('\n') {
        let (count, from, to) = map_words!(line, |s: &str| s.parse::<usize>().unwrap(), 1, 3, 5);
        let from_v = stacks.get_mut(from - 1).unwrap();
        let to_move = from_v.split_off(from_v.len() - count);
        let to_v = stacks.get_mut(to - 1).unwrap();
        if reverse {
            to_v.extend(to_move.chars().rev());
        } else {
            to_v.extend(to_move.chars());
        }
    }
    stacks
        .iter_mut()
        .map(|v| v.pop().unwrap())
        .collect::<String>()
}

pub fn main() {
    let input = puzzle_input!();
    let (init, code) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = parse_stacks(init);
    println!("part 1: {}", run(&mut stacks.clone(), code, true));
    println!("part 2: {}", run(&mut stacks, code, false));
}
