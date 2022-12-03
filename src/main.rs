use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(PartialEq, Copy, Clone)]
enum MoveType {
    Rock,
    Paper,
    Scissors,
}

impl MoveType {
    fn get_value(self) -> u8 {
        match self {
            MoveType::Rock => 1,
            MoveType::Paper => 2,
            MoveType::Scissors => 3,
        }
    }

    fn get_beating_move(self) -> Self {
        match self {
            MoveType::Rock => MoveType::Paper,
            MoveType::Paper => MoveType::Scissors,
            MoveType::Scissors => MoveType::Rock,
        }
    }

    fn get_losing_move(self) -> Self {
        match self {
            MoveType::Rock => MoveType::Scissors,
            MoveType::Paper => MoveType::Rock,
            MoveType::Scissors => MoveType::Paper,
        }
    }
}

enum MoveOutcome {
    Win,
    Loss,
    Draw,
}

impl MoveOutcome {
    fn get_value(self) -> u8 {
        match self {
            MoveOutcome::Win => 6,
            MoveOutcome::Draw => 3,
            MoveOutcome::Loss => 0,
        }
    }
}

fn get_input() -> Lines<BufReader<File>> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn get_moves<T, F>(input: Lines<BufReader<File>>, provided_strategy: F) -> Vec<(MoveType, T)>
where
    F: Fn(char) -> T,
{
    let mut result = Vec::new();

    for current_line in input {
        let Ok(current_line) = current_line else {
            panic!("Invalid input, expected Ok");
        };

        let chars = current_line.chars().take(3).collect::<Vec<char>>();

        let prompt_move = match chars[0] {
            'A' => MoveType::Rock,
            'B' => MoveType::Paper,
            'C' => MoveType::Scissors,
            x => panic!("Invalid character input, expected 'A' 'B' or 'C', instead got {x}"),
        };
        let response_move = provided_strategy(chars[2]);

        result.push((prompt_move, response_move));
    }

    result
}

fn play_move(prompt_move: MoveType, response_move: MoveType) -> MoveOutcome {
    match (prompt_move, response_move) {
        (x, y) if x == y => MoveOutcome::Draw,
        (x, y) if y == x.get_beating_move() => MoveOutcome::Win,
        _ => MoveOutcome::Loss,
    }
}

fn score_moves(moves: Vec<(MoveType, MoveType)>) -> u64 {
    let mut score = 0;

    for current_move in moves {
        let (prompt_move, response_move) = current_move;
        score +=
            (response_move.get_value() + play_move(prompt_move, response_move).get_value()) as u64;
    }

    score
}

fn puzzle_2() {
    let moves = get_moves(get_input(), |c| match c {
        'X' => MoveOutcome::Loss,
        'Y' => MoveOutcome::Draw,
        'Z' => MoveOutcome::Win,
        x => panic!("Invalid character input, expected 'X' 'Y' or 'Z', instead got {x}"),
    });

    let moves = moves
        .into_iter()
        .map(|(x, y)| {
            (x, {
                match y {
                    MoveOutcome::Draw => x,
                    MoveOutcome::Win => x.get_beating_move(),
                    MoveOutcome::Loss => x.get_losing_move(),
                }
            })
        })
        .collect();

    println!("{}", score_moves(moves));
}

fn puzzle_1() {
    let moves = get_moves(get_input(), |c| match c {
        'X' => MoveType::Rock,
        'Y' => MoveType::Paper,
        'Z' => MoveType::Scissors,
        x => panic!("Invalid character input, expected 'X' 'Y' or 'Z', instead got {x}"),
    });

    println!("{}", score_moves(moves))
}

fn main() {
    puzzle_1();
    puzzle_2();
}
