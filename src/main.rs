#![warn(clippy::all)]
use std::io::BufRead;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    let days = (1..=25)
        .filter(|day| std::fs::metadata(format!("src/day{:02}/main.rs", day)).is_ok())
        .collect::<Vec<usize>>();
    println!(
        "Enter a day number (1-{} inclusive) or just press enter for default",
        days.len()
    );
    let chosen_day = if let Some(Ok(day)) = stdin_lines.next() {
        if let Ok(day) = day.parse::<usize>() {
            day
        } else {
            *days.last().unwrap()
        }
    } else {
        *days.last().unwrap()
    };
    println!("{}", do_day_rust(chosen_day));
    println!("Press enter to quit");
    stdin_lines.next();
    Ok(())
}

fn do_day_rust(chosen_day: usize) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(format!("day{:02}", chosen_day))
        .arg("--release")
        .output()
        .expect("error running process");
    if !output.stderr.is_empty() {
        eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap())
    }
    std::str::from_utf8(&output.stdout).unwrap().to_string()
}
#[cfg(test)]
fn do_day_python(chosen_day: usize) -> String {
    let output = Command::new("python")
        .arg(format!("src/day{:02}/main.py", chosen_day))
        .output()
        .expect("error running process");
    if !output.stderr.is_empty() {
        eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap())
    }
    std::str::from_utf8(&output.stdout)
        .unwrap()
        .replace("\r\n", "\n")
        .to_string()
}
#[cfg(test)]
#[track_caller]
fn do_test<T: std::fmt::Display, U: std::fmt::Display>(day: usize, p1: T, p2: U) {
    if std::path::Path::new(&format!("src/day{:02}/main.rs", day)).exists() {
        assert_eq!(
            do_day_rust(day),
            format!("part 1: {}\npart 2: {}\n", p1, p2),
            "Rust solution failed for day {}",
            day
        );
    }
    if std::path::Path::new(&format!("src/day{:02}/main.py", day)).exists() {
        assert_eq!(
            do_day_python(day),
            format!("part 1: {}\npart 2: {}\n", p1, p2),
            "Python solution failed for day {}",
            day
        );
    }
}
#[test]
fn day01() {
    do_test(1, 997899, 131248694);
}
#[test]
fn day02() {
    do_test(2, 465, 294);
}
#[test]
fn day03() {
    do_test(3, 207, 2655892800usize);
}
#[test]
fn day04() {
    do_test(4, 196, 114);
}
#[test]
fn day05() {
    do_test(5, 930, 515);
}
#[test]
fn day06() {
    do_test(6, 6683, 3122);
}
#[test]
fn day07() {
    do_test(7, 229, 6683);
}
#[test]
fn day08() {
    do_test(8, 1394, 1626);
}
#[test]
fn day09() {
    do_test(9, 50047984, 5407707);
}
#[test]
fn day10() {
    do_test(10, 1885, 2024782584832usize);
}
#[test]
fn day11() {
    do_test(11, 2359, 2131);
}
#[test]
fn day12() {
    do_test(12, 757, 51249);
}
#[test]
fn day13() {
    do_test(13, 205, 803025030761664u64);
}
#[test]
fn day14() {
    do_test(14, 11327140210986u64, 2308180581795u64);
}
#[test]
fn day15() {
    do_test(15, 981, 164878);
}
#[test]
fn day16() {
    do_test(16, 25059, 3253972369789u64);
}
#[test]
fn day17() {
    do_test(17, 271, 2064);
}
#[test]
fn day18() {
    do_test(18, 23507031841020u64, 218621700997826u64);
}
#[test]
fn day19() {
    do_test(19, 147, 263);
}
#[test]
fn day20() {
    do_test(20, 14129524957217u64, 1649);
}
#[test]
fn day21() {
    do_test(
        21,
        2020,
        "bcdgf,xhrdsl,vndrb,dhbxtb,lbnmsr,scxxn,bvcrrfbr,xcgtv",
    );
}
#[test]
fn day22() {
    do_test(22, 32366, 30891);
}
#[test]
fn day23() {
    do_test(23, 0, 0);
}
#[test]
fn day24() {
    do_test(24, 0, 0);
}
#[test]
fn day25() {
    do_test(25, 0, 0);
}
