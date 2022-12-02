use std::fs;
use std::io::{self, BufRead};

pub fn day1() -> (i32, i32) {
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

    (
        *elves.iter().max().unwrap(),
        elves.iter().rev().take(3).sum(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        assert_eq!((70369, 203002), day1());
    }
}
