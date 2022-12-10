use shared::puzzle_input;

pub fn main() {
    let input = puzzle_input!();
    let xs = input.split('\n').fold(vec![1, 1], |mut xs, line| {
        let x = *xs.last().unwrap();
        xs.push(x);
        if line.starts_with("addx") {
            let v = line[5..].parse::<isize>().unwrap();
            xs.push(x + v);
        }
        xs
    });
    let p1: isize = (20..=220).step_by(40).map(|i| i as isize * xs[i]).sum();
    print!("part 1: {}\npart 2: \n", p1);
    for y in 0..6 {
        for x in 0..40 {
            let sprite = xs[(y * 40 + x) as usize];
            let s = if (0..=2).contains(&(x - sprite)) {
                "[]"
            } else {
                "  "
            };
            print!("{}", s);
        }
        println!();
    }
}
