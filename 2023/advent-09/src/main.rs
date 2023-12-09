use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // println!("Start : {line}");
                let parts = line.split(" ").map(|str| str.parse::<i32>().unwrap()).collect::<Vec<i32>>();

                let mut index = 0;
                let mut results = vec![parts];

                while results[index].iter().any(|item| item != &0) {
                    let row = &results[index];
                    // println!("ROW {}: {:?}", index, row);
                    let mut nextParts = vec![];

                    for i in 0..(row.len() - 1) {
                        let item = row[i];
                        let next = row[i + 1];
                        nextParts.push(next - item);
                    }

                    index += 1;
                    results.push(nextParts);
                }

                results.reverse();

                let mut add = 0;
                for (i, row) in results.iter_mut().enumerate() {
                    if i == 0 {
                        add = 0;
                        continue;
                    }

                    add += row[row.len() - 1];
                    // println!("Add : {add}");
                    row.push(add);
                }

                // println!("Parts : {:?}", results);
                dayOne += add;

                let mut minus = 0;
                for i  in 0..results.len() {
                    if i == 0 {
                        continue;
                    }
                    let prev = &results[i - 1][0].clone();
                    let row = &mut results[i];

                    minus = row[0] - prev;
                    // println!("Minus : {minus}");
                    row.insert(0, minus);
                }

                // println!("Parts : {:?}", results);
                dayTwo += minus;
            }
            _ => {}
        }
    }

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}
