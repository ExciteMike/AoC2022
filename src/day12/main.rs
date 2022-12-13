use std::collections::VecDeque;

use itertools::Itertools;
use shared::puzzle_input;

pub fn main() {
    let input = puzzle_input!();
    let mut start = (0isize, 0isize);
    let mut end = (0, 0);
    let mut heights = vec![vec![None; 160]; 42];
    for (y, line) in input.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = (x as isize, y as isize);
            match c {
                'S' => {
                    start = coord;
                    heights[y][x] = Some(10);
                }
                'E' => {
                    end = coord;
                    heights[y][x] = Some(35);
                }
                'a' => {
                    heights[y][x] = Some(10);
                }
                c => {
                    heights[y][x] = c.to_digit(36);
                }
            }
        }
    }

    let mut distances = vec![vec![None; 160]; 42];
    distances[end.1 as usize][end.0 as usize] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back((end, 0));
    while !queue.is_empty() {
        let ((x, y), d) = queue.pop_front().unwrap();
        let h = heights[y as usize][x as usize].unwrap();
        for (x, y) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if (x >= 0) && (y >= 0) && distances[y as usize][x as usize].is_none() {
                if let Some(h2) = heights[y as usize][x as usize] {
                    if h < h2 + 2 {
                        distances[y as usize][x as usize] = Some(d + 1);
                        queue.push_back(((x, y), d + 1));
                    }
                }
            }
        }
    }

    let p1 = distances[start.1 as usize][start.0 as usize].unwrap();

    let p2 = heights
        .into_iter()
        .enumerate()
        .flat_map(|(y, hs)| {
            hs.into_iter()
                .enumerate()
                .flat_map(|(x, h)| {
                    if let Some(10) = h {
                        distances[y][x]
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .min()
        .unwrap();

    println!("part 1: {}\npart 2: {}", p1, p2); // 447, 446
}
