use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn count_char(character: char, s: &str) -> u8 {
    let mut count: u8 = 0;
    for c in s.chars() {
        if c == character {
            count += 1;
        }
    }
    count
}

fn part1(input_file: &Path) {
    let file = File::open(input_file).unwrap();
    let lines = io::BufReader::new(file).lines();

    println!("part1");
    let mut valid_count: u32 = 0;
    for line in lines {
        let s = line.unwrap();
        let v: Vec<&str> = s.split(|c| c == ' ' || c == '-' || c == ':').collect();
        if let [min_, max_, c_, _, pw] = &v[..] {
            let c: char = c_.chars().next().unwrap();
            let min = min_.parse::<u8>().unwrap();
            let max = max_.parse::<u8>().unwrap();
            let count = count_char(c, pw);
            if count >= min && count <= max {
                valid_count += 1;
                // println!("{}-{} {}: {} => {}", min, max, c, pw, count);
            }
        }
    }
    println!("valid passwords: {}", valid_count);
}

fn part2(input_file: &Path) {
    let file = File::open(input_file).unwrap();
    let lines = io::BufReader::new(file).lines();

    println!("part2");
    let mut valid_count: u32 = 0;
    for line in lines {
        let s = line.unwrap();
        let v: Vec<&str> = s.split(|c| c == ' ' || c == '-' || c == ':').collect();
        if let [min_, max_, c_, _, pw] = &v[..] {
            let p1 = min_.parse::<usize>().unwrap();
            let p2 = max_.parse::<usize>().unwrap();
            let c: char = c_.chars().next().unwrap();
            let c1: char = pw.chars().nth(p1 - 1).unwrap();
            let c2: char = pw.chars().nth(p2 - 1).unwrap();
            if (c1 == c && c2 != c) || (c1 != c && c2 == c) {
                valid_count += 1;
                // println!("{} {} {}", c1, c2, c);
            }
        }
    }
    println!("valid passwords: {}", valid_count);
}

fn main() {
    let input_file = Path::new("./input.txt");
    part1(input_file);
    part2(input_file);
}
