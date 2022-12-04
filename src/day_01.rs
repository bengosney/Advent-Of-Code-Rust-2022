fn part_1(contents: &String) -> u32 {}

fn part_2(contents: &String) -> u32 {}

#[allow(dead_code)]
fn get_test_string() -> String {
    "".to_string().clone()
}

#[test]
fn test_part_1() {
    let test_str: String = get_test_string();

    assert_eq!(part_1(&test_str), 0);
}

#[test]
fn test_part_2() {
    let test_str: String = get_test_string();

    assert_eq!(part_2(&test_str), 0);
}

pub fn run(input: String) -> (u32, u32) {
    let part_1: u32 = part_1(&input);
    let part_2: u32 = part_2(&input);

    (part_1, part_2)
}
