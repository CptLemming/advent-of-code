use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, HashSet}};
use colored::{Colorize, ColoredString};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut parts: Vec<PartNo> = vec![];
    let mut symbols: Vec<Position> = vec![];

    let mut maxRows = 0;
    let mut maxCols = 0;

    for (row, line) in reader.lines().enumerate() {
        if row > maxRows {
            maxRows = row;
        }
        match line {
            Ok(line) => {
                let mut parsedPositions: Vec<Position> = vec![];
                for (col, char) in line.chars().enumerate() {
                    if col > maxCols {
                        maxCols = col;
                    }
                    if char.to_string() == "." {
                        parts.push(PartNo { positions: parsedPositions.clone() });
                        parsedPositions.clear();
                        continue;
                    } else if char.to_string().parse::<u32>().is_ok() {
                        parsedPositions.push(Position { value: char.to_string(), x: col as u32, y: row as u32 });
                    } else {
                        symbols.push(Position { value: char.to_string(), x: col as u32, y: row as u32 });

                        if parsedPositions.len() > 0 {
                            parts.push(PartNo { positions: parsedPositions.clone() });
                            parsedPositions.clear();
                        }
                    }
                }

                if parsedPositions.len() > 0 {
                    parts.push(PartNo { positions: parsedPositions.clone() });
                }
            }
            _ => {}
        }
    }

    // println!("Parts {:?}", parts);
    // println!("Symbols {:?}", symbols);

    for part in parts.iter() {
        for symbol in symbols.iter() {
            if part.matches(symbol) {
                dayOne += part.get_part_reference();
                break;
            }
        }
    }

    for symbol in symbols.iter() {
        let mut matched: Vec<u32> = vec![];

        if symbol.value != "*" {
            continue;
        }

        for part in parts.iter() {
            if part.matches(symbol) {
                matched.push(part.get_part_reference());
            }
        }

        if matched.len() == 2 {
            dayTwo += matched.get(0).unwrap() * matched.get(1).unwrap();
        }
    }

    // draw_preview(&parts, &symbols, maxRows, maxCols);

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

#[derive(Clone, Debug)]
struct Position {
    value: String,
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct PartNo {
    positions: Vec<Position>,
}

impl PartNo {
    pub fn matches(&self, pos: &Position) -> bool {
        return self.positions.iter().any(|part_pos| {
            return part_pos.x >= pos.x - 1 && part_pos.x <= pos.x + 1 && part_pos.y >= pos.y - 1 && part_pos.y <= pos.y + 1
        });
    }

    pub fn get_part_reference(&self) -> u32 {
        return self.positions.iter().map(|pos| pos.value.clone()).collect::<Vec<String>>().join("").parse::<u32>().unwrap();
    }
}

fn draw_preview(parts: &Vec<PartNo>, symbols: &Vec<Position>, maxRows: usize, maxCols: usize) {
    let mut preview: HashMap<u32, HashMap<u32, ColoredString>> = HashMap::new();

    for i in 0..=maxRows {
        let mut row: HashMap<u32, ColoredString> = HashMap::new();
        for j in 0..=maxCols {
            row.insert(j as u32, ".".dimmed());
        }
        preview.insert(i as u32, row);
    }

    for part in parts.iter() {
        let matched = symbols.iter().any(|symbol| part.matches(symbol));

        for pos in part.positions.iter() {
            if !preview.contains_key(&pos.y) {
                preview.insert(pos.y, HashMap::default());
            }
            let row = preview.get_mut(&pos.y).unwrap();

            if matched {
                row.insert(pos.x, pos.value.green());
            } else {
                row.insert(pos.x, pos.value.blue());
            }
        }
    }
    for pos in symbols.iter() {
        let row = preview.get_mut(&pos.y).unwrap();
        row.insert(pos.x, pos.value.red());
    }

    let mut rows = preview.keys().into_iter().map(|k| k.clone()).collect::<Vec<u32>>();
    rows.sort();

    for row in rows {
        let items = preview.get(&row).unwrap();
        let mut cols = items.keys().into_iter().map(|k| k.clone()).collect::<Vec<u32>>();
        cols.sort();

        for col in cols.iter() {
            let item = items.get(col).unwrap();
            print!("{}", item);
        }
        println!("");
    }
}
