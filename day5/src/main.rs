use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn get_input(filename: &str) -> Result<Vec<String>, Box<dyn Error>>{
    let reader = BufReader::new(File::open(filename)?);
    let mut input= Vec::<String>::new();
    for line in reader.lines() {
        input.push(line?);
    }
    Ok(input)
}

fn parse_as_bin(bp_slice: &str, zero: char, one: char) -> isize {
    let s:&str = &bp_slice.replace(zero, "0").replace(one, "1");
    isize::from_str_radix(s, 2).unwrap()
}

fn get_seat_id(row: isize, seat: isize) -> isize {
    row*8+seat
}

fn main() -> Result<(), Box<dyn Error>> {

    let boardings = get_input("input.txt")?;

    let mut seats = HashMap::new();
    let mut min = 128 * 8 + 7;
    let mut max = 0;
    for boarding in boardings {
        let row = parse_as_bin(&boarding[0..7], 'F', 'B');
        let seat = parse_as_bin(&boarding[7..], 'L', 'R');
        let seat_id = get_seat_id(row, seat);
        if min > seat_id { min = seat_id; }
        if max < seat_id { max = seat_id; }
        seats.insert(seat_id, true);
    }

    // part 1
    println!("min seat:{}, max seat:{}", min, max);

    // part 2
    for seat_id in min..max {
        if seats.get(&seat_id) == None {
            println!("seat {} does not exist", seat_id);
        }
    }

    Ok(())
}
