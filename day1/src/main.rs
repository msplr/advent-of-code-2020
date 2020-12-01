use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn get_input(filename: &Path) -> Vec<u32> {
    let mut vals: Vec<u32> = Vec::new();
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        vals.push(line.unwrap().parse::<u32>().unwrap());
    }
    vals
}

fn main() {
    let input_file = Path::new("./input.txt");
    let input = get_input(input_file);

    let mut values : HashSet<u32> = HashSet::with_capacity(input.len());
    for val in input.iter() {
        values.insert(*val);
        let counterpart = 2020 - val;
        if values.contains(&counterpart) {
            let result = val * counterpart;
            println!("{} * {} = {}", val, counterpart, result);
            break;
        }
    }
}
