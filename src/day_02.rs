enum Moves {
    Rock,
    Paper,
    Scissors,
}

struct Game(Moves, Moves);

fn parse_move(c: &str) -> Moves {
    match c {
        "A" | "X" => Moves::Rock,
        "B" | "Y" => Moves::Paper,
        "C" | "Z" => Moves::Scissors,
        _ => panic!("Unknown Move: {c}"),
    }
}

fn parse_row(row: &str) -> Game {
    let (them, you) = match row.split_once(" ") {
        Some(m) => m,
        None => panic!("Invalid game state"),
    };

    Game(parse_move(them), parse_move(you))
}

fn score(game: &Game) -> u32 {
    match game {
        Game(_, Moves::Rock) => 1,
        Game(_, Moves::Paper) => 2,
        Game(_, Moves::Scissors) => 3,
    }
}

fn play(game: Game) -> u32 {
    let s = match game {
        Game(Moves::Rock, Moves::Scissors) => 6,
        Game(Moves::Paper, Moves::Rock) => 6,
        Game(Moves::Scissors, Moves::Paper) => 6,
        Game(Moves::Rock, Moves::Rock) => 3,
        Game(Moves::Paper, Moves::Paper) => 3,
        Game(Moves::Scissors, Moves::Scissors) => 3,
        _ => 0,
    };

    score(&game) + s
}

fn win(m: Moves) -> Game {
    match m {
        Moves::Rock => Game(Moves::Rock, Moves::Paper),
        Moves::Paper => Game(Moves::Paper, Moves::Scissors),
        Moves::Scissors => Game(Moves::Scissors, Moves::Rock),
    }
}

fn loose(m: Moves) -> Game {
    match m {
        Moves::Rock => Game(Moves::Rock, Moves::Scissors),
        Moves::Paper => Game(Moves::Paper, Moves::Rock),
        Moves::Scissors => Game(Moves::Scissors, Moves::Rock),
    }
}

fn draw(m: Moves) -> Game {
    match m {
        Moves::Rock => Game(Moves::Rock, Moves::Rock),
        Moves::Paper => Game(Moves::Paper, Moves::Paper),
        Moves::Scissors => Game(Moves::Scissors, Moves::Scissors),
    }
}

fn fix(game: Game) -> u32 {
    match game {
        Game(them, Moves::Rock) => play(loose(them)),
        Game(them, Moves::Paper) => play(draw(them)),
        Game(them, Moves::Scissors) => play(win(them)),
        _ => 0,
    }
}

fn part_1(contents: &String) -> u32 {
    contents.lines().map(parse_row).map(play).sum::<u32>()
}

fn part_2(contents: &String) -> u32 {
    contents.lines().map(parse_row).map(fix).sum::<u32>()
}

#[allow(dead_code)]
fn get_test_string() -> String {
    "A Y
B X
C Z"
    .to_string()
    .clone()
}

#[test]
fn test_part_1() {
    let test_str: String = get_test_string();

    assert_eq!(part_1(&test_str), 15);
}

#[test]
fn test_part_2() {
    let test_str: String = get_test_string();

    assert_eq!(part_2(&test_str), 12);
}

pub fn run(input: String) -> (u32, u32) {
    let part_1: u32 = part_1(&input);
    let part_2: u32 = part_2(&input);

    (part_1, part_2)
}
