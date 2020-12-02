use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

struct Record {
    f1: usize,
    f2: usize,
    ch: String,
    pass: String,
}

fn populate_record(line:&str, reg:&Regex) -> Result<Record, Box<dyn Error>> {
    let caps = reg.captures(line).expect("regex error");
    Ok(Record {
        f1:   caps.get(1).expect("f1").as_str().parse::<usize>()?,
        f2:   caps.get(2).expect("f2").as_str().parse::<usize>()?,
        ch:   String::from(caps.get(3).expect("ch").as_str()),
        pass: String::from(caps.get(4).expect("pass").as_str()),
    })
}

fn check_password_1(rec:&Record) -> bool {
    let matches = rec.pass.matches(&rec.ch).count();
    return rec.f1 <= matches && matches <= rec.f2;
}

fn check_password_2(rec:&Record) -> bool {
    let mi = rec.pass.get(rec.f1-1..rec.f1).unwrap() == rec.ch;
    let ma = rec.pass.get(rec.f2-1..rec.f2).unwrap() == rec.ch ;
    return mi && !ma || !mi &&  ma;
}

fn main() -> Result<(), Box<dyn Error>> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut valids_1 = 0;
    let mut valids_2 = 0;

    let reg = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")?;
    for line in reader.lines() {
        let l = &line?;
        let rec = populate_record(&l, &reg)?;
        if check_password_1(&rec) {
            valids_1 += 1;
        }
        if check_password_2(&rec) {
            valids_2 += 1;
        }
    }

    println!("{}", valids_1);
    println!("{}", valids_2);
    Ok(())
}
