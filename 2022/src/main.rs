use std::fs;
use std::io::{self, BufRead};

pub fn day1_part_1() -> i32 {
    let file = fs::File::open("src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let calories = reader.lines().map(|line| line.unwrap());

    let mut max_calories: i32 = i32::MIN;
    let mut current = 0;

    for calorie in calories {
        if calorie.is_empty() {
            if current > max_calories {
                max_calories = current;
            }
            current = 0;
            continue;
        }

        current += calorie.parse::<i32>().unwrap();
    }
    max_calories
}

pub fn day1_part_2() -> i32 {
    let file = fs::File::open("src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let calories = reader.lines().map(|line| line.unwrap());

    let mut current = 0;
    let mut elves = Vec::new();

    for calorie in calories {
        if calorie.is_empty() {
            elves.push(current);
            current = 0;
            continue;
        }

        current += calorie.parse::<i32>().unwrap();
    }

    elves.sort();
    elves.iter().rev().take(3).sum()
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(70369, day1_part_1());
        assert_eq!(203002, day1_part_2());
    }
}
