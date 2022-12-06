use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day6.txt";

const START_OF_PACKET_MARKER_SIZE: usize = 4;
const START_OF_MESSAGE_MARKER_SIZE: usize = 14;

fn read_datastream() -> IOResult<String> {
    Ok(BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .flat_map(|line| line)
        .next()
        .expect("Input must contain a line"))
}

fn first_marker_pos(datastream: &str, marker_size: usize) -> usize {
    let (pos, _) = datastream
        .as_bytes()
        .windows(marker_size)
        .enumerate()
        .find(|(_, window)| !(1..marker_size).any(|i| window[i..].contains(&window[i - 1])))
        .expect("There's an answer to this problem ... right?");
    pos + marker_size
}

fn main() -> IOResult<()> {
    let datastream = read_datastream()?;
    println!(
        "Task 1: {}",
        first_marker_pos(&datastream, START_OF_PACKET_MARKER_SIZE)
    );
    println!(
        "Task 2: {}",
        first_marker_pos(&datastream, START_OF_MESSAGE_MARKER_SIZE)
    );
    Ok(())
}
