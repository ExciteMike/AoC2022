use std::cmp::max;

use itertools::Itertools;
use shared::puzzle_input;

fn row_ranges(
    data: &[(isize, isize, isize, isize)],
    row: isize,
) -> std::vec::IntoIter<(isize, isize)> {
    data.iter()
        .filter_map(|(sx, sy, bx, by)| {
            let db = (sx - bx).abs() + (sy - by).abs();
            let drow = (sy - row).abs();
            let r = db - drow;
            if r > 0 {
                let lo = sx - r;
                let hi = sx + r;
                if *by == row {
                    if bx < sx {
                        Some((lo + 1, hi + 1))
                    } else {
                        Some((lo, hi))
                    }
                } else {
                    Some((lo, hi + 1))
                }
            } else {
                None
            }
        })
        .sorted()
}

fn count<'a, I: IntoIterator<Item = &'a (isize, isize)>>(i: I) -> isize {
    i.into_iter()
        .fold(None, |acc, (lo, hi)| match acc {
            None => Some((hi, hi - lo)),
            Some((prevhi, n)) => {
                let hi = max(prevhi, hi);
                let lo = max(prevhi, lo);
                if lo < hi {
                    Some((hi, n + hi - lo))
                } else {
                    Some((hi, n))
                }
            }
        })
        .unwrap()
        .1
}

pub fn main() {
    let input = puzzle_input!();
    let row = 2000000;
    let search_size = 4000000;
    //let (input, row, search_size) = (EXAMPLE, EXAMPLE_ROW, EXAMPLE_SEARCH_SIZE);
    let sensors = input
        .split('\n')
        .map(|s| {
            let (sx, s) = s[12..].split_once(',').unwrap();
            let (sy, s) = s[3..].split_once(':').unwrap();
            let (bx, s) = s[24..].split_once(',').unwrap();
            let by = &s[3..];
            [sx, sy, bx, by]
                .map(|s| s.parse::<isize>().unwrap())
                .iter()
                .cloned()
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    println!("part 1: {}", count(row_ranges(&sensors, row).as_slice())); // 6078701

    let (x, y) = sensors
        .iter()
        .flat_map(|(sx, sy, bx, by)| {
            let d = 1 + (sx - bx).abs() + (sy - by).abs();
            (0..=d).flat_map(move |offset| {
                [
                    (sx + d - offset, sy - offset),
                    (sx - offset, sy - d + offset),
                    (sx - d + offset, sy + offset),
                    (sx + offset, sy + d - offset),
                ]
            })
        })
        .find(|(x, y)| {
            let range = 0..=search_size;
            range.contains(x)
                && range.contains(y)
                && sensors.iter().all(|(sx, sy, bx, by)| {
                    let beacon_dist = (bx - sx).abs() + (by - sy).abs();
                    let candidate_dist = (x - sx).abs() + (y - sy).abs();
                    candidate_dist > beacon_dist
                })
        })
        .unwrap();
    println!("part 2: {}", x * 4000000 + y); // 12567351400528
}
