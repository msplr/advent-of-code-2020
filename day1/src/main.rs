use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::option::Option;
use std::path::Path;

fn get_input(filename: &Path) -> Vec<u32> {
    let mut vals: Vec<u32> = Vec::new();
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        vals.push(line.unwrap().parse::<u32>().unwrap());
    }
    vals
}

fn find_pair(target: u32, input: &[u32]) -> Option<(u32, u32)> {
    let mut values: HashSet<u32> = HashSet::with_capacity(input.len());
    for val in input.iter() {
        if target >= *val {
            let counterpart = target - val;
            if values.contains(&counterpart) {
                return Some((*val, counterpart));
            }
        }
        values.insert(*val);
    }
    None
}

fn main() {
    let input_file = Path::new("./input.txt");
    let input = get_input(input_file);

    println!("part1");
    let res = find_pair(2020, &input[..]);
    if let Some((val, counterpart)) = res {
        let product = val * counterpart;
        println!("{} * {} = {}", val, counterpart, product);
    }

    println!("part2");
    for i in 0..input.len() - 1 {
        let val = input[i];
        if let Some((val2, val3)) = find_pair(2020 - val, &input[i + 1..]) {
            let product = val * val2 * val3;
            println!("{} * {} * {} = {}", val, val2, val3, product);
            break;
        }
    }
}
