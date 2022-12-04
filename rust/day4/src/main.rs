use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day4.txt";

type Section = u32;
type Assignment = (Section, Section, Section, Section);

fn as_assignment(line: &str) -> Assignment {
    let sections = line
        .split(&[',', '-'])
        .map(|s| s.parse::<Section>().unwrap())
        .collect::<Vec<_>>();
    (sections[0], sections[1], sections[2], sections[3])
}

fn read_assignments() -> IOResult<Vec<Assignment>> {
    Ok(BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .map(|s| as_assignment(&s.unwrap()))
        .collect())
}

fn is_subsumed((start1, end1, start2, end2): &&Assignment) -> bool {
    (start1 >= start2 && end1 <= end2) || (start1 <= start2 && end1 >= end2)
}

fn is_overlapping((start1, end1, start2, end2): &&Assignment) -> bool {
    Section::max(*start1, *start2) <= Section::min(*end1, *end2)
}

fn main() -> IOResult<()> {
    let assigns = read_assignments()?;
    println!("Task 1: {}", assigns.iter().filter(is_subsumed).count());
    println!("Task 2: {}", assigns.iter().filter(is_overlapping).count());
    Ok(())
}
