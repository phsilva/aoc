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
            let min_chars = usize::from_str(minmax[0]).unwrap();
            let max_chars = usize::from_str(minmax[1]).unwrap();
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

    println!("valid password: {}", valid_password);
}

pub fn day2_part_2() {
    let passwords = day2_input();

    let mut really_valid_password = 0;
    for (password, policy) in passwords {
        if policy.is_really_valid(&password) {
            really_valid_password += 1
        }
    }

    println!("really valid password: {}", really_valid_password);
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
    println!("day3: part 1: trees found {}", trees);
}

pub fn day3_part_2() {
    let map = day3_input();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let large_tree_number: i64 = slopes
        .iter()
        .map(|slope| tree_finder(&map, slope.0, slope.1))
        .product();

    println!("day3: part 3: trees found product {}", large_tree_number);
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
                "byr" => passport.byr = String::from_str(parts[1]).unwrap(),
                "iyr" => passport.iyr = String::from_str(parts[1]).unwrap(),
                "eyr" => passport.eyr = String::from_str(parts[1]).unwrap(),
                "hgt" => passport.hgt = String::from_str(parts[1]).unwrap(),
                "hcl" => passport.hcl = String::from_str(parts[1]).unwrap(),
                "ecl" => passport.ecl = String::from_str(parts[1]).unwrap(),
                "pid" => passport.pid = String::from_str(parts[1]).unwrap(),
                "cid" => passport.cid = String::from_str(parts[1]).unwrap(),
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
        let color = String::from_str(&self.hcl[1..]).unwrap();
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

        current_passport += &(String::from_str(" ").unwrap() + &line.clone());
    }

    passports
}

pub fn day4_part_1() {
    let passports = day4_input();

    let valid = passports
        .into_iter()
        .filter(|passport| passport.is_valid())
        .count();
    println!("day4/1: valid passports: {}", valid);
}

pub fn day4_part_2() {
    let passports = day4_input();

    let really_valid = passports
        .into_iter()
        .filter(|passport| passport.is_really_valid())
        .count();
    println!("day4/1: really valid passports: {}", really_valid);
}

fn main() {
    // day1_part_1();
    // day1_part_2();
    // day2_part_1();
    // day2_part_2();
    // day3_part_1();
    // day3_part_2();
    day4_part_1();
    day4_part_2();
}
