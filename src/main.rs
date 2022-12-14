#![warn(clippy::all)]
use chrono::{NaiveDate, Utc};
use std::cmp::min;
use std::process::Command;
use std::{cmp::max, io::BufRead};

const YEAR: usize = 2022;

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    let now = Utc::now().date_naive();
    let start_of_month = NaiveDate::parse_from_str(&format!("{}-12-01", YEAR), "%Y-%m-%d").unwrap();
    let default_day = max(
        0,
        min(25, now.signed_duration_since(start_of_month).num_days() + 1),
    );
    println!(
        "Enter a day number (1-25) or just press enter for default ({})",
        default_day,
    );
    let chosen_day = if let Ok(day) = stdin_lines.next().unwrap()?.parse::<i64>() {
        day
    } else {
        default_day
    };
    println!("{}", do_day_rust(chosen_day));
    println!("Press enter to quit");
    stdin_lines.next();
    Ok(())
}

fn do_day_rust<T: std::fmt::Display>(chosen_day: T) -> String {
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
        .arg("src/main.py")
        .arg(format!("{}", chosen_day))
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
fn do_day_elixir(chosen_day: usize) -> String {
    let output = Command::new("elixir.bat")
        .arg(format!("src/day{:02}/main.exs", chosen_day))
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
    let expected = format!("part 1: {}\npart 2: {}\n", p1, p2);
    if std::path::Path::new(&format!("src/day{:02}/main.rs", day)).exists() {
        println!("Testing Rust solution");
        assert_eq!(
            do_day_rust(day),
            expected,
            "Rust solution failed for day {}",
            day
        );
    }
    if std::path::Path::new(&format!("src/day{:02}/main.py", day)).exists() {
        println!("Testing Python solution");
        assert_eq!(
            do_day_python(day),
            expected,
            "Python solution failed for day {}",
            day
        );
    }
    if std::path::Path::new(&format!("src/day{:02}/main.exs", day)).exists() {
        println!("Testing Elixir solution");
        assert_eq!(
            do_day_elixir(day),
            expected,
            "Elixir solution failed for day {}",
            day
        );
    }
}
#[test]
fn day01() {
    do_test(1, 69310, 206104);
}
#[test]
fn day02() {
    do_test(2, 9651, 10560);
}
#[test]
fn day03() {
    do_test(3, 8123, 2620);
}
#[test]
fn day04() {
    do_test(4, 588, 911);
}
#[test]
fn day05() {
    do_test(5, "CFFHVVHNC", "FSZWBPTBG");
}
#[test]
fn day06() {
    do_test(6, 1531, 2518);
}
#[test]
fn day07() {
    do_test(7, 2104783, 5883165);
}
#[test]
fn day08() {
    do_test(8, 1763, 671160);
}
#[test]
fn day09() {
    do_test(9, 6271, 2458);
}
#[test]
fn day10() {
    const DAY10P2: &str = "
  [][][][]  [][][]      [][]    [][][]    []    []  [][][][]    [][]    []    []
  []        []    []  []    []  []    []  []    []  []        []    []  []    []
  [][][]    []    []  []        []    []  [][][][]  [][][]    []        [][][][]
  []        [][][]    []  [][]  [][][]    []    []  []        []  [][]  []    []
  []        []        []    []  []        []    []  []        []    []  []    []
  []        []          [][][]  []        []    []  []          [][][]  []    []";
    do_test(10, 10760, DAY10P2);
}
#[test]
fn day11() {
    do_test(11, 316888, 35270398814u64);
}
#[test]
fn day12() {
    do_test(12, 447, 446);
}
#[test]
fn day13() {
    do_test(13, 5588, 23958);
}
#[test]
fn day14() {
    do_test(14, 913, 30762);
}
#[test]
fn day15() {
    do_test(15, 6078701, 12567351400528u64);
}
#[test]
fn day16() {
    do_test(16, 2181, 2824);
}
#[test]
fn day17() {
    do_test(17, 3098, 1525364431487u64);
}
#[test]
fn day18() {
    do_test(18, 4288, 2494);
}
#[test]
fn day19() {
    do_test(19, 1395, 2700);
}
#[test]
fn day20() {
    do_test(20, 9866, 12374299815791u64);
}
#[test]
fn day21() {
    do_test(21, 158731561459602u64, 3769668716709u64);
}
#[test]
fn day22() {
    do_test(22, 13566, 11451);
}
#[test]
fn day23() {
    do_test(23, 4162, 986);
}
#[test]
fn day24() {
    do_test(24, 257, 828);
}
#[test]
fn day25() {
    do_test(25, "20=2-02-0---02=22=21", "");
}
