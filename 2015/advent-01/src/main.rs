use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // read in file
    let file = File::open("apartment.txt").expect("Failed to open file");
    // let file = File::open("test.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut floor = 0;
    let mut basement_floor = 0;
    let mut basement_found = false;

    for line in reader.lines() {
        match line {
            Ok(real_line) if !real_line.is_empty() => {

                for ch in real_line.split("") {
                    if ch == "(" {
                        floor += 1;
                    } else if ch == ")" {
                        floor -= 1;
                    }

                    if !basement_found && floor == -1 {
                        basement_found = true;
                    }
                    if !basement_found {
                        basement_floor += 1;
                    }
                }
            }
            _ => {}
        }
    }

    println!("Part one : {}", floor);
    println!("Part two : {}", basement_floor);
}