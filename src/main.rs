use std::io::{self, BufRead};
use std::{collections::HashSet, fs};

pub fn day1_part_1() -> Option<(i32, i32, i32)> {
    let file = fs::File::open("src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    for (i, n) in nums.iter().enumerate() {
        for m in nums.iter().skip(i + 1) {
            if n + m == 2020 {
                println!("day1/1: {} {} {}", n, m, n * m);
                return Some((*n, *m, n * m));
            }
        }
    }

    None
}

pub fn day1_part_2() {
    let file = fs::File::open("src/day1.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate().skip(i + 1) {
            for o in nums.iter().skip(j + 1) {
                if n + m + o == 2020 {
                    println!("day1/2: {} {} {} {}", n, m, o, n * m * o);
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PasswordPolicy {
    min_chars: usize,
    max_chars: usize,
    required_char: char,
}

impl PasswordPolicy {
    fn is_valid(self, password: &str) -> bool {
        let required_chars = password.chars().fold(0, |found, c| {
            if c == self.required_char {
                return found + 1;
            }
            found
        });
        required_chars >= self.min_chars && required_chars <= self.max_chars
    }

    fn is_really_valid(self, password: &str) -> bool {
        let c1 = password.chars().skip(self.min_chars - 1).next().unwrap();
        let c2 = password.chars().skip(self.max_chars - 1).next().unwrap();
        (c1 == self.required_char) ^ (c2 == self.required_char)
    }
}

pub fn day2_input() -> Vec<(String, PasswordPolicy)> {
    let file = fs::File::open("src/day2.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let passwords: Vec<(String, PasswordPolicy)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(' ').collect();
            let minmax: Vec<&str> = parts[0].split('-').collect();
            let min_chars: usize = minmax[0].parse().unwrap();
            let max_chars: usize = minmax[1].parse().unwrap();
            let required_char = parts[1].chars().next().unwrap();
            let password = parts[2];
            (
                String::from(password),
                PasswordPolicy {
                    min_chars,
                    max_chars,
                    required_char,
                },
            )
        })
        .collect();

    passwords
}
pub fn day2_part_1() {
    let passwords = day2_input();

    let mut valid_password = 0;
    for (password, policy) in passwords {
        if policy.is_valid(&password) {
            valid_password += 1
        }
    }

    println!("day2/1: {}", valid_password);
}

pub fn day2_part_2() {
    let passwords = day2_input();

    let mut really_valid_password = 0;
    for (password, policy) in passwords {
        if policy.is_really_valid(&password) {
            really_valid_password += 1
        }
    }

    println!("day2/2: {}", really_valid_password);
}

type Day3Map = Vec<Vec<char>>;

pub fn day3_input() -> Day3Map {
    let file = fs::File::open("src/day3.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let map: Day3Map = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    map
}

pub fn tree_finder(map: &Day3Map, right: usize, down: usize) -> i64 {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;
    let rows = map.len();
    let cols = map[0].len();

    loop {
        x = (x + right) % cols;
        y += down;

        if y > rows - 1 {
            break;
        }

        if map[y][x] == '#' {
            trees += 1;
        }
    }

    trees
}

pub fn day3_part_1() {
    let map = day3_input();
    let trees = tree_finder(&map, 3, 1);
    println!("day3/1: {}", trees);
}

pub fn day3_part_2() {
    let map = day3_input();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let large_tree_number: i64 = slopes
        .iter()
        .map(|slope| tree_finder(&map, slope.0, slope.1))
        .product();

    println!("day3/2: {}", large_tree_number);
}

#[derive(Debug, Default)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn from_str(text: &str) -> Passport {
        let mut passport = Passport::default();

        let key_values: Vec<&str> = text.split(' ').collect();
        for kv in key_values {
            let parts: Vec<&str> = kv.split(':').collect();
            match parts[0] {
                "byr" => passport.byr = parts[1].into(),
                "iyr" => passport.iyr = parts[1].into(),
                "eyr" => passport.eyr = parts[1].into(),
                "hgt" => passport.hgt = parts[1].into(),
                "hcl" => passport.hcl = parts[1].into(),
                "ecl" => passport.ecl = parts[1].into(),
                "pid" => passport.pid = parts[1].into(),
                "cid" => passport.cid = parts[1].into(),
                _ => (),
            }
        }

        passport
    }

    fn is_valid(&self) -> bool {
        // need to have everything but self.cid
        let valid = self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid();
        valid
    }

    fn valid_byr(&self) -> bool {
        !self.byr.is_empty()
    }

    fn valid_iyr(&self) -> bool {
        !self.iyr.is_empty()
    }

    fn valid_eyr(&self) -> bool {
        !self.eyr.is_empty()
    }

    fn valid_hgt(&self) -> bool {
        !self.hgt.is_empty()
    }

    fn valid_hcl(&self) -> bool {
        !self.hcl.is_empty()
    }

    fn valid_ecl(&self) -> bool {
        !self.ecl.is_empty()
    }

    fn valid_pid(&self) -> bool {
        !self.pid.is_empty()
    }

    fn is_really_valid(&self) -> bool {
        // need to have everything but self.cid
        let valid = self.really_valid_byr()
            && self.really_valid_iyr()
            && self.really_valid_eyr()
            && self.really_valid_hgt()
            && self.really_valid_hcl()
            && self.really_valid_ecl()
            && self.really_valid_pid();
        valid
    }

    fn really_valid_byr(&self) -> bool {
        let byr: i32 = self.byr.parse().unwrap_or_default();
        let mut valid = self.byr.chars().filter(|c| c.is_ascii_digit()).count() == 4;
        valid &= byr >= 1920 && byr <= 2002;
        valid
    }

    fn really_valid_iyr(&self) -> bool {
        let iyr: i32 = self.iyr.parse().unwrap_or_default();
        let mut valid = self.iyr.chars().filter(|c| c.is_ascii_digit()).count() == 4;
        valid &= iyr >= 2010 && iyr <= 2020;
        valid
    }

    fn really_valid_eyr(&self) -> bool {
        let eyr: i32 = self.eyr.parse().unwrap_or_default();
        let mut valid = self.eyr.chars().filter(|c| c.is_ascii_digit()).count() == 4;
        valid &= eyr >= 2020 && eyr <= 2030;
        valid
    }

    fn really_valid_hgt(&self) -> bool {
        let mut valid = !self.hgt.is_empty();
        if !valid {
            return false;
        }

        let is_inch = self.hgt.ends_with("in");
        if is_inch {
            let h = self.hgt[..2].parse::<i32>().unwrap_or_default();
            valid &= h >= 59 && h <= 76;
            return valid;
        }
        let is_cm = self.hgt.ends_with("cm");
        if is_cm {
            let h = self.hgt[..3].parse::<i32>().unwrap_or_default();
            valid &= h >= 150 && h <= 193;
            return valid;
        }

        false
    }

    fn really_valid_hcl(&self) -> bool {
        let mut valid = !self.hcl.is_empty();
        valid &= self.hcl.starts_with("#");
        if !valid {
            return false;
        }
        let color = &self.hcl[1..];
        valid &= color.chars().filter(|c| c.is_ascii_hexdigit()).count() == 6;
        valid
    }

    fn really_valid_ecl(&self) -> bool {
        let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valid_eye_colors.contains(&self.ecl.as_str())
    }

    fn really_valid_pid(&self) -> bool {
        self.pid.chars().filter(|c| c.is_ascii_digit()).count() == 9
    }
}

pub fn day4_input() -> Vec<Passport> {
    let file = fs::File::open("src/day4.input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut passports = Vec::new();

    let mut current_passport = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            passports.push(Passport::from_str(&current_passport));
            current_passport.clear();
            continue;
        }

        current_passport.push_str(" ");
        current_passport.push_str(&line);
    }

    passports
}

pub fn day4_part_1() {
    let passports = day4_input();

    let valid = passports
        .into_iter()
        .filter(|passport| passport.is_valid())
        .count();
    println!("day4/1: {}", valid);
}

pub fn day4_part_2() {
    let passports = day4_input();

    let really_valid = passports
        .into_iter()
        .filter(|passport| passport.is_really_valid())
        .count();
    println!("day4/2: {}", really_valid);
}

pub fn decode_boarding_pass(boarding_pass: &str) -> (u8, u8) {
    let mut row = (0u8, 127u8);

    for c in boarding_pass.chars().take(7) {
        let gap = ((row.1 - row.0) / 2) + 1;
        if c == 'B' {
            row.0 += gap;
        }

        if c == 'F' {
            row.1 -= gap;
        }
    }

    let mut column = (0u8, 7u8);

    for c in boarding_pass.chars().skip(7).take(3) {
        let gap = ((column.1 - column.0) / 2) + 1;
        if c == 'R' {
            column.0 += gap;
        }

        if c == 'L' {
            column.1 -= gap;
        }
    }

    (row.0, column.0)
}

pub fn seat_id(seat: (u8, u8)) -> u32 {
    (seat.0 as u32 * 8) + seat.1 as u32
}

fn day5_input() -> Vec<String> {
    let file = fs::File::open("src/day5.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let boarding_passes: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    boarding_passes
}
pub fn day5_part_1() {
    let max_id = day5_input()
        .iter()
        .map(|boarding_pass| seat_id(decode_boarding_pass(boarding_pass)))
        .max()
        .unwrap();
    println!("day5/1: {}", max_id);
}

pub fn day5_part_2() {
    let seats: HashSet<u32> = day5_input()
        .iter()
        .map(|boarding_pass| seat_id(decode_boarding_pass(boarding_pass)))
        .collect();

    let min = *seats.iter().min().unwrap();
    let max = *seats.iter().max().unwrap();

    for seat in min..=max {
        if !seats.contains(&seat) {
            println!("day5/2: {}", seat);
            break;
        }
    }
}

fn day6_input() -> Vec<String> {
    let file = fs::File::open("src/day6.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut answers = Vec::new();

    let mut current_answer = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            answers.push(current_answer.clone());
            current_answer.clear();
            continue;
        }

        current_answer.push_str(&line);
    }

    answers
}

pub fn day6_part_1() {
    let answers = day6_input();
    let sum_answers: usize = answers
        .iter()
        .map(|answer| answer.chars().collect::<HashSet<char>>().len()).sum();
    println!("day6/1: {}", sum_answers);
}

pub fn day6_part_2() {
    // let data = day6_input();
}

fn main() {
    // day1_part_1();
    // day1_part_2();
    // day2_part_1();
    // day2_part_2();
    // day3_part_1();
    // day3_part_2();
    // day4_part_1();
    // day4_part_2();
    // day5_part_1();
    // day5_part_2();
    day6_part_1();
    day6_part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2020() {
        assert_eq!((455i32, 1565i32, 712075i32), day1_part_1().unwrap());
        // assert_eq!((1142,695,183,145245270), day1_part_2());
        // assert_eq!(640, day2_part_1());
        // assert_eq!(472, day2_part_2());
        // assert_eq!(232, day3_part_1());
        // assert_eq!(3952291680, day3_part_2());
        // assert_eq!(230, day4_part_1());
        // assert_eq!(156, day4_part_2());
        // assert_eq!(928, day5_part_1());
        // assert_eq!(610, day5_part_2());
    }
}
