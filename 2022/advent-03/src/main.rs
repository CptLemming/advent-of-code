use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("rucksack.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Sum of errors
    let mut sum = 0;
    let mut buffer: Vec<String> = Vec::new();

    for line in reader.lines() {
        // split each line in half
        // find the common char between the two
        // lookup the priority of the char:
        // a-z = 1-26
        // A-Z = 27 - 52
        match line {
            Ok(line) if !line.is_empty() => {
                // let len = line.len();
                // let (first, second) = line.split_at(len / 2);

                // for part in second.chars() {
                //     if first.contains(part) {
                //         let val = get_value_from_char(part);
                //         sum += val;

                //         // Prevent counting same char twice
                //         break;
                //     }
                // }
                // println!("push {}", line);
                buffer.push(line);

                if buffer.len() == 3 {
                    sum += process_buffer(&buffer);
                    buffer.clear();
                }
            }
            _ => {}
        }
    }

    if !buffer.is_empty() {
        sum += process_buffer(&buffer);
    }

    // test = 157
    println!("Sum : {}", sum);
}

fn process_buffer(buffer: &Vec<String>) -> u32 {
    // println!("Buffer len -> {}", buffer.len());
    let first = buffer.get(0).unwrap();
    let second = buffer.get(1).unwrap();
    let third = buffer.get(2).unwrap();

    for part in first.chars() {
        if second.contains(part) && third.contains(part) {
            // println!("Match -> {}", part);
            return get_value_from_char(part);
        }
    }

    return 0;
}

fn get_value_from_char(char: char) -> u32 {
    let val = char as u32;

    if val > 96 {
        // Lower case
        return val - 96;
    }
    // Upper case
    return val - 38;
}
