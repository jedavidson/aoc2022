use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day10.txt";

const INTERESTING_CYCLES: [i64; 6] = [20, 60, 100, 140, 180, 220];
const CRT_ROWS: i64 = 6;
const CRT_COLS: i64 = 40;
const CRT_CYCLES: i64 = CRT_ROWS * CRT_COLS;

#[derive(Copy, Clone)]
pub enum Instruction {
    NoOp,
    AddX(i32),
}

impl From<String> for Instruction {
    fn from(inst: String) -> Self {
        if inst == "noop" {
            return Instruction::NoOp;
        } else if inst.starts_with("addx") {
            let amt = inst
                .split_ascii_whitespace()
                .nth(1)
                .expect("addx instruction takes an argument")
                .parse::<i32>()
                .expect("addx argument is a valid integer");
            return Instruction::AddX(amt);
        }

        unreachable!("Bad instruction in input")
    }
}

fn read_instructions() -> IOResult<Vec<Instruction>> {
    Ok(BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .flat_map(|line| line)
        .map(Instruction::from)
        .collect())
}

fn simulate_cpu(instructions: &Vec<Instruction>) {
    let mut x = 1;
    let mut crt = [[' '; CRT_COLS as usize]; CRT_ROWS as usize];

    let mut signal_sum = 0;

    let mut cycle = 1;
    let mut inst_no = 0;
    let mut executing_inst = None;
    while inst_no < instructions.len() || executing_inst.is_some() || cycle <= CRT_CYCLES {
        if executing_inst.is_none() {
            match instructions[inst_no] {
                Instruction::NoOp => {}
                addx @ Instruction::AddX(_) => executing_inst = Some((cycle + 1, addx)),
            }
            inst_no += 1;
        }

        let (crt_row, crt_col) = ((cycle - 1) / CRT_COLS, (cycle - 1) % CRT_COLS);
        if (x - 1..=x + 1).contains(&crt_col) {
            crt[crt_row as usize][crt_col as usize] = '#';
        }

        if INTERESTING_CYCLES.contains(&cycle) {
            signal_sum += cycle * x;
        }

        if let Some((cycle_completed, Instruction::AddX(amt))) = executing_inst {
            if cycle == cycle_completed {
                x += amt as i64;
                executing_inst = None;
            }
        }

        cycle += 1;
    }

    println!("Task 1: {signal_sum}");
    println!("Task 2: final CRT state is");
    for row in 0..CRT_ROWS {
        for col in 0..CRT_COLS {
            print!("{}", crt[row as usize][col as usize]);
        }
        print!("\n");
    }
}

fn main() -> IOResult<()> {
    let mut instructions = read_instructions()?;
    Ok(simulate_cpu(&mut instructions))
}
