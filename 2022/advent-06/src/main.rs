use std::{fs::File, io::Read, collections::HashSet};

fn main() {
    // read in file
    // let mut file = File::open("test.txt").expect("Failed to open file");
    let mut file = File::open("sequence.txt").expect("Failed to open file");
    let mut content = String::new();
    let reader = file.read_to_string(&mut content).expect("Failed to read file");
    // read sequence
    // find 1st time a sequence of 4x unique chars appears
    // report the char position

    // let message_size = 4usize;
    let message_size = 14usize;

    let mut index = message_size;
    let mut seq = -1i32;

    for window in content.trim().split("").collect::<Vec<&str>>()[1..].windows(message_size) {
        println!("Seq -> {:?}", window);
        let mut set: HashSet<&str> = HashSet::new();
        set.extend(window);

        if set.len() == message_size {
            seq = index as i32;
            break;
        }

        index += 1;

    }

    println!("Sequence is {}", seq);
}
