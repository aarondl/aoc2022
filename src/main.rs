#![allow(dead_code)]

use std::io::Write;

mod day01;
mod day02;
mod day03;
mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
mod helpers;

const SOLUTIONS: &'static [fn(String)] = &[
    day01::a,
    day01::b,
    day02::a,
    day02::b,
    day03::a,
    day03::b,
    day04::a,
    day04::b,
    // day05::a,
    // day05::b,
    // day06::a,
    // day06::b,
    // day07::a,
    // day07::b,
    // day08::a,
    // day08::b,
    // day09::a,
    // day09::b,
    // day10::a,
    // day10::b,
    // day11::a,
    // day11::b,
    // day12::a,
    // day12::b,
    // day13::a,
    // day13::b,
    // day14::a,
    // day14::b,
    // day15::a,
    // day15::b,
    // day16::a,
    // day16::b,
    // day17::a,
    // day17::b,
    // day18::a,
    // day18::b,
    // day19::a,
    // day19::b,
    // day20::a,
    // day20::b,
];

enum PuzzleHalf {
    Both,
    A,
    B,
}

fn main() {
    let Some((day, half)) = parse_args() else {
        return;
    };

    let data = read_input(day).expect("to be able to read input");

    let day_a_index = (day - 1) * 2;
    let day_b_index = (day - 1) * 2 + 1;
    match half {
        PuzzleHalf::A => {
            SOLUTIONS[day_a_index](data);
        }
        PuzzleHalf::B => {
            SOLUTIONS[day_b_index](data);
        }
        PuzzleHalf::Both => {
            SOLUTIONS[day_a_index](data.clone());
            SOLUTIONS[day_b_index](data);
        }
    }
}

fn parse_args() -> Option<(usize, PuzzleHalf)> {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("usage: aoc2022 <day|day+(a|b)>");
        return None;
    }
    let arg = args.nth(1).unwrap();

    if arg.ends_with('a') || arg.ends_with('b') {
        let num = &arg[0..arg.len() - 1];
        let half = match arg.chars().last().unwrap() {
            'A' | 'a' => PuzzleHalf::A,
            'B' | 'b' => PuzzleHalf::B,
            _ => panic!("invalid entry: {}", arg),
        };
        Some((num.parse::<usize>().unwrap(), half))
    } else {
        Some((arg.parse::<usize>().unwrap(), PuzzleHalf::Both))
    }
}

fn read_input(day: usize) -> std::io::Result<String> {
    if std::fs::metadata("inputs").is_err() {
        std::fs::create_dir_all("inputs").unwrap();
    }
    let path = format!("inputs/day{}.txt", day);

    if let Ok(b) = std::fs::read_to_string(&path) {
        println!("input cached");
        return Ok(b);
    }

    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    println!("input file not found, downloading: {}", url);

    let session_cookie_value = std::fs::read_to_string("session_cookie.dat")?;

    let session_cookie = ureq::Cookie::build("session", session_cookie_value.trim())
        .domain(".adventofcode.com")
        .http_only(true)
        .path("/")
        .secure(true)
        .finish();
    let resp = ureq::get(&url).set("Cookie", session_cookie.encoded().to_string().as_str()).call().unwrap();
    if resp.status() != 200 {
        panic!("there is a bad status while downloading: {}", resp.status());
    }

    let mut buf = Vec::<u8>::new();
    resp.into_reader().read_to_end(&mut buf)?;

    let mut file = std::fs::File::create(&path).expect("to be able to create file");
    file.write(&buf).unwrap();

    Ok(String::from_utf8(buf).expect("expect text to be utf8"))
}
