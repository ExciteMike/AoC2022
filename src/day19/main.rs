use itertools::{izip, Itertools};
use regex::Regex;
use shared::puzzle_input;

#[derive(Debug)]
struct Blueprint {
    id: u8,
    costs: [[u8; 3]; 4],
}

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

fn play(bp: &Blueprint) -> u8 {
    let mut stack = vec![(
        State {
            bot_counts: [1, 0, 0, 0],
            resources: Counts::default(),
        },
        0,
    )];
    let caps = izip!(
        bp.costs[0].iter(),
        bp.costs[1].iter(),
        bp.costs[2].iter(),
        bp.costs[3].iter()
    )
    .map(|(a, b, c, d)| *[a, b, c, d].iter().max().unwrap())
    .collect_vec();
    if bp.id == 1 {
        eprintln!("{:?}", bp);
        eprintln!("{:?}", caps);
    }
    let mut best = 0;
    let mut geo_bot_counts= vec![0;25];
    while let Some((state, time)) = stack.pop() {
        if (state.resources[3] > best) || bp.id == 1 {
            eprintln!(
                "BP {} MINUTE {} bots {:?} resources {:?}",
                bp.id, time, state.bot_counts, state.resources
            );
        }
        best = std::cmp::max(best, state.resources[3]);

        let geo_bot_count = state.bot_counts[3];
        if geo_bot_count > geo_bot_counts[time as usize] {
            geo_bot_counts[time as usize] = geo_bot_count;
        } else if geo_bot_count < geo_bot_counts[time as usize] {
            continue;
        }

        let time = time + 1;
        if time > 24 {
            continue;
        }

        for bot_type in (0..4).rev() {
            if bp.id == 1 && time == 11 && bot_type == 2{
                eprintln!("11 {:?} {:?} {}", bp.costs[bot_type], state.resources, can_afford(&bp.costs[bot_type], &state.resources));
            }
            let full = (bot_type != 3) && (state.bot_counts[bot_type] >= *caps[bot_type]);
            if !full && can_afford(&bp.costs[bot_type], &state.resources) {
                let resources = spend(state.resources, &bp.costs[bot_type]);
                let resources = gather(resources, &state.bot_counts);
                let mut bot_counts = state.bot_counts;
                bot_counts[bot_type] += 1;
                stack.push((
                    State {
                        bot_counts,
                        resources,
                    },
                    time,
                ));
            }
        }

        if !state.resources.iter().zip(caps.iter()).all(|(r,c)|r>=c) {
            let resources = gather(state.resources, &state.bot_counts);
            stack.push((State { resources, ..state }, time));
        }
    }
    best
}

pub fn main() {
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
            Blueprint {
                id: v[0],
                costs: [[v[1], 0, 0], [v[2], 0, 0], [v[3], v[4], 0], [v[5], 0, v[6]]],
            }
        })
        .collect_vec();
    let p1 = bps
        .iter()
        .map(|bp| play(&bp) as u16 * bp.id as u16)
        .max()
        .unwrap();
    let p2 = 0;

    println!("part 1: {}\npart 2: {}", p1, p2);
}
