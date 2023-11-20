use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    // read in file
    let file = File::open("test.txt").expect("Failed to open file");
    // let file = File::open("moves.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut buffer: Vec<Vec<String>> = Vec::new();
    let mut mode = Mode::ReadCrates;
    let mut crates: Vec<Vec<String>> = Vec::new();

    let re = Regex::new(r"move (?<crate>\d.*) from (?<from>\d.*) to (?<to>\d.*)").unwrap();

    for line in reader.lines() {
        // Read crates
        // Read crate IDs
        // Read blank line
        // Read moves
        match line {
            Ok(line) => {
                if mode == Mode::ReadCrates && !line.contains("[") {
                    mode = Mode::ReadIndex;
                } else if mode == Mode::ReadBlank {
                    mode = Mode::ReadMoves;
                    continue;
                }

                match mode {
                    Mode::ReadCrates => {
                        buffer.push(line.split("").into_iter().map(|x| x.to_string()).collect());
                    }
                    Mode::ReadIndex => {
                        let mut index = 0;
                        for char in line.split("") {
                            match char.parse::<u32>() {
                                Ok(_) => {
                                    let mut items: Vec<String> = Vec::new();

                                    for buf in &buffer {
                                        let lookup = buf.get(index).unwrap();
                                        if !lookup.trim().is_empty() {
                                            items.push(lookup.to_string());
                                        }
                                    }

                                    crates.push(items);
                                }
                                _ => {}
                            }

                            index += 1;
                        }
                        mode = Mode::ReadBlank;
                    }
                    Mode::ReadMoves => {
                        // move 24 from 2 to 3
                        let caps = re.captures(line.as_str()).unwrap();

                        println!(" Move {} from {} to {}", &caps["crate"], &caps["from"], &caps["to"]);

                        let src = crates.get_mut(0).unwrap();
                        let dst = crates.get_mut(1).unwrap();

                        // src.
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let result = crates
        .iter()
        .filter(|row| row.len() > 0)
        .map(|row| row.get(0).unwrap().to_owned())
        .collect::<Vec<String>>()
        .join("");

    println!("Crates : {:?}", crates);
    println!("Result : {}", result);
}

#[derive(Debug, PartialEq)]
enum Mode {
    ReadCrates,
    ReadIndex,
    ReadBlank,
    ReadMoves,
}
