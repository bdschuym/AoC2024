use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // settings for the example
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut patterns: Vec<Vec<char>> = Vec::new();
    let mut designs: Vec<Vec<char>> = Vec::new();

    let execute_part1 = false;
    if let Ok(mut lines) = read_lines(path) {
        // read the first line before the loop
        let first_line = lines.next().unwrap().unwrap();
        first_line
            .split(", ")
            .for_each(|line| patterns.push(line.chars().collect()));
        lines.next(); // skip the empty line
        lines.for_each(|line| designs.push(line.unwrap().chars().collect()));
    }
    if execute_part1 {
        part1(&patterns, &designs);
    } else {
        part2(&patterns, &designs);
    }
}

fn part2(patterns: &Vec<Vec<char>>, designs: &Vec<Vec<char>>) {
    let mut num_possible_designs = 0;
    // with the puzzle input: it's super fast with the HashMap, while it takes forever without the HashMap
    let mut found_sub_patterns: HashMap<String, u64> = HashMap::new();
    for design in designs.iter() {
        num_possible_designs += layout_all_designs(patterns, design, &mut found_sub_patterns);
    }
    println!("Number of possible designs: {}", num_possible_designs);
}

fn layout_all_designs(
    patterns: &Vec<Vec<char>>,
    design: &[char],
    found_sub_patterns: &mut HashMap<String, u64>,
) -> u64 {
    assert!(design.len() > 0);
    let str: String = design.iter().collect();
    if found_sub_patterns.contains_key(&str) {
        return *found_sub_patterns.get(&str).unwrap();
    }
    let mut num_possible_designs = 0;
    for pattern in patterns.iter() {
        if design.len() <= pattern.len() {
            if design == pattern {
                num_possible_designs += 1;
            }
        } else if design[0..pattern.len()] == pattern[..] {
            num_possible_designs +=
                layout_all_designs(patterns, &design[pattern.len()..], found_sub_patterns);
        }
    }
    found_sub_patterns.insert(str, num_possible_designs);
    num_possible_designs
}

fn part1(patterns: &Vec<Vec<char>>, designs: &Vec<Vec<char>>) {
    let mut num_possible_designs = 0;
    for design in designs.iter() {
        if layout_design(patterns, design) {
            num_possible_designs += 1;
        }
    }
    println!("Number of possible designs: {}", num_possible_designs);
}

fn layout_design(patterns: &Vec<Vec<char>>, design: &[char]) -> bool {
    if design.len() == 0 {
        return true;
    }
    for pattern in patterns.iter() {
        if design.len() < pattern.len() {
            continue;
        }
        if design[0..pattern.len()] == pattern[..] {
            if layout_design(patterns, &design[pattern.len()..]) {
                return true;
            }
        }
    }
    false
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
