use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use shared::puzzle_input;

type T = i16;

fn play(
    blizzard_data: &[(T, T, T, T)],
    h: T,
    w: T,
    from_row: T,
    from_col: T,
    start_time: T,
    forward: bool,
) -> T {
    const CAPACITY: usize = 1024;
    let mut seen = HashSet::<(T, T, T)>::new();
    let mut stack = VecDeque::with_capacity(CAPACITY);
    let target = if forward { (h, w) } else { (0, 1) };
    stack.push_back((from_row, from_col, start_time));
    while let Some((row, col, time)) = stack.pop_front() {
        if seen.contains(&(row, col, time)) {
            continue;
        }
        seen.insert((row, col, time));
        let time = time + 1;
        if (row, col) == target {
            return time;
        }
        let possible_moves = if forward {
            [
                (row, col + 1),
                (row + 1, col),
                (row, col),
                (row - 1, col),
                (row, col - 1),
            ]
        } else {
            [
                (row, col - 1),
                (row - 1, col),
                (row, col),
                (row + 1, col),
                (row, col + 1),
            ]
        };
        let valid_steps = <[(T, T); 5] as IntoIterator>::into_iter(possible_moves)
            .filter(|(move_row, move_col)| {
                let t = time - start_time;
                (move_row >= &0) // can't go into negative rows
                    && ((move_row > &0) || (*move_col == 1)) // only one gap in the top row
                    && (move_col > &0) // can't be on left edge
                    && (move_row <= &(h+1)) // dont'fall off bottom of board
                    && ((move_row <= &h) || (*move_col == w)) // only one gap in bottom wall
                    && (move_col <= &w) // stay off right wall
                    && !blizzard_data.iter().any(|(r0, rstep, c0, cstep)| {
                        let r = 1 + (((r0 - 1) + (rstep * (time % h))) % h);
                        let c = 1 + (((c0 - 1) + (cstep * (time % w))) % w);
                        (r, c) == (*move_row, *move_col)
                    }) // can't share space with a blizzard
                    && (!forward || (row >= (t / 16))) // downward pace
                    && (forward || (row <= (h + 1 - t / 14))) // upward pace when reversing
                    && (!forward || (col >= (t / 6))) // rightward pace
                    && (forward || (col <= (w + 1 - t / 12))) // leftward pace when backwards
            })
            .map(|(r, c)| (r, c, time));
        stack.extend(valid_steps);
        if stack.len() > CAPACITY {
            panic!();
        }
    }
    unreachable!();
}

pub fn main() {
    let input = puzzle_input!();
    let lines = input.split('\n').collect_vec();
    let w = lines[0].len() as T - 2;
    let h = lines.len() as T - 2;
    let blizzard_data = lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '^' => Some((row as T, h - 1, col as T, 0)),
                    'v' => Some((row as T, 1, col as T, 0)),
                    '<' => Some((row as T, 0, col as T, w - 1)),
                    '>' => Some((row as T, 0, col as T, 1)),
                    _ => None,
                })
        })
        .collect_vec();
    let p1 = play(&blizzard_data, h, w, 0, 1, 0, true);
    println!("part 1: {}", p1);
    let temp = play(&blizzard_data, h, w, h + 1, w, p1, false);
    let p2 = play(&blizzard_data, h, w, 0, 1, temp, true);
    println!("part 2: {}", p2);
}
