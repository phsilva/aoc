use std::fs;

pub fn day1() -> (i32, i32) {
    let input = fs::read_to_string("src/day1.input.txt").unwrap();
    let mut elves: Vec<_> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

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

fn main() {}