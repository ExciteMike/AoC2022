use itertools::Itertools;
use shared::puzzle_input;

#[derive(Debug, Clone)]
enum Op {
    Square,
    Add(usize),
    Mult(usize),
}

#[derive(Debug, Clone)]
struct Monkey {
    op: Op,
    items: Vec<usize>,
    test: usize,
    pass_dst: usize,
    fail_dst: usize,
    count: usize,
}

const MODULO: usize = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

fn parse_monkey(s: &str) -> Monkey {
    let mut i = s.split('\n');
    let _ = i.next().unwrap();
    let items = i.next().unwrap()[18..]
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let op = match &i.next().unwrap()[23..].split(' ').collect_vec()[..] {
        ["*", "old"] => Op::Square,
        ["*", s] => Op::Mult(s.parse().unwrap()),
        ["+", s] => Op::Add(s.parse().unwrap()),
        _ => panic!(),
    };
    let test = i.next().unwrap()[21..].parse().unwrap();
    let pass_dst = i.next().unwrap()[29..].parse().unwrap();
    let fail_dst = i.next().unwrap()[30..].parse().unwrap();
    Monkey {
        op,
        items,
        test,
        pass_dst,
        fail_dst,
        count: 0,
    }
}

fn run_turn(m: &mut Monkey, div3: bool) -> Vec<(usize, usize)> {
    let throws = m
        .items
        .iter()
        .map(|x| {
            let x = match m.op {
                Op::Square => x * x,
                Op::Add(y) => x + y,
                Op::Mult(y) => x * y,
            };
            let x = if div3 { x / 3 } else { x };
            let x = x % MODULO;
            let dst = if (x % m.test) == 0 {
                m.pass_dst
            } else {
                m.fail_dst
            };
            (dst, x)
        })
        .collect_vec();
    m.count += m.items.len();
    m.items.clear();
    throws
}

fn run_game(input: &str, rounds: usize, div3: bool) -> usize {
    let mut ms = input.split("\n\n").map(parse_monkey).collect_vec();
    for _round in 0..rounds {
        for i in 0..ms.len() {
            for (dst, x) in run_turn(&mut ms[i], div3) {
                ms[dst].items.push(x);
            }
        }
    }
    ms.iter().map(|m| m.count).sorted().rev().take(2).product()
}

pub fn main() {
    let input = puzzle_input!();
    println!(
        "part 1: {}\npart 2: {}",
        run_game(&input, 20, true),
        run_game(&input, 10000, false)
    );
}
