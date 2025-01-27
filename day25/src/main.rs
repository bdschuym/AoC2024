use day25::part1;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let use_example_input = true;
    let path = match use_example_input {
        false => "./src/input.txt",
        true => "./src/input_example.txt",
    };
    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();
    let mut start_new_read = true;
    let mut reading_lock = true;
    let mut line_ix = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            if line.len() == 0 {
                start_new_read = true;
                continue;
            }
            if start_new_read {
                start_new_read = false;
                reading_lock = line == "#####";
                if reading_lock {
                    locks.push([0; 5]);
                } else {
                    keys.push([0; 5]);
                }
                line_ix = 0;
                continue;
            }
            if reading_lock {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        locks.last_mut().unwrap()[i] += 1;
                    }
                }
            } else if line_ix < 5 {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        keys.last_mut().unwrap()[i] += 1;
                    }
                }
            }
            line_ix += 1;
        }
    }
    //println!("locks: {:?}", locks);
    //println!("keys: {:?}", keys);
    println!("num pairs: {}", part1(&keys, &locks)); // there is no second part
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
