#![feature(binary_heap_drain_sorted)]

use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day1.txt";

type Calories = u64;

fn read_calories() -> IOResult<BinaryHeap<Calories>> {
    let mut total_cals = BinaryHeap::new();
    let mut curr_cals = 0;

    for line in BufReader::new(File::open(INPUT_PATH)?).lines() {
        match line?.as_str() {
            "" => {
                total_cals.push(curr_cals);
                curr_cals = 0;
            }
            cals => curr_cals += cals.parse::<Calories>().expect("Calories are valid"),
        }
    }

    Ok(total_cals)
}

fn most_calories_carried(total_cals: &BinaryHeap<Calories>) -> Calories {
    *total_cals.iter().max().expect("There is at least one elf")
}

fn calories_carried_by_top_3_elves(total_cals: &mut BinaryHeap<Calories>) -> Calories {
    total_cals.drain_sorted().take(3).sum()
}

fn main() -> IOResult<()> {
    let mut total_cals = read_calories()?;
    println!("Task 1: {}", most_calories_carried(&total_cals));
    println!(
        "Task 2: {}",
        calories_carried_by_top_3_elves(&mut total_cals)
    );

    Ok(())
}
