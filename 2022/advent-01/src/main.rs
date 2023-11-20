use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read in file
    let file = File::open("elf_calories.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // group by empty lines
    let mut items: HashMap<u32, u32> = HashMap::new();
    let mut index = 0;

    for line in reader.lines() {
        if !items.contains_key(&index) {
            items.insert(index, 0);
        }

        match line {
            Ok(real_line) => {
                if real_line.is_empty() {
                    index += 1;
                    continue;
                }
                // add items in groups
                *items.get_mut(&index).unwrap() += real_line.parse::<u32>().unwrap();
            }
            _ => {}
        }
    }

    // emit highest count
    let max_calories = items.values().max().expect("No max value");
    println!("Max: {:?}", max_calories);

    // emit top 3x values summed
    let mut values: Vec<u32> = items.values().into_iter().map(|x| x.clone()).collect();
    values.sort();
    // println!("Vales: {:?}", values);
    let sum: u32 = values.into_iter().rev().take(3).sum();
    println!("Sum: {:?}", sum);
}
