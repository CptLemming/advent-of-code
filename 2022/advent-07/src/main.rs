use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, ops::Add};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("commands.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut directories: HashMap<String, EntryDirectory> = HashMap::new();
    let mut reference: Vec<String> = vec![];

    // for name in vec!["a", "d"] {
    //     if let Entry::Directory(entry) = &mut dir {
    //         entry.add(Entry::Directory(EntryDirectory::new(name.into())));
    //     }
    // }

    // if let Entry::Directory(entry) = &mut dir {
    //     entry.add(Entry::Directory(EntryDirectory::new("d".into())));
    // }

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Build up dir structure of dir / files + sizes
                println!("Dir is {:?}", reference);
                // List dirs under 100000 and sum them
                if line.starts_with("$") {
                    // command
                    let command_parts = line.split(" ").collect::<Vec<&str>>();

                    if command_parts.get(1).unwrap() == &"cd" {
                        if command_parts.get(2).unwrap() == &".." {
                            reference.pop();
                        } else {
                            reference.push(command_parts.get(2).unwrap().to_string());
                        }
                    } else if command_parts.get(1).unwrap() == &"ls" {
                        let dir_path = reference.join("/");
                        let dir_name = reference.last().unwrap().to_string();
                        directories.insert(dir_path, EntryDirectory::new(dir_name));
                    }
                } else {
                    // dir or file
                    let dir_parts = line.split(" ").collect::<Vec<&str>>();
                    
                    if !line.starts_with("dir") {
                        let entry = EntryFile::new(dir_parts.get(1).unwrap().to_string(), dir_parts.get(0).unwrap().parse::<usize>().unwrap());
                        println!("File -> {:?} = {:?}", entry, reference);

                        // TODO Add file to all directories under reference
                        let ref_size = reference.len();

                        for part in 1..=ref_size {
                            let dir_path = reference[0..part].join("/");
                            println!("Add file to -> {}", dir_path);
                            if let Some(dir) = directories.get_mut(&dir_path) {
                                dir.add(entry.clone());
                            }
                        }

                        // let dir_path = reference.join("/");

                        
                    }
                }
                /*
                // COMMANDS
                $ cd /
                $ ls
                dir a
                14848514 b.txt
                8504156 c.dat
                dir d
                $ cd a
                $ ls
                dir e
                29116 f
                2557 g
                62596 h.lst
                $ cd e
                $ ls
                584 i
                $ cd ..
                $ cd ..
                $ cd d
                $ ls
                4060174 j
                8033020 d.log
                5626152 d.ext
                7214296 k

                // PREVIEW
                - / (dir)
                    - a (dir)
                        - e (dir)
                            - i (file, size=584)
                        - f (file, size=29116)
                        - g (file, size=2557)
                        - h.lst (file, size=62596)
                    - b.txt (file, size=14848514)
                    - c.dat (file, size=8504156)
                    - d (dir)
                        - j (file, size=4060174)
                        - d.log (file, size=8033020)
                        - d.ext (file, size=5626152)
                        - k (file, size=7214296)
                */
            }
            _ => {}
        }
    }

    for dir in directories.values() {
        let size = dir.get_size();

        if size < 100_000 {
            println!("{} is {}", dir.name, size);
            result += size;
        }
    }

    println!("Dir -> {:?}", directories);
    println!("Result : {}", result);
}

#[derive(Debug)]
struct EntryDirectory {
    name: String,
    children: Vec<EntryFile>,
}

impl EntryDirectory {
    pub fn new(name: String) -> Self {
        EntryDirectory { name, children: Default::default() }
    }

    pub fn add(&mut self, entry: EntryFile) {
        self.children.push(entry);
    }

    pub fn get_size(&self) -> usize {
        return self.children.iter().map(|file| file.size).sum();
    }
}

#[derive(Debug, Clone)]
struct EntryFile {
    name: String,
    size: usize,
}

impl EntryFile {
    pub fn new(name: String, size: usize) -> Self {
        EntryFile { name, size }
    }
}
