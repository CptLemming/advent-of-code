use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // read in file
    let file = File::open("test.txt").expect("Failed to open file");
    // let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
            }
            _ => {}
        }
    }
}
