#![allow(soft_unstable)]
use std::collections::VecDeque;

use itertools::{izip, Itertools};
use regex::Regex;
use shared::puzzle_input;

const ALLOW_ALL: [bool; 4] = [true, true, true, true];

type Blueprint = [[u8; 3]; 4];

type Counts = [u8; 4];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    bot_counts: Counts,
    resources: Counts,
}

fn can_afford(costs: &[u8; 3], resources: &Counts) -> bool {
    costs.iter().zip(resources.iter()).all(|(a, b)| a <= b)
}

fn spend(mut resources: Counts, costs: &[u8; 3]) -> Counts {
    for i in 0..3 {
        resources[i] -= costs[i];
    }
    resources
}

fn gather(mut resources: Counts, bots: &Counts) -> Counts {
    for i in 0..4 {
        resources[i] += bots[i];
    }
    resources
}

fn play(bp: &Blueprint, minutes: u8) -> u8 {
    // PRUNING - changed from DFS to BFS so that we can check when things are falling behind with just 1 number
    let mut queue = VecDeque::<_>::new();
    queue.push_back((
        State {
            bot_counts: [1, 0, 0, 0],
            resources: Counts::default(),
        },
        ALLOW_ALL,
        1u8,
    ));
    let caps = izip!(bp[0].iter(), bp[1].iter(), bp[2].iter(), bp[3].iter())
        .map(|(a, b, c, d)| *[a, b, c, d].iter().max().unwrap())
        .collect_vec();
    let mut best = 0;
    'next: while let Some((state, allow, time)) = queue.pop_front() {
        best = best.max(state.resources[3]);

        if time > minutes {
            continue;
        }

        // PRUNING - falling behind the rest of the BFS
        if state.resources[3] + state.bot_counts[3] + 1 < best {
            continue;
        }

        // PRUNING - if we skipped building something we could, no point in considering building it next time
        let allow_after_skip = [0, 1, 2, 3].map(|i| !can_afford(&bp[i], &state.resources));

        for bot_type in (0..4).rev() {
            let full = (bot_type != 3) && (state.bot_counts[bot_type] >= *caps[bot_type]);
            if !full && allow[bot_type] && !allow_after_skip[bot_type] {
                let resources = spend(state.resources, &bp[bot_type]);
                let resources = gather(resources, &state.bot_counts);
                let mut bot_counts = state.bot_counts;
                bot_counts[bot_type] += 1;
                queue.push_back((
                    State {
                        bot_counts,
                        resources,
                    },
                    ALLOW_ALL,
                    time + 1,
                ));
                // PRUNING - if we built a geode bot, don't consider other bots or skipping
                if bot_type == 3 {
                    continue 'next;
                }
            }
        }

        let resources = gather(state.resources, &state.bot_counts);
        queue.push_back((State { resources, ..state }, allow_after_skip, time + 1));
    }
    best
}

pub fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let (p1, p2) = {
        let sep = Regex::new(r"[^0-9]+").unwrap();
        let input = puzzle_input!();
        let bps = input
            .split('\n')
            .map(|line| {
                let v = sep
                    .split(line)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect_vec();
                [[v[1], 0, 0], [v[2], 0, 0], [v[3], v[4], 0], [v[5], 0, v[6]]]
            })
            .collect_vec();
        let p1 = bps
            .iter()
            .map(|bp| play(bp, 24) as usize)
            .enumerate()
            .map(|(i, v)| (i + 1) * v)
            .sum::<usize>();
        let p2 = bps
            .iter()
            .take(3)
            .map(|bp| play(bp, 32) as usize)
            .product::<usize>();
        (p1, p2)
    };
    let elapsed = now.elapsed();
    eprintln!("Elapsed: {:.2?}", elapsed);

    // 1395, 2700
    println!("part 1: {}\npart 2: {}", p1, p2);
}
