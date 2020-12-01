use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let file = fs::File::open("src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| i32::from_str(line.unwrap().as_str()).unwrap())
        .collect();

    for i in 0..(nums.len() - 1) {
        let n = nums[i];
        for j in 1..nums.len() {
            let m = nums[j];
            if n + m == 2020 {
                println!("{} {} {}", n, m, n*m);
            }
        }
    }
}
