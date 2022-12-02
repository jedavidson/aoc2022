#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

pub type Round = (Shape, Shape);
pub type Instruction = (Shape, Outcome);
pub type Score = u32;

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!("???"),
        }
    }
}

impl From<Shape> for Outcome {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Rock => Outcome::Lose,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Win,
        }
    }
}

pub fn get_score(round: Round) -> Score {
    match round {
        (Shape::Rock, Shape::Rock) => 3 + 1,
        (Shape::Rock, Shape::Paper) => 6 + 2,
        (Shape::Rock, Shape::Scissors) => 0 + 3,
        (Shape::Paper, Shape::Rock) => 0 + 1,
        (Shape::Paper, Shape::Paper) => 3 + 2,
        (Shape::Paper, Shape::Scissors) => 6 + 3,
        (Shape::Scissors, Shape::Rock) => 6 + 1,
        (Shape::Scissors, Shape::Paper) => 0 + 2,
        (Shape::Scissors, Shape::Scissors) => 3 + 3,
    }
}

pub fn play_instruction(inst: Instruction) -> Score {
    match inst {
        (Shape::Rock, Outcome::Lose) => 0 + 3,
        (Shape::Rock, Outcome::Draw) => 3 + 1,
        (Shape::Rock, Outcome::Win) => 6 + 2,
        (Shape::Paper, Outcome::Lose) => 0 + 1,
        (Shape::Paper, Outcome::Draw) => 3 + 2,
        (Shape::Paper, Outcome::Win) => 6 + 3,
        (Shape::Scissors, Outcome::Lose) => 0 + 2,
        (Shape::Scissors, Outcome::Draw) => 3 + 3,
        (Shape::Scissors, Outcome::Win) => 6 + 1,
    }
}
