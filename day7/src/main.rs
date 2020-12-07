use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn get_input(filename: &str) -> Result<Vec<String>, Box<dyn Error>>{
let reader = BufReader::new(File::open(filename)?);
    let mut input= Vec::<String>::new();
    for line in reader.lines() {
        input.push(line?);
    }
    Ok(input)
}

fn find_bag(parent: &String, rules_hash: &HashMap<String, Vec<(String,i32)>>, mut visited: &mut HashSet<String>) -> bool {
    if visited.contains(parent) {
        return false;
    }
    for (bag, _) in rules_hash.get(parent).unwrap() {
        if visited.contains(bag) || find_bag(&bag, &rules_hash, &mut visited) {
            visited.insert(parent.clone());
            return true;
        }
    }
    false
}

fn count_bags(parent: &String, rules_hash: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let bags = rules_hash.get(parent).unwrap();
    let mut sum = 0;
    for (bag, count) in bags {
        sum += count * count_bags(bag, &rules_hash) + count;
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {

    let rules = get_input("input.txt")?;

    let reg_key = Regex::new(r"^\w+ \w+").unwrap();
    let reg_vals = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    let mut rules_hash = HashMap::new();

    for rule in rules {
        let mut k: String = String::from("");
        let mut vs: Vec<(String, i32)> = Vec::<(String, i32)>::new();
        for key in reg_key.captures_iter(&rule) {
            k = String::from(&key[0]);
        }
        for vals in reg_vals.captures_iter(&rule) {
            vs.push((String::from(&vals[2]), String::from(&vals[1]).parse::<i32>()?));
        }
        rules_hash.insert(k, vs);
    }

    // part 1
    let mut visited = HashSet::<String>::new();
    visited.insert(String::from("shiny gold"));     // add target
    for key in rules_hash.keys() {
        find_bag(&String::from(key), &rules_hash, &mut visited);
    }
    println!("{}", visited.len()-1);                // exclude target

    // part 2
    let count = count_bags(&String::from("shiny gold"), &rules_hash);
    println!("{}", count);

    Ok(())
}