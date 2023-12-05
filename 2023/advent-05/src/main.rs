use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, cell::RefCell};
use rayon::prelude::*;

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut seeds: Vec<u64> = vec![];
    let mut conversions: HashMap<Mode, Conversions> = HashMap::new();
    let mut mode = Mode::Seeds;
    conversions.insert(Mode::SeedToSoil, Conversions::default());
    conversions.insert(Mode::SoilToFertilizer, Conversions::default());
    conversions.insert(Mode::FertilizerToWater, Conversions::default());
    conversions.insert(Mode::WaterToLight, Conversions::default());
    conversions.insert(Mode::LightToTemperature, Conversions::default());
    conversions.insert(Mode::TemperatureToHumidity, Conversions::default());
    conversions.insert(Mode::HumidityToLocation, Conversions::default());

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    match mode {
                        Mode::Seeds => mode = Mode::SeedToSoil,
                        Mode::SeedToSoil => mode = Mode::SoilToFertilizer,
                        Mode::SoilToFertilizer => mode = Mode::FertilizerToWater,
                        Mode::FertilizerToWater => mode = Mode::WaterToLight,
                        Mode::WaterToLight => mode = Mode::LightToTemperature,
                        Mode::LightToTemperature => mode = Mode::TemperatureToHumidity,
                        Mode::TemperatureToHumidity => mode = Mode::HumidityToLocation,
                        _ => {}
                    }
                }
                if line.is_empty() {
                    continue;
                } else if line.starts_with("seed-to-soil") {
                    mode = Mode::SeedToSoil;
                    continue;
                } else if line.starts_with("soil-to-fertilizer") {
                    mode = Mode::SoilToFertilizer;
                    continue;
                } else if line.starts_with("fertilizer-to-water") {
                    mode = Mode::FertilizerToWater;
                    continue;
                } else if line.starts_with("water-to-light") {
                    mode = Mode::WaterToLight;
                    continue;
                } else if line.starts_with("light-to-temperature") {
                    mode = Mode::LightToTemperature;
                    continue;
                } else if line.starts_with("temperature-to-humidity") {
                    mode = Mode::TemperatureToHumidity;
                    continue;
                } else if line.starts_with("humidity-to-location") {
                    mode = Mode::HumidityToLocation;
                    continue;
                }

                match mode {
                    Mode::Seeds => {
                        seeds = line.replace("seeds:", "").trim().split(" ").map(|str| str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                    }
                    _ => {
                        let split = line.split(" ").map(|str| str.parse::<u64>().unwrap()).collect::<Vec<u64>>();

                        let item = conversions.get_mut(&mode).unwrap();
                        item.ranges.push(Conversion { src: split[1], dst: split[0], num: split[2] });
                    }
                }
            }
            _ => {}
        }
    }

    println!("Seeds -> {:?}", seeds);
    // println!("Collections -> {:?}", conversions);

    let dayOne = seeds.iter().map(|seed| convert(&conversions, seed.clone())).min();

    let mut parts = vec![];
    for i in 0..seeds.len() / 2 {
        parts.push(i);
    }

    let dayTwo = parts.par_iter()
        .map(|i| {
            let mut results = vec![];
            let start_index = i * 2;
            let end_index = start_index + 1;
            println!("Running {i} : {start_index} to {end_index}");
            let start = seeds[start_index];
            let end = seeds[end_index];
            for x in start..start + end {
                results.push(convert(&conversions, x));
            }
            println!("Done {i}");
            return results;
        }).flatten().min();

    println!("dayOne : {:?}", dayOne);
    println!("dayTwo : {:?}", dayTwo);
}

fn convert(conversions: &HashMap<Mode, Conversions>, seed: u64) -> u64 {
    let end = conversions.get(&Mode::SeedToSoil).unwrap().convert(seed);
    let end = conversions.get(&Mode::SoilToFertilizer).unwrap().convert(end);
    let end = conversions.get(&Mode::FertilizerToWater).unwrap().convert(end);
    let end = conversions.get(&Mode::WaterToLight).unwrap().convert(end);
    let end = conversions.get(&Mode::LightToTemperature).unwrap().convert(end);
    let end = conversions.get(&Mode::TemperatureToHumidity).unwrap().convert(end);
    let end = conversions.get(&Mode::HumidityToLocation).unwrap().convert(end);

    return end;
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Mode {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Default, Debug)]
struct Conversion {
    src: u64,
    dst: u64,
    num: u64,
}

#[derive(Default, Debug)]
struct Conversions {
    ranges: Vec<Conversion>,
}

impl Conversions {
    pub fn convert(&self, value: u64) -> u64 {
        for range in self.ranges.iter() {
            if value >= range.src && value < range.src + range.num {
                return value - range.src + range.dst;
            }
        }

        return value;
    }
}

mod test {
    use crate::{Conversions, Conversion};

    #[test]
    pub fn testSeedToSoil() {
        let mut seedToSoil = Conversions::default();
        seedToSoil.ranges.push(Conversion { src: 98, dst: 50, num: 2 });
        seedToSoil.ranges.push(Conversion { src: 50, dst: 52, num: 45 });

        // assert_eq!(seedToSoil.convert(48), 48);
        // assert_eq!(seedToSoil.convert(50), 52);
        // assert_eq!(seedToSoil.convert(98), 50);
        // assert_eq!(seedToSoil.convert(99), 51);
        assert_eq!(seedToSoil.convert(79), 81);
        assert_eq!(seedToSoil.convert(14), 14);
        assert_eq!(seedToSoil.convert(55), 57);
        assert_eq!(seedToSoil.convert(13), 13);
    }

    #[test]
    pub fn testConvertAll() {
        let start = 79;

        let mut seedToSoil = Conversions::default();
        seedToSoil.ranges.push(Conversion { src: 98, dst: 50, num: 2 });
        seedToSoil.ranges.push(Conversion { src: 50, dst: 52, num: 45 });

        let mut soilToFertilizer = Conversions::default();
        soilToFertilizer.ranges.push(Conversion { src: 15, dst: 0, num: 37 });
        soilToFertilizer.ranges.push(Conversion { src: 52, dst: 37, num: 2 });
        soilToFertilizer.ranges.push(Conversion { src: 0, dst: 39, num: 15 });

        let mut fertilizerToWater = Conversions::default();
        fertilizerToWater.ranges.push(Conversion { src: 53, dst: 49, num: 8 });
        fertilizerToWater.ranges.push(Conversion { src: 11, dst: 0, num: 42 });
        fertilizerToWater.ranges.push(Conversion { src: 0, dst: 42, num: 7 });
        fertilizerToWater.ranges.push(Conversion { src: 7, dst: 57, num: 4 });

        let mut waterToLight = Conversions::default();
        waterToLight.ranges.push(Conversion { src: 18, dst: 88, num: 7 });
        waterToLight.ranges.push(Conversion { src: 25, dst: 18, num: 70 });

        let mut lightToTemperature = Conversions::default();
        lightToTemperature.ranges.push(Conversion { src: 77, dst: 45, num: 23 });
        lightToTemperature.ranges.push(Conversion { src: 45, dst: 81, num: 19 });
        lightToTemperature.ranges.push(Conversion { src: 64, dst: 68, num: 13 });

        let mut temperatureToHumidity = Conversions::default();
        temperatureToHumidity.ranges.push(Conversion { src: 69, dst: 0, num: 1 });
        temperatureToHumidity.ranges.push(Conversion { src: 0, dst: 1, num: 69 });

        let mut humidityToLocation = Conversions::default();
        humidityToLocation.ranges.push(Conversion { src: 56, dst: 60, num: 37 });
        humidityToLocation.ranges.push(Conversion { src: 93, dst: 56, num: 4 });

        let end = seedToSoil.convert(start);
        println!("seedToSoil : {end}");
        let end = soilToFertilizer.convert(end);
        println!("soilToFertilizer : {end}");
        let end = fertilizerToWater.convert(end);
        println!("fertilizerToWater : {end}");
        let end = waterToLight.convert(end);
        println!("waterToLight : {end}");
        let end = lightToTemperature.convert(end);
        println!("lightToTemperature : {end}");
        let end = temperatureToHumidity.convert(end);
        println!("temperatureToHumidity : {end}");
        let end = humidityToLocation.convert(end);
        println!("humidityToLocation : {end}");

        assert_eq!(end, 82);
    }
}
