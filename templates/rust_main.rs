use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/dayX.txt";

fn read_input() -> IOResult<()> {
    BufReader::new(File::open(INPUT_PATH)?).lines();
    Ok(())
}

fn task1() {
    // ...
}

fn task2() {
    // ...
}

fn main() -> IOResult<()> {
    let _ = read_input()?;
    println!("Task 1: {:?}", task1());
    println!("Task 2: {:?}", task2());
    Ok(())
}
