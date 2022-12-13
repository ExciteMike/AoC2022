use std::{cmp::Ordering, iter::Peekable};

use itertools::Itertools;
use shared::puzzle_input;

type I<'a> = Peekable<std::str::Chars<'a>>;

const KEYS: [&str; 2] = ["[[2]]", "[[6]]"];

fn pair_map<T, U, F>(src: (T, T), f: F) -> (U, U)
where
    F: FnMut(T) -> U,
{
    let mut f = f;
    (f(src.0), f(src.1))
}

fn digit_is_next(i: &mut I) -> bool {
    i.peek().map(char::is_ascii_digit).unwrap_or(false)
}

fn peek_auto_closed(i: &mut I, auto_depth: u32) -> Option<char> {
    i.peek().map(|c| {
        if c.is_ascii_digit() || auto_depth == 0 {
            *c
        } else {
            ']'
        }
    })
}

fn next_auto_closed<'a>(i: &mut I<'a>, auto_depth: &mut u32) -> Option<char> {
    match i.peek() {
        Some(c) if !c.is_ascii_digit() && *auto_depth > 0 => {
            *auto_depth -= 1;
            Some(']')
        }
        _ => i.next(),
    }
}

fn test_ints<'a>(left: &mut I<'a>, right: &mut I<'a>) -> Ordering {
    let mut l = 0;
    let mut r = 0;
    loop {
        match (digit_is_next(left), digit_is_next(right)) {
            (false, false) => return l.cmp(&r),
            (false, true) => return Ordering::Less,
            (true, false) => return Ordering::Greater,
            (true, true) => {
                l = l * 10 + left.next().unwrap().to_digit(10).unwrap();
                r = r * 10 + right.next().unwrap().to_digit(10).unwrap();
            }
        }
    }
}

fn test_pair(left: &str, right: &str) -> Ordering {
    let mut left = left.chars().peekable();
    let mut right = right.chars().peekable();
    let mut auto_close = (0, 0);
    loop {
        let cs = (
            peek_auto_closed(&mut left, auto_close.0),
            peek_auto_closed(&mut right, auto_close.1),
        );
        // check for end of string
        let cs = match cs {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (a, b) if a == b => {
                next_auto_closed(&mut left, &mut auto_close.0);
                next_auto_closed(&mut right, &mut auto_close.1);
                continue;
            }
            cs => pair_map(cs, |c| c.unwrap()),
        };

        // effectively auto encloses the number in []...
        // but it shortcuts. both would start with '[' after,
        // so instead we skip the other's '['
        match &cs {
            (c, '[') if c.is_ascii_digit() => {
                right.next();
                auto_close.0 += 1;
                continue;
            }
            ('[', c) if c.is_ascii_digit() => {
                left.next();
                auto_close.1 += 1;
                continue;
            }
            _ => (),
        }
        match pair_map(cs, |c| c.is_ascii_digit()) {
            (true, true) => match test_ints(&mut left, &mut right) {
                Ordering::Equal => {}
                x => return x,
            },
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            (false, false) => {}
        }
        match cs {
            (']', _) => return Ordering::Less,
            (_, ']') => return Ordering::Greater,
            x => panic!("unhandled {:?}", x),
        }
    }
}

pub fn main() {
    let input = puzzle_input!();
    let pairs = input.split("\n\n").map(|s| s.split('\n').collect_vec());
    let p1: usize = pairs
        .clone()
        .map(|v| test_pair(v[0], v[1]))
        .enumerate()
        .filter(|&t| t.1 == Ordering::Less)
        .map(|t| t.0 + 1)
        .sum();
    let mut sorted = pairs
        .flatten()
        .chain(KEYS)
        .sorted_by(|&l, &r| test_pair(l, r))
        .enumerate();
    let p2: usize = IntoIterator::into_iter(KEYS)
        .map(|k| sorted.find(|&(_, s)| s.eq(k)).unwrap().0 + 1)
        .product();

    println!("part 1: {}\npart 2: {}", p1, p2); // 5588 23958
}
