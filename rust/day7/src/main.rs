mod file_system;
mod output;

use file_system::FileSystem;
use output::OutputLine;
use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day7.txt";

const DISK_SIZE: usize = 70_000_000;
const DISK_SPACE_NEEDED: usize = 30_000_000;

fn read_fs() -> IOResult<FileSystem> {
    let cmd_output = BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .skip(1) // ignore initial `cd /`
        .flat_map(|line| line)
        .map(OutputLine::try_from)
        .flat_map(|output| output)
        .collect::<Vec<_>>();
    Ok(FileSystem::from(cmd_output))
}

fn total_directory_size_under_100k(fs: &FileSystem) -> usize {
    fs.fold_dirs(0, |total_size, dir| {
        if dir.size <= 100_000 {
            total_size + dir.size
        } else {
            total_size
        }
    })
}

fn size_of_smallest_viable_directory(fs: &FileSystem) -> usize {
    let available_space = DISK_SIZE - fs.size();

    fs.fold_dirs(fs.size(), |min_size, dir| {
        if available_space + dir.size >= DISK_SPACE_NEEDED {
            min_size.min(dir.size)
        } else {
            min_size
        }
    })
}

fn main() -> IOResult<()> {
    let fs = read_fs()?;
    println!("Task 1: {}", total_directory_size_under_100k(&fs));
    println!("Task 2: {}", size_of_smallest_viable_directory(&fs));
    Ok(())
}
