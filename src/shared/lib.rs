#[macro_export]
macro_rules! tokenize {
    ($input:expr, $($i:expr),+) => {{
        use unicode_segmentation::UnicodeSegmentation;
        let words: Vec<_> = $input.split_word_bounds().filter(|s| !s.trim().is_empty()).collect();
        (
            $(words[$i]),+
        )
    }}
}

#[macro_export]
macro_rules! words {
    ($input:expr, $($i:expr),+) => {{
        use unicode_segmentation::UnicodeSegmentation;
        let words: Vec<_> = $input.unicode_words().filter(|s| !s.trim().is_empty()).collect();
        (
            $(words[$i]),+
        )
    }}
}

#[macro_export]
macro_rules! puzzle_input {
    () => {{
        const year: i32 = 2022;
        let mod_name = module_path!().rsplit("::").next().unwrap();
        let day_num = (&mod_name[3..5])
            .parse::<u32>()
            .expect(&format!("something went wrong parsing \"{}\"", mod_name));
        let x = shared::puzzle_input(year, day_num).replace("\r\n", "\n");
        x.trim().to_string()
    }};
}

pub fn puzzle_input(year: i32, day_num: u32) -> String {
    use std::io::Write;
    let input_path = format!("puzzle_input/day{:02}", day_num);
    if std::fs::metadata(&input_path).is_ok() {
        eprintln!("puzzle input already downloaded for day {}", day_num);
    } else {
        use chrono::prelude::{TimeZone, Utc};
        if let std::cmp::Ordering::Less =
            Utc::now().cmp(&Utc.with_ymd_and_hms(year, 12, day_num, 0, 0, 0).unwrap())
        {
            panic!("too early for fetching puzzle input!");
        }
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day_num);
        eprintln!("downloading {:?}", url);
        let session_cookie =
            std::fs::read_to_string("session_cookie").expect("error obtaining session cookie");
        let contents = reqwest::blocking::Client::new()
            .get(&url)
            .header("Cookie", format!("session={}", session_cookie))
            .send()
            .expect("error sending puzzle input request")
            .text()
            .expect("error converting response to text");
        eprintln!("done");
        write!(
            std::fs::File::create(&input_path).expect("error creating puzzle input file"),
            "{}",
            contents
        )
        .expect("error writing puzzle input file");
    }
    std::fs::read_to_string(&input_path).expect("error opening puzzle input")
}
