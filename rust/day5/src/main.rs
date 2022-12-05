mod types;

use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};
use types::*;

const INPUT_PATH: &str = "../inputs/day5.txt";

fn read_input() -> IOResult<(Vec<CrateStack>, Vec<Instruction>)> {
    let lines = BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .flat_map(|line| line)
        .collect::<Vec<_>>();

    let num_stacks = lines
        .iter()
        .filter(|line| line.starts_with(" 1"))
        .next()
        .expect("Last line of crate display is missing")
        .split(' ')
        .last()
        .map(|stack_no| stack_no.parse::<usize>().expect("Bad stack number"))
        .expect("Missing stack numbers");

    let mut crate_stacks = vec![vec![]; num_stacks];
    for line in lines
        .iter()
        .enumerate()
        .filter(|(idx, line)| *idx == 0 || line.starts_with('['))
        .map(|(_, line)| line)
        .rev()
    {
        for stack_no in 0..num_stacks {
            let Some(crate_label) = line.chars().nth(1 + 4 * stack_no) else {
                continue
            };
            if crate_label != ' ' {
                crate_stacks[stack_no].push(crate_label);
            }
        }
    }

    let instructions = lines
        .iter()
        .filter(|line| line.starts_with("move"))
        .map(|line| Instruction::from(line.as_str()))
        .collect::<Vec<_>>();

    Ok((crate_stacks, instructions))
}

fn rearrange_crates(
    crate_stacks: &mut Vec<CrateStack>,
    instructions: &Vec<Instruction>,
    crane_mode: CraneMode,
) {
    for Instruction { amount, from, to } in instructions {
        let crates_to_move = {
            let crates = crate_stacks[*from]
                .iter()
                .rev()
                .take(*amount)
                .map(CrateLabel::to_owned);
            match crane_mode {
                CraneMode::Sequential => crates.collect::<Vec<_>>(),
                CraneMode::AllAtOnce => crates.rev().collect::<Vec<_>>(),
            }
        };

        for crate_label in crates_to_move {
            crate_stacks[*from].pop();
            crate_stacks[*to].push(crate_label);
        }
    }
}

fn get_top_crate_message(crate_stacks: &Vec<CrateStack>) -> String {
    crate_stacks
        .iter()
        .filter(|crate_stack| !crate_stack.is_empty())
        .map(|crate_stack| crate_stack.last().unwrap())
        .collect()
}

fn main() -> IOResult<()> {
    let (mut crate_stacks, instructions) = read_input()?;
    let mut crate_stacks_copy = crate_stacks.clone();

    rearrange_crates(&mut crate_stacks, &instructions, CraneMode::Sequential);
    println!("Task 1: {}", get_top_crate_message(&crate_stacks));

    rearrange_crates(&mut crate_stacks_copy, &instructions, CraneMode::AllAtOnce);
    println!("Task 2: {}", get_top_crate_message(&crate_stacks_copy));

    Ok(())
}
