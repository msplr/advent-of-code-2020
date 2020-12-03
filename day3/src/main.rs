use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(input_file: &Path) {
    let file = File::open(input_file).unwrap();
    let lines = io::BufReader::new(file).lines();

    println!("part1");
    let mut tree_count: usize = 0;
    let mut pos: usize = 0; // horizontal position
    for line in lines {
        let s = line.unwrap();
        let mut marker: char = 'O';
        if s.chars().nth(pos).unwrap() == '#' {
            tree_count += 1;
            marker = 'X';
        }
        println!("{}  {}{}{}", s, &s[..pos], marker, &s[pos + 1..]);

        // position wraps around after end of map
        pos = (pos + 3) % s.chars().count();
    }
    println!("Number of trees: {}", tree_count);
}

fn part2(input_file: &Path) {
    println!("part2");

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]; // right, down

    let mut solution = 1;
    for (slope_right, slope_down) in slopes {
        let file = File::open(input_file).unwrap();
        let lines = io::BufReader::new(file).lines();

        let mut tree_count: usize = 0;
        let mut pos: usize = 0; // horizontal position
        for (vpos, line) in lines.enumerate() {
            // skip nb down slope lines
            if vpos % slope_down != 0 {
                continue;
            }

            let s = line.unwrap();
            if s.chars().nth(pos).unwrap() == '#' {
                tree_count += 1;
            }

            // position wraps around after end of map
            pos = (pos + slope_right) % s.chars().count();
        }
        solution *= tree_count;
        println!(
            "Number of trees: {}  for slope right {} down {}",
            tree_count, slope_right, slope_down
        );
    }
    println!("Solution: {}", solution);
}

fn main() {
    let input_file = Path::new("./input.txt");
    part1(input_file);
    part2(input_file);
}
