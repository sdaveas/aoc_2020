fn main() {
    let nums = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let window_size = 25;
    let mut window_start = 0;
    let mut target: u64 = 0;
    for n in window_size..nums.len() {
        let mut found = false;
        for i in window_start..window_start + window_size {
            for j in window_start..window_start + window_size {
                if i < j && nums[i] + nums[j] == nums[n] {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            target = nums[n];
        }
        window_start += 1;
    }
    println!("{}", target);

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i < j && nums[i..j].iter().sum::<u64>() == target {
                println!(
                    "{:?}",
                    nums[i..j].iter().min().unwrap() + nums[i..j].iter().max().unwrap()
                );
                return;
            }
        }
    }
}
