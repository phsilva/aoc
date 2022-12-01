use std::io::{self, BufRead};
use std::fs;

pub fn day1_part_1() -> i32 {
    let file = fs::File::open("2020/src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut answer: i32 = 0;

    for (i, n) in nums.iter().enumerate() {
        for m in nums.iter().skip(i + 1) {
            if n + m == 2020 {
                println!("day1/1: {}", n * m);
                answer = n * m;
            }
        }
    }

    answer
}

pub fn day1_part_2() -> i32 {
    let file = fs::File::open("2020/src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut answer: i32 = 0;

    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate().skip(i + 1) {
            for o in nums.iter().skip(j + 1) {
                if n + m + o == 2020 {
                    answer = n * m * o;
                    println!("day1/2: {}", answer);
                    break;
                }
            }
        }
    }

    answer
}

fn main() {
    day1_part_1();
    day1_part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(712075, day1_part_1());
        assert_eq!(145245270, day1_part_2());
    }

}
