use itertools::Itertools;
use shared::{map_words, puzzle_input};
use std::collections::VecDeque;

fn parse_stacks(s: &str) -> Vec<VecDeque<char>> {
    let mut stacks = vec![VecDeque::<char>::new(); 9];
    for line in s.split('\n') {
        for (i, mut cs) in line.chars().chunks(4).into_iter().enumerate() {
            if let Some(c) = cs.find(|c| c.is_alphabetic()) {
                stacks.get_mut(i).unwrap().push_front(c);
            }
        }
    }
    stacks
}

fn run<I: Iterator<Item = (usize, usize, usize)>>(
    stacks: &mut [VecDeque<char>],
    cmds: I,
    p2: bool,
) -> String {
    for (count, from, to) in cmds {
        let from_v = stacks.get_mut(from - 1).unwrap();
        let to_move = from_v.split_off(from_v.len() - count);
        let to_v = stacks.get_mut(to - 1).unwrap();
        if p2 {
            to_v.extend(to_move.iter());
        } else {
            to_v.extend(to_move.iter().rev());
        }
    }
    stacks.iter().map(|v| v.back().unwrap()).collect::<String>()
}

pub fn main() {
    let input = puzzle_input!();
    let mut halves = input.split("\n\n");
    let mut stacks = parse_stacks(halves.next().unwrap());
    let cmds = halves
        .next()
        .unwrap()
        .split('\n')
        .map(|line| map_words!(line, |s: &str| s.parse::<usize>().unwrap(), 1, 3, 5));
    println!("part 1: {}", run(&mut stacks.clone(), cmds.clone(), false));
    println!("part 2: {}", run(&mut stacks, cmds, true));
}
