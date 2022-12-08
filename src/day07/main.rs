#![allow(unused_imports)]
use std::collections::HashMap;

use itertools::Itertools;
use shared::puzzle_input;

const TARGET: usize = 30000000;

pub fn main() {
    let mut sizes = HashMap::from([(vec!["/"], 0)]);
    let mut space = 70000000;
    let mut cd = vec![];
    let input = puzzle_input!();
    for line in input.split('\n') {
        let v = line.split_whitespace().collect_vec();
        match v.as_slice() {
            ["$", "cd", ".."] => {
                cd.pop();
            }
            ["$", "cd", "/"] => cd.clear(),
            ["$", "cd", s] => cd.push(*s),
            ["$", "ls"] => (),
            ["dir", s] => {
                let mut path = cd.clone();
                path.push(s);
                sizes.insert(path, 0);
            }
            [s, _] if s.chars().all(|c| c.is_digit(10)) => {
                let file_size: usize = s.parse().unwrap();
                let mut path = cd.clone();
                while !path.is_empty() {
                    let x = sizes.get_mut(&path).unwrap();
                    *x += file_size;
                    path.pop();
                }

                space -= file_size;
            }
            _ => panic!(),
        }
    }

    let p1: usize = sizes.values().filter(|&&x| x <= 100000).sum();

    let needed = TARGET - space;
    let p2 = sizes.values().filter(|&&x| x >= needed).min().unwrap();

    println!("part 1: {}\npart 2: {}", p1, p2);
}
