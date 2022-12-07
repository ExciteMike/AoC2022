#![allow(unused_imports)]
use std::collections::HashMap;

use itertools::Itertools;
use shared::puzzle_input;

struct State {
    sizes: HashMap<Vec<String>, usize>,
    space: usize,
    cd: Vec<String>,
}

const TARGET: usize = 30000000;

fn cd<'a, I: Iterator<Item = &'a str>>(state: &mut State, mut tokens: I) {
    match tokens.next().unwrap() {
        ".." => {
            state.cd.pop();
        }
        "/" => {
            state.cd.clear();
        }
        s => state.cd.push(s.to_owned()),
    }
}

fn cmd<'a, I: Iterator<Item = &'a str>>(state: &mut State, mut tokens: I) {
    if "cd" == tokens.next().unwrap() {
        cd(state, tokens)
    }
}

fn dir<'a, I: Iterator<Item = &'a str>>(state: &mut State, mut tokens: I) {
    let mut path = state.cd.clone();
    path.push(tokens.next().unwrap().to_owned());
    state.sizes.insert(path, 0);
}

fn size(state: &mut State, file_size: usize) {
    let mut path = state.cd.clone();
    while !path.is_empty() {
        let x = state.sizes.get_mut(&path).unwrap();
        *x += file_size;
        path.pop();
    }

    state.space -= file_size;
}

pub fn main() {
    let mut state = State {
        sizes: HashMap::from([(vec![String::from("/")], 0)]),
        space: 70000000,
        cd: vec![],
    };
    let input = puzzle_input!();
    for line in input.split('\n') {
        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "$" => cmd(&mut state, tokens),
            "dir" => dir(&mut state, tokens),
            s => size(&mut state, s.parse().unwrap()),
        }
    }

    let p1: usize = state.sizes.values().filter(|&&x| x <= 100000).sum();

    let needed = TARGET - state.space;
    let p2 = state
        .sizes
        .values()
        .filter(|&&x| x >= needed)
        .min()
        .unwrap();

    println!("part 1: {}\npart 2: {}", p1, p2);
}
