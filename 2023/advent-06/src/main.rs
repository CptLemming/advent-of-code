use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut dayOneTimes: Vec<u64> = vec![];
    let mut dayOneDistances: Vec<u64> = vec![];
    let mut dayTwoTime = 0;
    let mut dayTwoDistance = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("Time: ") {
                    dayOneTimes = line.replace("Time:", "").trim().split(" ").filter(|str| !str.is_empty()).map(|str| str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                    dayTwoTime = line.replace("Time:", "").replace(" ", "").parse::<u64>().unwrap();
                } else if line.starts_with("Distance: ") {
                    dayOneDistances = line.replace("Distance:", "").trim().split(" ").filter(|str| !str.is_empty()).map(|str| str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                    dayTwoDistance = line.replace("Distance:", "").replace(" ", "").parse::<u64>().unwrap();
                }
            }
            _ => {}
        }
    }

    println!("Time(1) : {dayOneTimes:?}");
    println!("Distance(1) : {dayOneDistances:?}");
    println!("Time(2) : {dayTwoTime:?}");
    println!("Distance(2) : {dayTwoDistance:?}");

    let mut results = vec![];

    for race in 0..dayOneTimes.len() {
        let time = dayOneTimes[race];
        let distance = dayOneDistances[race];

        // println!("Begin race {race}, travel {distance} in {time}");

        let wins = get_no_wins(time, distance);

        println!("Race {race} has {wins} wins");
        results.push(wins);
    }

    let dayOne = results.iter().fold(1, |acc, item| acc * item);
    let dayTwo = get_no_wins(dayTwoTime, dayTwoDistance);

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

fn get_no_wins(time: u64, distance: u64) -> u64 {
    let mut wins = 0;
    for timeHeld in 1..time {
        let remaining = time - timeHeld;
        let travelled = timeHeld * remaining;
        // println!("Hold for {timeHeld} goes {travelled}");
        if travelled > distance {
            // println!("Win {timeHeld}");
            wins += 1;
        }
    }

    return wins;
}