use itertools::Itertools;
use shared::puzzle_input;

const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

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
const TO_DROP: u16 = 2022;
const WELL_WIDTH: u8 = 7;
const START_MARGIN_X: u8 = 2;
const START_MARGIN_Y: u16 = 3;

#[derive(Debug)]
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

fn play<'a, R, J>(rocks: R, jets: J) -> u16
where
    R: Iterator<Item = &'a Rock> + Clone,
    J: Iterator<Item = char> + Clone,
{
    let mut well = Well::new();
    let mut rocks = rocks.cycle();
    let mut jets = jets.cycle();
    let mut height = 0u16;
    for i in 0..TO_DROP {
        let rock = rocks.next().unwrap();
        let y = height + START_MARGIN_Y;
        let drop_loc = drop(rock, &well, y, &mut jets);
        height = std::cmp::max(height, drop_loc.1 + rock.h);
        place_rock(&mut well, rock, drop_loc);
    }
    height
}

fn drop<J>(rock: &Rock, well: &Well, mut y: u16, jets: &mut J) -> (u8, u16)
where
    J: Iterator<Item = char> + Clone,
{
    let mut x = START_MARGIN_X;
    for jet in jets {
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
        if (y == 0) || (y+1 < rock.h) || overlap(rock, x, y - 1, well) {
            return (x, y);
        }
        y -= 1;
    }
    unreachable!();
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

fn debug_draw(well: &Well) {
    println!();
    for y in (0..well.len()).rev() {
        print!("|");
        for x in 0..7 {
            match well[y] & (0x1 << x) {
                0 => print!(".."),
                _ => print!("[]")
            }
        }
        println!("|");
    }
    println!("+--------------+\n");
}

pub fn main() {
    let input = puzzle_input!();
    let rocks = rocks();
    let jets = if false {
        EXAMPLE.chars()
    } else {
        input.chars()
    };
    let p1 = play(rocks.iter(), jets);
    println!("part 1: {}", p1);
}
