mod day_03;
use std::fs;

fn run_day(day: u8, input: String) -> (u32, u32) {
    match day {
        3 => day_03::run(input),
        _ => panic!("Not implemented yet!"),
    }
}

fn main() {
    let day: u8 = 3;

    let file_path = format!("inputs/day_{day:0>2}.txt");
    let contents = fs::read_to_string(file_path).expect("Input not found!");

    let (answer_1, _) = run_day(day as u8, contents);
    println!("{answer_1}");
}
