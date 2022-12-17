use itertools::Itertools;
use shared::puzzle_input;

const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

const EXAMPLE_P1PACE: [u16; 30] = [
    0, 0, 20, 40, 60, 93, 126, 159, 192, 246, 300, 354, 408, 462, 516, 570, 624, 700, 776, 852,
    928, 1007, 1086, 1165, 1244, 1323, 1402, 1481, 1560, 1600,
];
const EXAMPLE_P2PACE: [u16; 30] = [
    0, 0, 20, 61, 102, 143, 184, 260, 336, 414, 492, 573, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1707,
];
const P1PACE: [u16; 30] = [
    0, 0, 0, 0, 10, 20, 50, 100, 100, 100, 200, 300, 300, 400, 500, 600, 600, 700, 800, 900, 1000,
    1100, 1200, 1300, 1500, 1600, 1700, 1800, 2000, 2000,
];
const P2PACE: [u16; 30] = [
    0, 0, 0, 1, 10, 50, 100, 150, 200, 300, 400, 500, 600, 700, 850, 1000, 1150, 1300, 1500, 1650, 1850, 2000, 2200, 2400, 2600, 2800, 0, 0, 0, 0,
];

struct StackItem<const WORKERS: usize> {
    workers: [u8; WORKERS],
    minute: u8,
    flow: u16,
    pressure: u16,
    closed: u64,
}

#[derive(Clone)]
enum Action {
    Open(u8),
    Move(usize, u8),
}

fn moves(
    worker: usize,
    cur_valve: u8,
    closed: u64,
    rates: &[u8],
    edges: &[Box<[u8]>],
) -> Vec<Action> {
    let mut v = vec![];
    let mask = 0x1 << cur_valve;
    let valve_flow = rates[cur_valve as usize];
    if (0 != valve_flow) && (0 != (closed & mask)) {
        v.push(Action::Open(cur_valve));
    }
    for dst in edges[cur_valve as usize].as_ref() {
        v.push(Action::Move(worker, *dst));
    }
    v
}

fn play<const WORKERS: usize, const TIME: u8>(
    names: &[&'_ str],
    rates: &[u8],
    edges: &[Box<[u8]>],
    pace: &[u16; 30],
) -> u16 {
    let aa = names.iter().position(|&x| x == "AA").unwrap() as u8;
    let mut stack = Vec::with_capacity(128);
    stack.push(StackItem {
        workers: [aa; WORKERS],
        minute: 0,
        flow: 0,
        pressure: 0,
        closed: !0u64,
    });
    let mut best = 0;
    while let Some(state) = stack.pop() {
        // tick open valves
        let pressure = state.pressure + state.flow;
        // prune
        if pressure < pace[state.minute as usize] {
            continue;
        }
        let minute = state.minute + 1;
        // time up
        if minute >= TIME {
            if pressure > best {
                best = pressure;
            }
            continue;
        }
        for actions in state
            .workers
            .iter()
            .enumerate()
            .map(|(w, valve)| moves(w, *valve, state.closed, rates, edges))
            .multi_cartesian_product()
        {
            let mut flow = state.flow;
            let mut closed = state.closed;
            let mut workers = state.workers;
            for action in actions {
                match action {
                    Action::Open(valve) => {
                        if 0 != (closed & (0x1 << valve)) {
                            flow += rates[valve as usize] as u16;
                            closed = closed & !(0x1 << valve);
                        }
                    }
                    Action::Move(w, v) => {
                        workers[w] = v;
                    }
                }
            }
            stack.push(StackItem {
                workers,
                minute,
                flow,
                pressure,
                closed,
            });
        }
    }
    return best;
}

fn parse(input: &str) -> (Box<[&'_ str]>, Box<[u8]>, Box<[Box<[u8]>]>) {
    let mut names = Vec::with_capacity(59);
    let mut rates = Vec::with_capacity(59);
    let mut edges = Vec::with_capacity(59);
    for line in input.split('\n') {
        names.push(&line[6..8]);
        let (_, rest) = line.split_once(" has flow rate=").unwrap();
        let (rate, rest) = rest.split_once("; ").unwrap();
        let rate = rate.parse::<u8>().unwrap();
        rates.push(rate);
        let (_, _, _, _, rest) = rest.splitn(5, ' ').collect_tuple().unwrap();
        let to = rest.split(", ").collect_vec().into_boxed_slice();
        edges.push(to);
    }
    let edges = edges
        .iter()
        .map(|edges| {
            edges
                .iter()
                .map(|&dst| names.iter().position(|&x| x == dst).unwrap() as u8)
                .collect_vec()
                .into_boxed_slice()
        })
        .collect_vec();
    let names = names.into_boxed_slice();
    let rates = rates.into_boxed_slice();
    let edges = edges.into_boxed_slice();
    (names, rates, edges)
}

pub fn main() {
    let doexample = false;
    let (input, p1pace, p2pace) = if doexample {
        (String::from(EXAMPLE), EXAMPLE_P1PACE, EXAMPLE_P2PACE)
    } else {
        (puzzle_input!(), P1PACE, P2PACE)
    };
    let (names, rates, edges) = parse(&input);
    let p1 = play::<1, 30>(&names, &rates, &edges, &p1pace);
    println!("part 1: {}", p1); // 2181
    let p2 = play::<2, 26>(&names, &rates, &edges, &p2pace);
    println!("part 2: {}", p2); // >=2824
}
