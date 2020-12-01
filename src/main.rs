use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn day1_part_1() {
    let file = fs::File::open("src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| i32::from_str(line.unwrap().as_str()).unwrap())
        .collect();

    for (i, n) in nums.iter().enumerate() {
        for m in nums.iter().skip(i + 1) {
            if n + m == 2020 {
                println!("{} {} {}", n, m, n * m);
                break;
            }
        }
    }
}

pub fn day1_part_2() {
    let file = fs::File::open("src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| i32::from_str(line.unwrap().as_str()).unwrap())
        .collect();

    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate().skip(i + 1) {
            for o in nums.iter().skip(j + 1) {
                if n + m + o == 2020 {
                    println!("{} {} {} {}", n, m, o, n * m * o);
                    break;
                }
            }
        }
    }
}

fn main() {
    day1_part_1();
    day1_part_2();
}
