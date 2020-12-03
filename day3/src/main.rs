use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn match_tree(l: &str, curr_column:usize) -> bool {
    if l.chars().nth(curr_column%l.len()).unwrap() == '#' {
        return true
    }
    return false
}

fn slide(lines: &Vec<String>, column_step:usize, row_step:usize) -> usize {
    let mut trees = 0;
    let mut column = 0;
    for row in (0..lines.len()).step_by(row_step) {
        if match_tree(&lines[row], column) {
            trees+=1;
        }
        column += column_step;
    }
    trees
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = Vec::<String>::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    let mut product = 1;
    product *= slide(&lines, 1, 1);
    product *= slide(&lines, 3, 1);
    product *= slide(&lines, 5, 1);
    product *= slide(&lines, 7, 1);
    product *= slide(&lines, 1, 2);

    println!("{}", product);
    Ok(())
}
