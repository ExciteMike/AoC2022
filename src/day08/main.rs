#![allow(unused_imports)]
use std::{iter::Map, ops::RangeInclusive};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use shared::puzzle_input;

fn sight_lines(grid: &[Box<[u32]>], x: usize, y: usize) -> [Box<[u32]>; 4] {
    [
        grid[0..y].iter().rev().map(|row| row[x]).collect_vec(),
        grid[y + 1..].iter().map(|row| row[x]).collect_vec(),
        grid[y][x + 1..].iter().cloned().collect_vec(),
        grid[y][0..x].iter().rev().cloned().collect_vec(),
    ]
    .map(|x| x.into_boxed_slice())
}

fn visible_from_outside(h: u32, sight_lines: &[Box<[u32]>]) -> bool {
    sight_lines
        .iter()
        .any(|trees| trees.iter().all(|&h2| h2 < h))
}

fn p2_score(h: u32, sight_lines: &[Box<[u32]>]) -> usize {
    sight_lines
        .iter()
        .map(|line| {
            let x = line.iter().fold_while(0, |count, h2| {
                if h2 >= &h {
                    Done(count + 1)
                } else {
                    Continue(count + 1)
                }
            });
            match x {
                Continue(x) => x,
                Done(x) => x,
            }
        })
        .product::<usize>()
}

pub fn main() {
    let input = puzzle_input!();
    let heights = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec()
                .into_boxed_slice()
        })
        .collect_vec()
        .into_boxed_slice();
    let data = (0..heights.len()).map(|y| {
        let row = &heights[y];
        (0..row.len())
            .map(|x| (row[x], sight_lines(&heights, x, y)))
            .collect_vec()
            .into_boxed_slice()
    });

    let p1: usize = data
        .clone()
        .map(|row| {
            row.iter()
                .map(|(h, lines)| visible_from_outside(*h, lines))
                .filter(|x| *x)
                .count()
        })
        .sum();
    let p2: usize = data
        .map(|row| {
            row.iter()
                .map(|(h, lines)| p2_score(*h, lines))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("part 1: {}\npart 2: {}", p1, p2);
}
