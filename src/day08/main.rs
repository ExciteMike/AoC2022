#![allow(unused_imports)]
use std::{iter::Map, ops::RangeInclusive};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use shared::puzzle_input;

fn north_from(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<u32> {
    grid[0..y].iter().rev().map(|row| row[x]).collect_vec()
}
fn south_from(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<u32> {
    grid[y + 1..].iter().map(|row| row[x]).collect_vec()
}
fn east_from(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<u32> {
    grid[y][x + 1..].iter().cloned().collect_vec()
}
fn west_from(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<u32> {
    grid[y][0..x].iter().rev().cloned().collect_vec()
}
fn sight_lines(grid: &[Vec<u32>], x: usize, y: usize) -> Vec<Vec<u32>> {
    vec![
        north_from(grid, x, y),
        south_from(grid, x, y),
        east_from(grid, x, y),
        west_from(grid, x, y),
    ]
}

fn visible_from_outside(h: u32, sight_lines: &[Vec<u32>]) -> bool {
    sight_lines
        .iter()
        .any(|trees| trees.iter().all(|&h2| h2 < h))
}

fn p2_score(h: u32, sight_lines: &[Vec<u32>]) -> usize {
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
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let data = (0..heights.len()).map(|y| {
        let row = &heights[y];
        (0..row.len())
            .map(|x| (row[x], sight_lines(&heights, x, y)))
            .collect_vec()
    });

    let visible_from_outside = data.clone().map(|row| {
        row.iter()
            .map(|(h, lines)| visible_from_outside(*h, lines))
            .collect_vec()
    });
    let p1: usize = visible_from_outside
        .map(|row| row.iter().filter(|x| **x).count())
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
