use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;
use shared::puzzle_input;

enum Bottom {
    Floor(isize),
    Void(isize),
}

fn drop_sand(blocked: &HashSet<(isize, isize)>, bottom: &Bottom) -> Option<(isize, isize)> {
    let mut x = 500;
    let mut y = 0;
    // make sure we can even drop it
    if blocked.contains(&(x, y)) {
        return None;
    }
    loop {
        match bottom {
            // come to rest on floor
            Bottom::Floor(floor) if y >= *floor => {
                return Some((x, y));
            }
            // if it would fall out into the void, stop!
            Bottom::Void(void) if y >= *void => return None,
            _ => (),
        }
        // fall one step
        let next_x = [x, x - 1, x + 1]
            .iter()
            .cloned()
            .find(|x| !blocked.contains(&(*x, y + 1)));
        if let Some(next_x) = next_x {
            x = next_x;
            y += 1;
        } else {
            // found where it came to rest!
            return Some((x, y));
        }
    }
}

fn play(blocked: &HashSet<(isize, isize)>, bottom: Bottom) -> usize {
    let mut blocked = blocked.clone();
    let before = blocked.len();
    loop {
        match drop_sand(&blocked, &bottom) {
            None => return blocked.len() - before,
            Some(p) => {
                blocked.insert(p);
            }
        }
    }
}

pub fn main() {
    let input = puzzle_input!();
    let parsed = input
        .split('\n')
        .map(|line| {
            line.split(" -> ").map(|s| {
                s.split(',')
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect_tuple::<(isize, isize)>()
                    .unwrap()
            })
        })
        .collect_vec();
    let mut blocked = HashSet::new();
    for scan in parsed {
        for ((x1, y1), (x2, y2)) in scan.tuple_windows() {
            let (x1, x2) = (min(x1, x2), max(x1, x2));
            let (y1, y2) = (min(y1, y2), max(y1, y2));
            for y in y1..=y2 {
                for x in x1..=x2 {
                    blocked.insert((x, y));
                }
            }
        }
    }
    let void_y = blocked.iter().map(|x| x.1).max().unwrap();
    println!("part 1: {}", play(&blocked, Bottom::Void(void_y))); // 913
    println!("part 2: {}", play(&blocked, Bottom::Floor(void_y + 1))); // 30762
}
