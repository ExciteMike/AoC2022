use std::collections::HashMap;

use itertools::Itertools;
use shared::puzzle_input;

const ROCKSDEF: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";
const WELL_WIDTH: u8 = 7;
const START_MARGIN_X: u8 = 2;
const START_MARGIN_Y: u16 = 3;

struct Rock {
    data: Box<[(u8, u16)]>,
    w: u8,
    h: u16,
}
impl Rock {
    pub fn new<I: Iterator<Item = (u8, u16)>>(i: I) -> Rock {
        let data = i.collect_vec().into_boxed_slice();
        let w = data.iter().map(|x| x.0 + 1).max().unwrap_or(0);
        let h = data.iter().map(|x| x.1 + 1).max().unwrap_or(0);
        Rock { data, w, h }
    }
}
type Well = Vec<u8>;

fn rocks() -> Box<[Rock]> {
    ROCKSDEF
        .split("\n\n")
        .map(|s| {
            Rock::new(s.split('\n').rev().enumerate().flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some((x as u8, y as u16)),
                    _ => None,
                })
            }))
        })
        .collect_vec()
        .into_boxed_slice()
}

enum CycleDetect {
    Searching(HashMap<(u8, usize, usize), (u16, u16)>),
    FindRemainder {
        cycs: u64,
        height_before_cycle: u16,
        cycle_height: u16,
        target_block: u16,
    },
}

fn play(target: u64, rocks: &[Rock], jets: &[char]) -> u64 {
    let mut well = Well::new();
    let mut rock_i = 0;
    let mut jets_i = 0;
    let mut height = 0u16;
    let mut cycle_detect = CycleDetect::Searching(HashMap::new());
    let mut i = 0u16;
    loop {
        let rock = &rocks[rock_i];
        let y = height + START_MARGIN_Y;
        let drop_loc = drop(rock, &well, y, jets, &mut jets_i);
        height = std::cmp::max(height, drop_loc.1 + rock.h);
        place_rock(&mut well, rock, drop_loc);
        rock_i = (rock_i + 1) % rocks.len();
        i += 1;

        cycle_detect = if let CycleDetect::Searching(mut map) = cycle_detect {
            let state = (*well.last().unwrap(), rock_i, jets_i);
            match map.insert(state, (i, height)) {
                Some((cycle_begin, height_before_cycle)) => {
                    let cycle_len = i - cycle_begin;
                    let cycle_height = height - height_before_cycle;
                    let cycs = (target - cycle_begin as u64) / cycle_len as u64;
                    let remainder = (target - cycle_begin as u64) % cycle_len as u64;
                    let target_block = i + (remainder as u16);
                    CycleDetect::FindRemainder {
                        cycs,
                        height_before_cycle,
                        cycle_height,
                        target_block,
                    }
                }
                None => CycleDetect::Searching(map),
            }
        } else {
            cycle_detect
        };
        if let CycleDetect::FindRemainder {
            height_before_cycle,
            cycle_height,
            cycs,
            target_block,
        } = cycle_detect
        {
            if i == target_block {
                let remainder_height = (height - cycle_height - height_before_cycle) as u64;
                return height_before_cycle as u64
                    + cycs * cycle_height as u64
                    + remainder_height as u64;
            }
        }
    }
}

fn drop(rock: &Rock, well: &Well, mut y: u16, jets: &[char], jets_i: &mut usize) -> (u8, u16) {
    let mut x = START_MARGIN_X;
    loop {
        let jet = jets[*jets_i % jets.len()];
        *jets_i = (*jets_i + 1) % jets.len();
        match jet {
            '<' if x == 0 => {}
            '>' if x + rock.w >= WELL_WIDTH => {}
            '<' if overlap(rock, x - 1, y, well) => {}
            '>' if overlap(rock, x + 1, y, well) => {}
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            _ => {}
        }
        if (y == 0) || (y + 1 < rock.h) || overlap(rock, x, y - 1, well) {
            return (x, y);
        }
        y -= 1;
    }
}

fn place_rock(well: &mut Well, rock: &Rock, loc: (u8, u16)) {
    if well.len() as u16 <= loc.1 + rock.h {
        well.resize((loc.1 + rock.h) as usize, 0)
    }
    for (x, y) in rock.data.iter() {
        let (x, y) = (x + loc.0, y + loc.1);
        well[y as usize] |= 0x1 << x;
    }
}

fn overlap(rock: &Rock, ox: u8, oy: u16, well: &Well) -> bool {
    for (x, y) in rock.data.iter() {
        let (x, y) = (x + ox, y + oy);
        if let Some(row) = well.get(y as usize) {
            if 0 != (row & (0x1 << x)) {
                return true;
            }
        }
    }
    false
}

pub fn main() {
    let jets = puzzle_input!().chars().collect_vec();
    let rocks = rocks();
    println!("part 1: {}", play(2022, &rocks, &jets));
    println!("part 2: {}", play(1000000000000u64, &rocks, &jets));
}
