use std::collections::{HashMap, HashSet};

use shared::puzzle_input;

type T = i8;

fn neighbors(elf: (T, T)) -> [(T, T); 8] {
    [
        (elf.0 - 1, elf.1 - 1),
        (elf.0 - 1, elf.1),
        (elf.0 - 1, elf.1 + 1),
        (elf.0, elf.1 - 1),
        (elf.0, elf.1 + 1),
        (elf.0 + 1, elf.1 - 1),
        (elf.0 + 1, elf.1),
        (elf.0 + 1, elf.1 + 1),
    ]
}
fn neighbors_north(elf: (T, T)) -> [(T, T); 3] {
    [
        (elf.0 - 1, elf.1 - 1),
        (elf.0 - 1, elf.1),
        (elf.0 - 1, elf.1 + 1),
    ]
}
fn neighbors_south(elf: (T, T)) -> [(T, T); 3] {
    [
        (elf.0 + 1, elf.1 - 1),
        (elf.0 + 1, elf.1),
        (elf.0 + 1, elf.1 + 1),
    ]
}
fn neighbors_west(elf: (T, T)) -> [(T, T); 3] {
    [
        (elf.0 - 1, elf.1 - 1),
        (elf.0, elf.1 - 1),
        (elf.0 + 1, elf.1 - 1),
    ]
}
fn neighbors_east(elf: (T, T)) -> [(T, T); 3] {
    [
        (elf.0 - 1, elf.1 + 1),
        (elf.0, elf.1 + 1),
        (elf.0 + 1, elf.1 + 1),
    ]
}

fn do_round<I: Iterator<Item = T> + Clone>(
    elves: HashSet<(T, T)>,
    order: I,
) -> Result<(), HashSet<(T, T)>> {
    let mut moves = HashMap::<(T, T), (T, T)>::new();
    let mut new_elves = HashSet::<(T, T)>::new();
    'outer: for &from in elves.iter() {
        if neighbors(from).iter().any(|p| elves.contains(p)) {
            for i in order.clone() {
                let to = match i {
                    0 if !neighbors_north(from).iter().any(|p| elves.contains(p)) => {
                        Some((from.0 - 1, from.1))
                    }
                    1 if !neighbors_south(from).iter().any(|p| elves.contains(p)) => {
                        Some((from.0 + 1, from.1))
                    }
                    2 if !neighbors_west(from).iter().any(|p| elves.contains(p)) => {
                        Some((from.0, from.1 - 1))
                    }
                    3 if !neighbors_east(from).iter().any(|p| elves.contains(p)) => {
                        Some((from.0, from.1 + 1))
                    }
                    _ => None,
                };
                if let Some(to) = to {
                    if let Some(&rewind) = moves.get(&to) {
                        new_elves.remove(&to);
                        new_elves.insert(rewind);
                        new_elves.insert(from);
                    } else {
                        moves.insert(to, from);
                        new_elves.insert(to);
                    }
                    continue 'outer;
                }
            }
        }
        new_elves.insert(from);
    }
    if new_elves.difference(&elves).next().is_none() {
        Ok(())
    } else {
        Err(new_elves)
    }
}
fn score(elves: &HashSet<(T, T)>) -> u64 {
    let row0 = elves.iter().map(|x| x.0).min().unwrap();
    let row1 = elves.iter().map(|x| x.0).max().unwrap();
    let col0 = elves.iter().map(|x| x.1).min().unwrap();
    let col1 = elves.iter().map(|x| x.1).max().unwrap();
    let h = (row1 + 1 - row0) as u64;
    let w = (col1 + 1 - col0) as u64;
    h * w - (elves.len() as u64)
}
fn play(mut elves: HashSet<(T, T)>, max_rounds: usize) -> Result<usize, HashSet<(T, T)>> {
    let directions_in_order: [T; 7] = [0, 1, 2, 3, 0, 1, 2];
    for round in 0..max_rounds {
        let i = round % 4;
        match do_round(elves, (directions_in_order[i..(i + 4)]).iter().cloned()) {
            Ok(()) => return Ok(round + 1),
            Err(e) => elves = e,
        }
    }
    Err(elves)
}
pub fn main() {
    let input = puzzle_input!();
    //let input = EXAMPLE;
    let elves = input
        .split('\n')
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '#' => Some((row as T, col as T)),
                    _ => None,
                })
        })
        .collect::<HashSet<_>>();
    let p1 = score(&play(elves.clone(), 10).unwrap_err());
    let p2 = play(elves, usize::MAX).unwrap();

    println!("part 1: {}\npart 2: {}", p1, p2);
}
