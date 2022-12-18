use std::{
    cmp::{max, min},
    collections::BTreeSet,
    ops::RangeInclusive,
};

use itertools::Itertools;
use shared::puzzle_input;

fn neighbors(x: i8, y: i8, z: i8) -> [(i8, i8, i8); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

pub fn main() {
    let input = puzzle_input!();
    let cubes = input
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i8>().unwrap())
                .collect_tuple::<(i8, i8, i8)>()
                .unwrap()
        })
        .collect::<BTreeSet<_>>();
    let p1: usize = cubes
        .iter()
        .cloned()
        .map(|(x, y, z)| {
            neighbors(x, y, z)
                .iter()
                .filter(|(x, y, z)| !cubes.contains(&(*x, *y, *z)))
                .count()
        })
        .sum();
    let ranges = cubes
        .iter()
        .fold(
            Option::<[RangeInclusive<_>; 3]>::None,
            |ranges, &(x, y, z)| match ranges {
                None => Some([x..=x, y..=y, z..=z]),
                Some([xs, ys, zs]) => Some([
                    min(*xs.start(), x - 1)..=max(*xs.end(), x + 1),
                    min(*ys.start(), y - 1)..=max(*ys.end(), y + 1),
                    min(*zs.start(), z - 1)..=max(*zs.end(), z + 1),
                ]),
            },
        )
        .unwrap();

    let mut stack = vec![(0, 0, 0)];
    let mut exterior = BTreeSet::<(i8, i8, i8)>::new();
    while let Some((x, y, z)) = stack.pop() {
        for (x, y, z) in neighbors(x, y, z) {
            if ranges.iter().zip(&[x, y, z]).all(|(r, v)| r.contains(v))
                && !cubes.contains(&(x, y, z))
                && !exterior.contains(&(x, y, z))
            {
                exterior.insert((x, y, z));
                stack.push((x, y, z));
            }
        }
    }
    let p2 = p1
        - cubes
            .iter()
            .map(|(x, y, z)| {
                neighbors(*x, *y, *z)
                    .iter()
                    .filter(|(x, y, z)| {
                        !cubes.contains(&(*x, *y, *z)) && !exterior.contains(&(*x, *y, *z))
                    })
                    .count()
            })
            .sum::<usize>();

    println!("part 1: {}\npart 2: {}", p1, p2);
}
