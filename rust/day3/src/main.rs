use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day3.txt";

type Item = char;
type Rucksack = String;
type Priority = u32;

fn read_input() -> IOResult<Vec<Rucksack>> {
    Ok(BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .flat_map(|r| r)
        .collect::<Vec<_>>())
}

fn as_item_set(rucksack: &str) -> HashSet<Item> {
    rucksack.chars().collect::<HashSet<_>>()
}

macro_rules! intersection {
    ($x:expr) => { as_item_set($x) };
    ($x:expr, $($xs:expr),+) => { &as_item_set($x) & &(intersection!($($xs),+)) }
}

fn get_priority(item: Item) -> Priority {
    item as u32
        - match &item {
            'a'..='z' => 'a' as u32 - 1,
            'A'..='Z' => 'A' as u32 - 27,
            _ => unreachable!("Bad item"),
        }
}

fn sum_common_item_priorities(rucksacks: &Vec<Rucksack>) -> Priority {
    rucksacks
        .iter()
        .map(|r| {
            let (fst, snd) = r.split_at(r.len() / 2);
            let common_item = intersection!(fst, snd)
                .iter()
                .next()
                .expect("At least one item is common to both compartments")
                .to_owned();
            get_priority(common_item)
        })
        .sum()
}

fn sum_badge_priorities(rucksacks: &Vec<Rucksack>) -> Priority {
    rucksacks
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let [fst, snd, thd] = chunk else { return 0; };
            let common_item = intersection!(fst, snd, thd)
                .iter()
                .next()
                .expect("Every elf in each group has one item in common")
                .to_owned();
            get_priority(common_item)
        })
        .sum()
}

fn main() -> IOResult<()> {
    let rucksacks = read_input()?;
    println!("Task 1: {:?}", sum_common_item_priorities(&rucksacks));
    println!("Task 2: {:?}", sum_badge_priorities(&rucksacks));
    Ok(())
}
