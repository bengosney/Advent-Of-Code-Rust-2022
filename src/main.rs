mod day_01;
mod day_02;
mod day_03;
mod day_04;

use std::env;
use std::fs;

fn run_day(day: u8, input: String) -> (u32, u32) {
    match day {
        1 => day_01::run(input),
        2 => day_02::run(input),
        3 => day_03::run(input),
        4 => day_04::run(input),
        _ => panic!("Not implemented yet!"),
    }
}

fn get_day(args: Vec<String>) -> Option<Vec<u8>> {
    match args.get(1) {
        Some(d) => match d.parse::<u8>().ok() {
            Some(day) => Some(vec![day]),
            None => None,
        },
        None => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<u8> = match get_day(args) {
        Some(days) => days,
        None => vec![1, 2, 3, 4],
    };

    days.iter().for_each(|day| {
        let file_path = format!("inputs/day_{day:0>2}.txt");
        let contents = fs::read_to_string(file_path).expect("Input not found!");

        let (answer_1, answer_2) = run_day(*day, contents);

        println!("Day {day}");
        println!("Part 1: {answer_1}");
        println!("Part 2: {answer_2}");
        println!()
    });
}
