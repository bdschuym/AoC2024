use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split("   ").collect();
                if parts.len() == 2 {
                    if let Ok(first) = parts[0].trim().parse::<i64>() {
                        first_numbers.push(first);
                    }
                    if let Ok(second) = parts[1].trim().parse::<i64>() {
                        second_numbers.push(second);
                    }
                }
            }
        }
    }
    assert_eq!(
        first_numbers.len(),
        second_numbers.len(),
        "Arrays have different sizes"
    );
    assert_ne!(first_numbers.len(), 0, "Arrays are empty");
    part1(&mut first_numbers, &mut second_numbers);
    part2(&mut first_numbers, &mut second_numbers);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}

fn part1(first_numbers: &mut Vec<i64>, second_numbers: &mut Vec<i64>) {
    first_numbers.sort();
    second_numbers.sort();
    let mut total_distance: i64 = 0;
    for i in 0..first_numbers.len() {
        total_distance += (first_numbers[i] - second_numbers[i]).abs();
    }
    println!("Total distance: {}", total_distance);
}

fn part2(first_numbers: &mut Vec<i64>, second_numbers: &mut Vec<i64>) {
    first_numbers.sort();
    let mut occurrences = HashMap::new();
    for &number in second_numbers.iter() {
        *occurrences.entry(number).or_insert(0) += 1;
    }
    let mut similarity_score: i64 = *occurrences.get(&first_numbers[0]).unwrap_or(&0i64);
    let mut previous_number: i64 = -1;
    let mut previous_num: i64 = -1;
    for i in 1..first_numbers.len() {
        let current_number = first_numbers[i];
        if current_number != previous_number {
            previous_number = current_number;
            previous_num = *occurrences.get(&current_number).unwrap_or(&0i64);
        }
        similarity_score += previous_num * previous_number;
    }
    println!("Similarity score: {}", similarity_score);
}
