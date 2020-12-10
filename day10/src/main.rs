use std::collections::HashMap;

fn main() {
    let mut nums = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    nums.sort();
    let mut diffs = [1; 4];
    let last = nums.last().cloned().unwrap();

    for pairs in nums.windows(2) {
        diffs[(pairs[1] - pairs[0]) as usize] += 1;
    }
    nums.push(last + 3);

    println!("{}", diffs[1] * diffs[3]);

    let mut h = HashMap::new();
    h.insert(0, 1 as i64);
    for num in nums {
        h.insert(
            num,
            h.get(&(&num - 1)).unwrap_or(&0)
                + h.get(&(&num - 2)).unwrap_or(&0)
                + h.get(&(&num - 3)).unwrap_or(&0),
        );
    }
    println!("{}", h.get(&last).unwrap());
}
