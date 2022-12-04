fn part_1(contents: &String) -> u32 {
    contents
        .split("\n\n")
        .map(|e| e.lines().filter_map(|c| c.parse::<u32>().ok()).sum::<u32>())
        .max()
        .expect("No max found!")
}

fn part_2(contents: &String) -> u32 {
    let mut totals = contents
        .split("\n\n")
        .map(|e| e.lines().filter_map(|c| c.parse::<u32>().ok()).sum::<u32>())
        .collect::<Vec<u32>>();
    totals.sort();

    totals.into_iter().rev().take(3).sum::<u32>()
}

#[allow(dead_code)]
fn get_test_string() -> String {
    "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        .to_string()
        .clone()
}

#[test]
fn test_part_1() {
    let test_str: String = get_test_string();

    assert_eq!(part_1(&test_str), 24000);
}

#[test]
fn test_part_2() {
    let test_str: String = get_test_string();

    assert_eq!(part_2(&test_str), 45000);
}

pub fn run(input: String) -> (u32, u32) {
    let part_1: u32 = part_1(&input);
    let part_2: u32 = part_2(&input);

    (part_1, part_2)
}
