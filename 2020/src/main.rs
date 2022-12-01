use std::io::{self, BufRead};
use std::{collections::HashSet, fs};

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
        let c1 = password.chars().nth(self.min_chars - 1).unwrap();
        let c2 = password.chars().nth(self.max_chars - 1).unwrap();
        (c1 == self.required_char) ^ (c2 == self.required_char)
    }
}

pub fn day2_input() -> Vec<(String, PasswordPolicy)> {
    let file = fs::File::open("2020/src/day2.input.txt").unwrap();
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
pub fn day2_part_1() -> i32 {
    let passwords = day2_input();

    let mut valid_password = 0;
    for (password, policy) in passwords {
        if policy.is_valid(&password) {
            valid_password += 1
        }
    }

    println!("day2/1: {}", valid_password);
    valid_password
}

pub fn day2_part_2() -> i32 {
    let passwords = day2_input();

    let mut really_valid_password = 0;
    for (password, policy) in passwords {
        if policy.is_really_valid(&password) {
            really_valid_password += 1
        }
    }

    println!("day2/2: {}", really_valid_password);
    really_valid_password
}

type Day3Map = Vec<Vec<char>>;

pub fn day3_input() -> Day3Map {
    let file = fs::File::open("2020/src/day3.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let map: Day3Map = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    map
}

pub fn tree_finder(map: &[Vec<char>], right: usize, down: usize) -> i64 {
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

pub fn day3_part_1() -> i64 {
    let map = day3_input();
    let trees = tree_finder(&map, 3, 1);
    println!("day3/1: {}", trees);
    trees
}

pub fn day3_part_2() -> i64 {
    let map = day3_input();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let large_tree_number: i64 = slopes
        .iter()
        .map(|slope| tree_finder(&map, slope.0, slope.1))
        .product();

    println!("day3/2: {}", large_tree_number);
    large_tree_number
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
        self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid()
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
        self.really_valid_byr()
            && self.really_valid_iyr()
            && self.really_valid_eyr()
            && self.really_valid_hgt()
            && self.really_valid_hcl()
            && self.really_valid_ecl()
            && self.really_valid_pid()
    }

    fn really_valid_byr(&self) -> bool {
        let byr: i32 = self.byr.parse().unwrap_or_default();
        let mut valid = self.byr.chars().filter(|c| c.is_ascii_digit()).count() == 4;
        valid &= (1920..=2002).contains(&byr);
        valid
    }

    fn really_valid_iyr(&self) -> bool {
        let iyr: i32 = self.iyr.parse().unwrap_or_default();
        let mut valid = self.iyr.chars().filter(|c| c.is_ascii_digit()).count() == 4;
        valid &= (2010..=2020).contains(&iyr);
        valid
    }

    fn really_valid_eyr(&self) -> bool {
        let eyr: i32 = self.eyr.parse().unwrap_or_default();
        let mut valid = self.eyr.chars().filter(|c| c.is_ascii_digit()).count() == 4;
        valid &= (2020..=2030).contains(&eyr);
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
            valid &= (59..=76).contains(&h);
            return valid;
        }
        let is_cm = self.hgt.ends_with("cm");
        if is_cm {
            let h = self.hgt[..3].parse::<i32>().unwrap_or_default();
            valid &= (150..=193).contains(&h);
            return valid;
        }

        false
    }

    fn really_valid_hcl(&self) -> bool {
        let mut valid = !self.hcl.is_empty();
        valid &= self.hcl.starts_with('#');
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
    let file = fs::File::open("2020/src/day4.input.txt").unwrap();
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

        current_passport.push(' ');
        current_passport.push_str(&line);
    }

    passports
}

pub fn day4_part_1() -> usize {
    let passports = day4_input();

    let valid = passports
        .into_iter()
        .filter(|passport| passport.is_valid())
        .count();
    println!("day4/1: {}", valid);
    valid
}

pub fn day4_part_2() -> usize {
    let passports = day4_input();

    let really_valid = passports
        .into_iter()
        .filter(|passport| passport.is_really_valid())
        .count();
    println!("day4/2: {}", really_valid);
    really_valid
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
    let file = fs::File::open("2020/src/day5.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let boarding_passes: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    boarding_passes
}
pub fn day5_part_1() -> u32 {
    let max_id = day5_input()
        .iter()
        .map(|boarding_pass| seat_id(decode_boarding_pass(boarding_pass)))
        .max()
        .unwrap();
    println!("day5/1: {}", max_id);
    max_id
}

pub fn day5_part_2() -> u32 {
    let seats: HashSet<u32> = day5_input()
        .iter()
        .map(|boarding_pass| seat_id(decode_boarding_pass(boarding_pass)))
        .collect();

    let min = *seats.iter().min().unwrap();
    let max = *seats.iter().max().unwrap();

    let mut my_seat = 0;

    for seat in min..=max {
        if !seats.contains(&seat) {
            my_seat = seat;
            println!("day5/2: {}", my_seat);
            break;
        }
    }

    my_seat
}

fn day6_input() -> Vec<Vec<String>> {
    let file = fs::File::open("2020/src/day6.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut answers = Vec::new();

    let mut current_answers = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            answers.push(current_answers.clone());
            current_answers.clear();
            continue;
        }

        current_answers.push(line.clone());
    }

    answers
}

pub fn day6_part_1() -> usize {
    let answers = day6_input();
    let sum_answers: usize = answers
        .iter()
        .map(|answers| answers.concat().chars().collect::<HashSet<char>>().len())
        .sum();
    println!("day6/1: {}", sum_answers);
    sum_answers
}

pub fn day6_part_2() -> u32 {
    let answers = day6_input();

    fn str_as_bits(s: &str) -> u32 {
        let mut n: u32 = 0;
        for c in s.chars() {
            let offset = c as u8 - b'a';
            n |= 1 << offset;
        }
        n
    }

    let mut sum_ansers = 0;

    for group in answers {
        let bit_answers: Vec<u32> = group.iter().map(|answer| str_as_bits(answer)).collect();
        sum_ansers += bit_answers
            .into_iter()
            .reduce(|a, b| a & b)
            .unwrap()
            .count_ones();
    }

    println!("day6/2: {}", sum_ansers);
    sum_ansers
}

fn day7_input() -> Vec<(String, String, i32)> {
    let file = fs::File::open("2020/src/day7.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split("contain").collect();
        let this_bag_color = parts[0].split(' ').take(2).collect::<Vec<&str>>().join(" ");

        let bag_links: Vec<&str> = parts[1].split(',').collect();
        for bag_link in bag_links {
            let bag_link_parts = bag_link.trim().split(' ');

            let other_bag_color = bag_link_parts
                .skip(1)
                .take(2)
                .collect::<Vec<&str>>()
                .join(" ");
            let bag_link_parts = bag_link.trim().split(' ').collect::<Vec<&str>>();

            let other_bag_capacity: i32 = bag_link_parts[0].parse().unwrap_or(0);
            edges.push((
                this_bag_color.clone(),
                other_bag_color.clone(),
                other_bag_capacity,
            ));
        }
    }

    edges
}

pub fn day7_part_1() -> usize {
    let edges = day7_input();

    let mut colors = HashSet::new();
    let mut colors_to_visit = Vec::new();

    colors_to_visit.push("shiny gold".to_string());

    while !colors_to_visit.is_empty() {
        let current_visit = colors_to_visit.pop().unwrap();

        for (e1, e2, _) in edges.iter() {
            if *e2 == current_visit {
                colors.insert(e1.clone());
                colors_to_visit.push(e1.clone());
            }
        }
    }

    println!("day7/1: {}", colors.len());
    colors.len()
}

pub fn day7_part_2() -> i32 {
    let edges = day7_input();

    let mut colors_to_visit = Vec::new();
    let mut total_bags = 0;

    colors_to_visit.push("shiny gold".to_string());

    while !colors_to_visit.is_empty() {
        let current_visit = colors_to_visit.pop().unwrap();

        for (e1, e2, c) in edges.iter() {
            if *e1 == current_visit {
                for _ in 0..*c {
                    colors_to_visit.push(e2.clone());
                    total_bags += 1;
                }
            }
        }
    }

    println!("day7/2: {}", total_bags);
    total_bags
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Eq, PartialEq)]
pub enum HaltState {
    EndOfProgram,
    InfiniteLoop,
}

pub struct Computer {
    a: i32,
    pc: u32,
    instructions: Vec<Instruction>,
    trace: HashSet<u32>,
}

impl Computer {
    pub fn run(&mut self) -> (HaltState, i32) {
        let halt_state;
        let n_ins = self.instructions.len();

        loop {
            if self.pc as usize >= n_ins {
                halt_state = HaltState::EndOfProgram;
                break;
            }

            let ins = self.instructions[self.pc as usize];

            // halt on pc repetition
            if self.trace.contains(&self.pc) {
                halt_state = HaltState::InfiniteLoop;
                break;
            }

            self.trace.insert(self.pc);

            match ins {
                Instruction::Acc(n) => self.acc(n),
                Instruction::Jmp(n) => self.jmp(n),
                Instruction::Nop(_) => self.nop(),
            }
        }

        (halt_state, self.a)
    }

    pub fn acc(&mut self, n: i32) {
        self.a += n;
        self.pc += 1;
    }

    pub fn jmp(&mut self, n: i32) {
        self.pc = ((self.pc as i32) + n) as u32;
    }

    pub fn nop(&mut self) {
        self.pc += 1;
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.pc = 0;
        self.trace = HashSet::default();
    }
}

fn day8_input() -> Computer {
    let file = fs::File::open("2020/src/day8.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let n = parts[1].parse().unwrap();

        let ins = match parts[0] {
            "acc" => Instruction::Acc(n),
            "jmp" => Instruction::Jmp(n),
            "nop" => Instruction::Nop(n),
            _ => panic!(),
        };

        instructions.push(ins);
    }

    Computer {
        a: 0,
        pc: 0,
        instructions,
        trace: HashSet::default(),
    }
}

pub fn day8_part_1() -> i32 {
    let mut computer = day8_input();
    let (_, a) = computer.run();
    println!("day8/1: {}", a);
    a
}

pub fn day8_part_2() -> i32 {
    let mut computer = day8_input();
    let mut last_a = 0;

    let n_ins = computer.instructions.len();

    for i in 0..n_ins {
        let target_ins = computer.instructions[i as usize];
        let mutated_ins = match target_ins {
            Instruction::Jmp(n) => Some(Instruction::Nop(n)),
            Instruction::Nop(n) => Some(Instruction::Jmp(n)),
            _ => None,
        };

        if let Some(ins) = mutated_ins {
            computer.instructions[i] = ins;
        }

        let (halt_state, a) = computer.run();
        if halt_state == HaltState::EndOfProgram {
            last_a = a;
            break;
        }

        // restore mutated ins and reset our computer
        computer.instructions[i] = target_ins;
        computer.reset();
    }

    println!("day8/2: {}", last_a);
    last_a
}

fn day9_input() -> Vec<i64> {
    let file = fs::File::open("2020/src/day9.input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut xmas = Vec::new();

    for line in reader.lines() {
        let n = line.unwrap().parse().unwrap();
        xmas.push(n);
    }

    xmas
}

pub fn day9_part_1() -> i64 {
    let xmas = day9_input();
    const PREAMBLE: usize = 25;

    let mut window = HashSet::<i64>::default();
    for i in &xmas[..PREAMBLE] {
        window.insert(*i);
    }

    let mut invalid = 0;

    for i in PREAMBLE..xmas.len() {
        let n = xmas[i];
        let mut valid = false;
        for w in window.iter() {
            let diff = n - w;
            if window.contains(&diff) {
                valid = true;
                break;
            }
        }
        if !valid {
            invalid = n;
            break;
        }

        window.insert(n);
        window.remove(&xmas[i - PREAMBLE]);
    }

    println!("day9/1: {}", invalid);
    invalid
}

pub fn day9_part_2() -> i32 {
    let xmas = day9_input();
    let invalid_from_part_1 = 756008079;
    let mut weakness = 0;

    'outer: for i in 0..(xmas.len() - 1) {
        for j in (i + 1)..xmas.len() {
            let set = &xmas[i..=j];
            let sum: i64 = set.iter().sum();
            if sum == invalid_from_part_1 {
                let min = set.iter().min().unwrap();
                let max = set.iter().max().unwrap();
                weakness = min + max;
                break 'outer;
            }
        }
    }

    println!("day9/2: {}", weakness);
    0
}

fn main() {
    day9_part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(712075, day1_part_1());
        assert_eq!(145245270, day1_part_2());
    }

    #[test]
    fn day2() {
        assert_eq!(640, day2_part_1());
        assert_eq!(472, day2_part_2());
    }

    #[test]
    fn day3() {
        assert_eq!(232, day3_part_1());
        assert_eq!(3952291680, day3_part_2());
    }

    #[test]
    fn day4() {
        assert_eq!(230, day4_part_1());
        assert_eq!(156, day4_part_2());
    }

    #[test]
    fn day5() {
        assert_eq!(928, day5_part_1());
        assert_eq!(610, day5_part_2());
    }

    #[test]
    fn day6() {
        assert_eq!(6521, day6_part_1());
        assert_eq!(3305, day6_part_2());
    }

    #[test]
    fn day7() {
        assert_eq!(126, day7_part_1());
        assert_eq!(220149, day7_part_2());
    }

    #[test]
    fn day8() {
        assert_eq!(2080, day8_part_1());
        assert_eq!(2477, day8_part_2());
    }

    #[test]
    fn day9() {
        assert_eq!(756008079, day9_part_1());
        assert_eq!(93727241, day9_part_2());
    }
}
