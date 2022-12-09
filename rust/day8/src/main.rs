use std::fs::File;
use std::io::{BufRead, BufReader, Result as IOResult};

const INPUT_PATH: &str = "../inputs/day8.txt";

type Tree = u32;
type Forest = Vec<Vec<Tree>>;
type ScenicScore = usize;

fn read_forest() -> IOResult<Forest> {
    Ok(BufReader::new(File::open(INPUT_PATH)?)
        .lines()
        .flat_map(|line| line)
        .map(|line| {
            line.chars()
                .map(|t| t.to_digit(10).expect("Line only contains numbers"))
                .collect()
        })
        .collect())
}

fn is_visible(forest: &Forest, (row, col): (usize, usize)) -> bool {
    let (height, width) = (forest.len(), forest[0].len());

    let from_left = (0..col).all(|c| forest[row][c] < forest[row][col]);
    let from_right = (col + 1..width).all(|c| forest[row][c] < forest[row][col]);
    let from_top = (0..row).all(|r| forest[r][col] < forest[row][col]);
    let from_bottom = (row + 1..height).all(|r| forest[r][col] < forest[row][col]);

    from_left || from_right || from_top || from_bottom
}

fn scenic_score(forest: &Forest, (row, col): (usize, usize)) -> ScenicScore {
    let (height, width) = (forest.len(), forest[0].len());

    let from_left = col
        - (0..col)
            .rev()
            .find(|c| forest[row][*c] >= forest[row][col])
            .unwrap_or(0);
    let from_right = (col + 1..width)
        .find(|c| forest[row][*c] >= forest[row][col])
        .unwrap_or(width - 1)
        - col;
    let from_top = row
        - (0..row)
            .rev()
            .find(|r| forest[*r][col] >= forest[row][col])
            .unwrap_or(0);
    let from_bottom = (row + 1..height)
        .find(|r| forest[*r][col] >= forest[row][col])
        .unwrap_or(height - 1)
        - row;

    from_left * from_right * from_top * from_bottom
}

fn count_visible_trees(forest: &Forest) -> usize {
    let (height, width) = (forest.len(), forest[0].len());
    (0..height).fold(0, |row_acc, row| {
        row_acc
            + (0..width).fold(0, |col_acc, col| {
                col_acc + is_visible(&forest, (row, col)) as usize
            })
    })
}

fn max_scenic_score(forest: &Forest) -> ScenicScore {
    let (height, width) = (forest.len(), forest[0].len());
    (0..height)
        .reduce(|row_max, row| {
            (0..width)
                .reduce(|col_max, col| scenic_score(forest, (row, col)).max(col_max))
                .expect("Column has a maximal scenic score")
                .max(row_max)
        })
        .expect("Forest has a maximal scenic score")
}

fn main() -> IOResult<()> {
    let forest = read_forest()?;
    assert!(
        forest.iter().all(|row| row.len() == forest[0].len()),
        "Forest rows are not of the same size"
    );

    println!("Task 1: {}", count_visible_trees(&forest));
    println!("Task 2: {}", max_scenic_score(&forest));
    Ok(())
}
