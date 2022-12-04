struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }
}

struct Pair {
    left: Range,
    right: Range,
}

impl Pair {
    fn contains(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn overlaps(&self) -> bool {
        self.left.overlaps(&self.right) || self.right.overlaps(&self.left)
    }
}

fn parse_str(string: &str) -> u32 {
    match string.parse::<u32>().ok() {
        Some(v) => v,
        None => 0,
    }
}

fn parse_range(range: &str) -> Range {
    let (start, end) = match range.split_once("-") {
        Some(r) => r,
        None => panic!("Invalid range"),
    };

    Range {
        start: parse_str(start),
        end: parse_str(end),
    }
}

fn parse_row(row: &str) -> Pair {
    let (left, right) = match row.split_once(",") {
        Some(p) => p,
        None => panic!("Invalid Row"),
    };

    return Pair {
        left: parse_range(left),
        right: parse_range(right),
    };
}

fn part_1(contents: &String) -> u32 {
    contents
        .lines()
        .map(parse_row)
        .map(|i| match i.contains() {
            true => 1,
            false => 0,
        })
        .sum::<u32>()
}

fn part_2(contents: &String) -> u32 {
    contents
        .lines()
        .map(parse_row)
        .map(|i| match i.overlaps() {
            true => 1,
            false => 0,
        })
        .sum::<u32>()
}

#[allow(dead_code)]
fn get_test_string() -> String {
    "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8"
        .to_string()
        .clone()
}

#[test]
fn test_part_1() {
    let test_str: String = get_test_string();

    assert_eq!(part_1(&test_str), 2);
}

#[test]
fn test_part_2() {
    let test_str: String = get_test_string();

    assert_eq!(part_2(&test_str), 4);
}

pub fn run(input: String) -> (u32, u32) {
    let part_1: u32 = part_1(&input);
    let part_2: u32 = part_2(&input);

    (part_1, part_2)
}
