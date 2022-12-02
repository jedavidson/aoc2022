mod types;

use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};
use types::*;

const INPUT_PATH: &str = "../inputs/day2.txt";

fn read_rounds() -> IOResult<Vec<Round>> {
    Ok(BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .map(|line| {
            line.expect("Lines are formatted properly")
                .split_whitespace()
                .map(Shape::from)
                .collect::<Vec<_>>()
        })
        .map(|round_vec| (round_vec[0], round_vec[1]))
        .collect())
}

fn total_score_faceoff(rounds: &Vec<Round>) -> Score {
    rounds
        .iter()
        .fold(0, |score, &round| score + get_score(round))
}

fn total_score_strategy(rounds: &Vec<Round>) -> Score {
    rounds
        .iter()
        .map(|&(them, you)| (them, Outcome::from(you)))
        .fold(0, |score, inst| score + play_instruction(inst))
}

fn main() -> IOResult<()> {
    let rounds = read_rounds()?;
    println!("Task 1: {}", total_score_faceoff(&rounds));
    println!("Task 2: {}", total_score_strategy(&rounds));
    Ok(())
}
