use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    // let file = File::open("test2.txt").expect("Failed to open file");
    // let file = File::open("dayTwoTest.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut instructions: Vec<String> = vec![];
    let mut lines: HashMap<String, Line> = HashMap::default();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if instructions.is_empty() {
                    instructions = line.chars().map(|str| str.to_string()).collect::<Vec<String>>();
                    continue;
                } else if line.is_empty() {
                    continue;
                }

                let split = line.split("=").collect::<Vec<&str>>();
                let id = split[0].trim().to_string();
                let routes = split[1].replace("(", "").replace(")", "").split(",").map(|str| str.trim().to_string()).filter(|str| !str.is_empty()).collect::<Vec<String>>();

                lines.insert(id.clone(), Line { id, left: routes[0].clone(), right: routes[1].clone() });
            }
            _ => {}
        }
    }

    let mut index = 0;
    let mut current = "AAA".to_string();

    while !current.ends_with("Z") {
        let instruction = instructions[index].clone();
        current = match instruction.as_str() {
            "L" => lines.get(&current).unwrap().left.clone(),
            "R" => lines.get(&current).unwrap().right.clone(),
            _ => panic!("Invalid instruction : {instruction}"),
        };

        dayOne += 1;
        index += 1;
        if index >= instructions.len() {
            index = 0;
        }
    }

    let nodes = lines.keys().into_iter().filter_map(|line| match line.ends_with("A") {
        true => Some(line.clone()),
        false => None,
    }).collect::<Vec<String>>();

    println!("DayTwo Nodes : {nodes:?}");

    let mut results = nodes.iter().map(|node| {
        let mut index = 0;
        let mut current = node.clone();
        let mut result = 0;
        // println!("Starting : {current}");

        while !current.ends_with("Z") {
            let instruction = instructions[index].clone();
            current = match instruction.as_str() {
                "L" => lines.get(&current).unwrap().left.clone(),
                "R" => lines.get(&current).unwrap().right.clone(),
                _ => panic!("Invalid instruction : {instruction}"),
            };
            // println!("Current : {current}");

            result += 1;
            index += 1;
            if index >= instructions.len() {
                index = 0;
            }
        }

        result
    }).collect::<Vec<usize>>();

    println!("Walk : {results:?}");
    // Use LCM, not really sure what this means but is mucho fast
    let dayTwo = results
        .iter()
        .fold(1, |acc, steps| num::integer::lcm(acc, *steps));

    // println!("Instructions : {instructions:?}");
    // println!("Lines : {lines:?}");
    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

#[derive(Debug)]
struct Line {
    id: String,
    left: String,
    right: String,
}
