use std::{fs::File, io::{BufReader, BufRead}, cell::RefCell};
use colored::{Colorize, ColoredString};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    // let file = File::open("test2.txt").expect("Failed to open file");
    let file = File::open("trees.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut buffer: Vec<Vec<RefCell<Cell>>> = Vec::new();
    let mut col_size = 0;

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(line) if !line.is_empty() => {
                col_size = line.len();
                buffer.push(
                    line
                        .chars()
                        .into_iter()
                        .enumerate()
                        .map(|(j, item)| RefCell::new(Cell::new(
                            item.to_string().parse::<u8>().unwrap(),
                            i as u8,
                            j as u8
                        )))
                        .collect::<Vec<RefCell<Cell>>>()
                );
            }
            _ => {}
        }
    }

    let row_size = buffer.len();
    let mut result = 0;

    // North
    let mut north_max: Vec<u8> = vec![0; col_size];
    for (i, row) in buffer.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            let mut max = north_max.get_mut(j).unwrap();
            let mut item = column.borrow_mut();

            if i == 0 {
                item.visible_north();
                *max = item.height;
                continue;
            }

            if &item.height > max {
                item.visible_north();
                *max = item.height;
                continue;
            }
        }
    }

    // East
    let mut east_max: Vec<u8> = vec![0; row_size];
    for (i, row) in buffer.iter().enumerate() {
        for (j, column) in row.iter().enumerate().rev() {
            let mut max = east_max.get_mut(i).unwrap();
            let mut item = column.borrow_mut();

            if j == (col_size - 1) {
                item.visible_east();
                *max = item.height;
                continue;
            }

            if &item.height > max {
                item.visible_east();
                *max = item.height;
                continue;
            }
        }
    }

    // South
    let mut south_max: Vec<u8> = vec![0; row_size];
    for (i, row) in buffer.iter().enumerate().rev() {
        for (j, column) in row.iter().enumerate() {
            let mut max = south_max.get_mut(j).unwrap();
            let mut item = column.borrow_mut();

            if i == (row_size - 1) {
                item.visible_south();
                *max = item.height;
                continue;
            }

            if &item.height > max {
                item.visible_south();
                *max = item.height;
                continue;
            }
        }
    }

    // West
    let mut west_max: Vec<u8> = vec![0; col_size];
    for (i, row) in buffer.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            let mut max = west_max.get_mut(i).unwrap();
            let mut item = column.borrow_mut();

            if j == 0 {
                item.visible_west();
                *max = item.height;
                continue;
            }

            if &item.height > max {
                item.visible_west();
                *max = item.height;
                continue;
            }
        }
    }

    // Results
    for row in buffer.iter() {
        for cell in row {
            // println!("Cell = {:?}", cell);
            print!("{}", cell.borrow().as_color());
            if cell.borrow().is_visible() {
                result += 1;
            }
        }
        println!("");
    }

    // println!("Cols : {}, Rows : {}", column_size, row_size);
    println!("Result : {}", result);
    // println!("Buffer : {:?}", buffer);
}

#[derive(Default, Debug)]
struct Cell {
    height: u8,
    row: u8,
    col: u8,
    north: u8,
    east: u8,
    south: u8,
    west: u8
}

impl Cell {
    pub fn new(height: u8, row: u8, col: u8) -> Self {
        Cell { height, row, col, ..Default::default() }
    }

    pub fn is_visible(&self) -> bool {
        // self.is_visible_north() || self.is_visible_east() || self.is_visible_south() || self.is_visible_west()
        self.is_partial_north() || self.is_partial_east() || self.is_partial_south() || self.is_partial_west()
    }

    pub fn is_partial_north(&self) -> bool {
        self.north > 0
    }
    pub fn is_partial_east(&self) -> bool {
        self.east > 0
    }
    pub fn is_partial_south(&self) -> bool {
        self.south > 0
    }
    pub fn is_partial_west(&self) -> bool {
        self.west > 0
    }

    pub fn is_visible_north(&self) -> bool {
        self.north == 2
    }
    pub fn is_visible_east(&self) -> bool {
        self.east == 2
    }
    pub fn is_visible_south(&self) -> bool {
        self.south == 2
    }
    pub fn is_visible_west(&self) -> bool {
        self.west == 2
    }

    pub fn visible_north(&mut self) {
        self.north = 2;
    }
    pub fn visible_east(&mut self) {
        self.east = 2;
    }
    pub fn visible_south(&mut self) {
        self.south = 2;
    }
    pub fn visible_west(&mut self) {
        self.west = 2;
    }

    pub fn partial_visible_north(&mut self) {
        self.north = 1;
    }
    pub fn partial_visible_east(&mut self) {
        self.east = 1;
    }
    pub fn partial_visible_south(&mut self) {
        self.south = 1;
    }
    pub fn partial_visible_west(&mut self) {
        self.west = 1;
    }

    pub fn as_color(&self) -> ColoredString {
        if self.is_visible() {
            if self.north == 2 {
                return self.height.to_string().red();
            }
            if self.north == 1 {
                return self.height.to_string().on_green().red();
            }
            if self.east == 2 {
                return self.height.to_string().yellow();
            }
            if self.east == 1 {
                return self.height.to_string().on_green().yellow();
            }
            if self.south == 2 {
                return self.height.to_string().blue();
            }
            if self.south == 1 {
                return self.height.to_string().on_green().blue();
            }
            if self.west == 2 {
                return self.height.to_string().cyan();
            }
            if self.west == 1 {
                return self.height.to_string().on_green().cyan();
            }
        }

        return self.height.to_string().dimmed();
    }
}
