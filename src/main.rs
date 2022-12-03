use std::collections::HashSet;
use std::fs;

fn score(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => (c as u32) - 38,
        _ => 0,
    }
}

fn part_1(contents: String) -> u32 {
    return contents
        .lines()
        .map(|line| {
            let (one, two) = line.split_at(line.len() / 2);

            let set_one: HashSet<char> = HashSet::from_iter(one.chars());
            let set_two: HashSet<char> = HashSet::from_iter(two.chars());

            return set_one
                .intersection(&set_two)
                .map(|c: &char| score(*c))
                .sum::<u32>();
        })
        .sum::<u32>();
}

#[test]
fn test_part_1() {
    let test_str = String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );

    println!("========================================================");
    let ans = part_1(test_str);
    println!("========================================================");
    println!("{}", ans);
    assert_eq!(ans, 157);
}

fn main() {
    let file_path = "inputs/day_03.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let answer_1 = part_1(contents);
    println!("With text:\n{answer_1}");
}
