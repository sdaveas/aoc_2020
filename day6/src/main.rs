use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

fn get_input(filename: &str) -> Result<Vec<String>, Box<dyn Error>>{
    let reader = BufReader::new(File::open(filename)?);
    let mut input= Vec::<String>::new();
    for line in reader.lines() {
        input.push(line?);
    }
    Ok(input)
}

fn main() -> Result<(), Box<dyn Error>> {

    let answers = get_input("input.txt")?;
    // part 1
    let mut total_unique_answers = 0;
    let mut unique_answers = HashSet::new();
    // part 2
    let mut total_common_answers = 0;
    let mut common_answers = HashMap::new();
    let mut grp_size = 0;

    for answer in answers {
        if answer == "" {
            // part 1
            total_unique_answers += unique_answers.len();
            unique_answers.clear();

            // part 2
            for (_, v) in common_answers.clone() {
                if v == grp_size {
                    total_common_answers += 1;
                }
            }
            common_answers.clear();
            grp_size = 0;
            continue;
        }
        grp_size += 1;
        for ch in answer.chars() {
            // part 1
            unique_answers.insert(ch);

            // part 2
            if common_answers.get(&ch) != None {
                *common_answers.get_mut(&ch).unwrap() += 1;
            }
            else {
                common_answers.insert(ch, 1);
            }
        }
    }

    // part 1
    println!("unique answers:{}", total_unique_answers);

    // part 2
    println!("common answers:{}", total_common_answers);

    Ok(())
}
