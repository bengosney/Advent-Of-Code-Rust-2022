use std::collections::HashSet;

fn score(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => (c as u32) - 38,
        _ => 0,
    }
}

fn part_1(contents: &String) -> u32 {
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

    assert_eq!(part_1(&test_str), 157);
}

fn part_2(contents: &String) -> u32 {
    let mut total = 0;
    for lines in Vec::from_iter(contents.lines()).chunks(3) {
        let one: HashSet<char> = HashSet::from_iter(lines[0].chars());
        let two: HashSet<char> = HashSet::from_iter(lines[1].chars());
        let three: HashSet<char> = HashSet::from_iter(lines[2].chars());

        let inter_hash: HashSet<char> =
            HashSet::from_iter(one.intersection(&two).map(|c| c.clone()));

        total += inter_hash
            .intersection(&three)
            .map(|c: &char| score(*c))
            .sum::<u32>();
    }

    return total;
}

#[test]
fn test_part_2() {
    let test_str: String = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        .to_string();

    assert_eq!(part_2(&test_str), 70);
}

pub fn run(input: String) -> (u32, u32) {
    let part_1: u32 = part_1(&input);
    let part_2: u32 = part_2(&input);

    (part_1, part_2)
}
