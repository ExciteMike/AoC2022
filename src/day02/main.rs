use shared::puzzle_input;

fn score_p1(line: &str) -> usize {
    match line {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => panic!("could't understand '{}'", line),
    }
}

fn score_p2(line: &str) -> usize {
    match line {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => panic!("could't understand '{}'", line),
    }
}

pub fn main() {
    let input = puzzle_input!();
    let rounds = input.split('\n');
    let p1: usize = rounds.clone().map(score_p1).sum();
    let p2: usize = rounds.map(score_p2).sum();
    println!("part 1: {}\npart 2: {}", p1, p2);
}
