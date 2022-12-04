use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day4.txt";

type Section = u32;
type Assignment = (Section, Section);
type Pair = (Assignment, Assignment);

fn as_pair(line: &str) -> Pair {
    let mut pairs = line.split(',').map(|s| {
        let mut sections = s.split('-').map(|n| n.parse::<Section>().unwrap());
        (sections.next().unwrap(), sections.next().unwrap())
    });
    (pairs.next().unwrap(), pairs.next().unwrap())
}

fn read_pairs() -> IOResult<Vec<Pair>> {
    Ok(BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .flat_map(|line| line)
        .map(|s| as_pair(&s))
        .collect())
}

fn is_subsumed(((start1, end1), (start2, end2)): &&Pair) -> bool {
    (start1 >= start2 && end1 <= end2) || (start1 <= start2 && end1 >= end2)
}

fn is_overlapping(((start1, end1), (start2, end2)): &&Pair) -> bool {
    Section::max(*start1, *start2) <= Section::min(*end1, *end2)
}

fn count_satisfying_pairs(pairs: &Vec<Pair>, pred: impl FnMut(&&Pair) -> bool) -> usize {
    pairs.iter().filter(pred).count()
}

fn main() -> IOResult<()> {
    let pairs = read_pairs()?;
    println!("Task 1: {}", count_satisfying_pairs(&pairs, is_subsumed));
    println!("Task 2: {}", count_satisfying_pairs(&pairs, is_overlapping));
    Ok(())
}
