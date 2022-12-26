use shared::puzzle_input;

fn parse_snafu(s: &str) -> i64 {
    s.chars().fold(0, |n, c| {
        let x = match c {
            '-' => -1,
            '=' => -2,
            c => c.to_digit(10).unwrap() as i64,
        };
        n * 5 + x
    })
}
fn to_snafu(mut x: i64) -> String {
    let mut s = String::new();
    while x != 0 {
        match x % 5 {
            0 | 1 | 2 => {
                s.push(char::from_digit((x % 5) as u32, 3).unwrap());
                x /= 5;
            }
            3 => {
                s.push('=');
                x = (x / 5) + 1;
            }
            4 => {
                s.push('-');
                x = (x / 5) + 1;
            }
            _ => unreachable!(),
        }
    }
    s.chars().rev().collect()
}

pub fn main() {
    let input = puzzle_input!();
    let p1 = to_snafu(input.split('\n').map(parse_snafu).sum());
    println!("part 1: {}\npart 2: ", p1);
}

#[cfg(test)]
const EXAMPLE_SNAFU: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

#[cfg(test)]
const EXAMPLE_DEC: [i64; 13] = [1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37];

#[test]
fn test_from_snafu() {
    use itertools::Itertools;

    assert_eq!(10, parse_snafu("20"));
    assert_eq!(to_snafu(10), "20");
    assert_eq!(976, parse_snafu("2=-01"));
    assert_eq!(to_snafu(976), "2=-01");
    for (dec, snafu) in [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ] {
        assert_eq!(dec, parse_snafu(snafu));
        assert_eq!(to_snafu(dec), snafu);
    }
    let parsed_example = EXAMPLE_SNAFU.split('\n').map(parse_snafu).collect_vec();
    assert_eq!(parsed_example, EXAMPLE_DEC);

    for (converted, example) in EXAMPLE_DEC
        .iter()
        .cloned()
        .map(to_snafu)
        .zip(EXAMPLE_SNAFU.split('\n'))
    {
        assert_eq!(converted, example);
    }

    assert_eq!(4890i64, parsed_example.iter().sum());
    assert_eq!(to_snafu(4890), "2=-1=0");
}
