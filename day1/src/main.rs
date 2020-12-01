// 1, 2, 3, 4, 5 | 9
// hashmap{8|1, 7|2, 6|3, 5|4, 4|5}

use std::error::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn first(nums: &Vec<i32>) {
    let mut hashmap = HashMap::new();
    for num in nums {
        hashmap.insert(2020 - num, num);
        if hashmap.get(num) != None {
            println!(
                "{} * {} = {}",
                num,
                2020 - num,
                num * (2020 - num)
            );
            break;
        }
    }
}

// 1, 2, 3, 4, 5 | 9
// hashmap{8|1, 7|2, 6|3, 5|4, 4|5}
// 1 + 2 == 8? 1 + 3 == 8? 1 + 4 == 8? ...
fn second(nums: &Vec<i32>) {
    let mut hashmap = HashMap::new();
    for i in 0..nums.len() {
        hashmap.insert(2020 - nums[i], nums[i]);
        for j in 0..i {
            if hashmap.get(&(nums[i] + nums[j])) != None {
                println!(
                    "{} * {} * {} = {}",
                    nums[i],
                    nums[j],
                    2020 - nums[i] + nums[j],
                    nums[i] * nums[j] * (2020 - nums[i] - nums[j])
                );
                return;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut nums = Vec::new();
    for line in reader.lines() {
        nums.push(line?.parse()?);
    }
    first(&nums);
    second(&nums);
    Ok(())
}
