use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read in file
    let file = File::open("assignments.txt").expect("Failed to open file");
    // let file = File::open("test.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // split line into two groups
    // determine if either group *fully* contains the other
    // e.g. 2-4,6-8 = no overlap
    // e.g. 6-6,4-6 = overlap
    // e.g. 2-8,3-7 = overlap
    let mut sum = 0;

    for line in reader.lines() {
        match line {
            Ok(real_line) if !real_line.is_empty() => {
                let groups = real_line.split(",").collect::<Vec<&str>>();
                let first = groups.get(0).unwrap();
                let second = groups.get(1).unwrap();

                let g1 = parse_group(first);
                let g2 = parse_group(second);
                let diff1 = get_range(g1);
                let diff2 = get_range(g2);

                let (base, checker) = if diff1 > diff2 { (g1, g2) } else { (g2, g1) };
                // // println!("parsed -> {:?} - {:?}", base, checker);

                let r1 = get_all_digits(base);
                let r2 = get_all_digits(checker);

                // Check one fully contains the other
                // if r2.is_subset(&r1) {
                //     // println!("Subset");
                //     sum += 1;
                // }

                // Check one contains any from the other
                if r2
                    .intersection(&r1)
                    .into_iter()
                    .collect::<Vec<&u32>>()
                    .len()
                    > 0
                {
                    // println!("Overlapped");
                    sum += 1;
                }
            }
            _ => {}
        }
    }

    println!("Sum : {sum}");
}

fn parse_group(input: &str) -> [u32; 2] {
    let split = input.split("-").collect::<Vec<&str>>();
    let first = split.get(0).unwrap().parse::<u32>().unwrap();
    let second = split.get(1).unwrap().parse::<u32>().unwrap();

    return [first, second];
}

fn get_range(input: [u32; 2]) -> u32 {
    return input.get(1).unwrap() - input.get(0).unwrap();
}

fn get_all_digits(input: [u32; 2]) -> HashSet<u32> {
    return (input.get(0).unwrap().clone()..=input.get(1).unwrap().clone()).collect();
}
