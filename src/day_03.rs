use std::collections::HashSet;

fn score(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => (c as u32) - 38,
        _ => 0,
    }
}

fn part_1(contents: String) -> u32 {
    contents
        .lines()
        .map(|line| {
            let (one, two) = line.split_at(line.len() / 2);

            let set_one: HashSet<char> = HashSet::from_iter(one.chars());
            let set_two: HashSet<char> = HashSet::from_iter(two.chars());

            set_one
                .intersection(&set_two)
                .map(|c: &char| score(*c))
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[test]
fn test_part_1() {
    let test_str: String = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        .to_string();

    assert_eq!(part_1(test_str), 157);
}

pub fn run(input: String) -> (u32, u32) {
    let part_1 = part_1(input);

    (part_1, 1)
}
