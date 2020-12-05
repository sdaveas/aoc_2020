use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = Vec::<String>::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    let reg_byr = Regex::new(r"byr:(\d{4})").unwrap();
    let reg_iyr = Regex::new(r"iyr:(\d{4})").unwrap();
    let reg_eyr = Regex::new(r"eyr:(\d{4})").unwrap();
    let reg_hgt = Regex::new(r"hgt:(\d+)(\w+)").unwrap();
    let reg_hcl = Regex::new(r"hcl:#[0-9a-f]{6}").unwrap();
    let reg_ecl = Regex::new(r"ecl:[amb|blu|brn|gry|grn|hzl|oth]").unwrap();
    let reg_pid = Regex::new(r"pid:[0-9]{9}").unwrap();

    let mut valids = 0;

    for line in lines {

        let mut byr = false;
        let mut iyr = false;
        let mut eyr = false;
        let mut hgt = false;
        let mut hcl = false;
        let mut ecl = false;
        let mut pid = false;

        println!("{}", &line);
        for cap in reg_byr.captures_iter(&line) {
            let year:i32 = cap[1].parse()?;
            if 1920 <= year && year <= 2002 {
                byr = true;
            }
        }

        for cap in reg_iyr.captures_iter(&line) {
            let year:i32 = cap[1].parse()?;
            if 2010 <= year && year <= 2020 {
                iyr = true;
            }
        }

        for cap in reg_eyr.captures_iter(&line) {
            let year:i32 = cap[1].parse()?;
            if 2020 <= year && year <= 2030 {
                eyr = true;
            }
        }

        for cap in reg_hgt.captures_iter(&line) {
            let height:i32 = cap[1].parse()?;
            let unit = &cap[2];
            println!("unit: {}, height: {}", unit, height);
            if (unit == "cm" && 150 <= height && height <= 193) ||
               (unit == "in" &&  59 <= height && height <=  76)
            {
                hgt = true;
            }
        }

        if reg_hcl.is_match(&line) {
            hcl = true;
        }

        if reg_ecl.is_match(&line) {
            ecl = true;
        }

        if reg_pid.is_match(&line) {
            pid = true;
        }

        //if byr && iyr && eyr && hgt && hcl && ecl && pid {
        if ecl {
            valids += 1;
        }
    }

    println!("{}", valids);

    Ok(())
}
