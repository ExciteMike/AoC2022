#![allow(unused_imports)]
use std::{
    collections::HashSet,
    iter::{self, FromIterator},
};

use itertools::Itertools;
use shared::puzzle_input;

fn follow(head: &[isize; 2], tail: &mut [isize; 2]) {
    if !(0..2)
        .map(|axis| ((head[axis] - 1)..=(head[axis] + 1)).contains(&tail[axis]))
        .all(|x| x)
    {
        for axis in 0..2 {
            tail[axis] += (head[axis] - tail[axis]).signum();
        }
    }
}

fn propagate(nodes: &mut [[isize; 2]]) {
    for i in 0..nodes.len() - 1 {
        follow(&nodes[i].clone(), &mut nodes[i + 1]);
    }
}

pub fn main() {
    let input = puzzle_input!();
    let mut nodes = [[0isize, 0isize]; 10];
    let mut visits1: HashSet<_> = HashSet::from_iter(iter::once([0isize; 2]));
    let mut visits9: HashSet<_> = HashSet::from_iter(iter::once([0isize; 2]));
    for line in input.split('\n') {
        let (dir, dist) = line.split_at(2);
        let dist = dist.parse().unwrap();
        let (dx, dy) = match dir {
            "U " => (0, 1),
            "D " => (0, -1),
            "L " => (-1, 0),
            "R " => (1, 0),
            _ => panic!(),
        };
        for _ in 0..dist {
            nodes[0][0] += dx;
            nodes[0][1] += dy;
            propagate(&mut nodes);
            visits1.insert(nodes[1]);
            visits9.insert(nodes[9]);
        }
    }

    println!("part 1: {}\npart 2: {}", visits1.len(), visits9.len());
}
