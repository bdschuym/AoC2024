use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut lines = Vec::new();

    if let Ok(input_lines) = read_lines(path) {
        for line in input_lines {
            lines.push(line.unwrap().chars().collect());
        }
    }
    let execute_part1 = false;
    if execute_part1 {
        part1(&lines);
    } else {
        part2(&lines);
    }
}

fn part1(lines: &Vec<Vec<char>>) {
    let mut count = 0;
    // horizontal
    for i in 0..lines.len() {
        for j in 0..lines[i].len() - 3 {
            let k = lines[i].len() - 1 - j;
            if lines[i][j] == 'X'
                && lines[i][j + 1] == 'M'
                && lines[i][j + 2] == 'A'
                && lines[i][j + 3] == 'S'
            {
                count += 1;
            }
            if lines[i][k] == 'X'
                && lines[i][k - 1] == 'M'
                && lines[i][k - 2] == 'A'
                && lines[i][k - 3] == 'S'
            {
                count += 1;
            }
        }
    }
    // vertical
    for i in 0..lines.len() - 3 {
        for j in 0..lines[i].len() {
            let k = lines.len() - 1 - i;
            if lines[i][j] == 'X'
                && lines[i + 1][j] == 'M'
                && lines[i + 2][j] == 'A'
                && lines[i + 3][j] == 'S'
            {
                count += 1;
            }
            if lines[k][j] == 'X'
                && lines[k - 1][j] == 'M'
                && lines[k - 2][j] == 'A'
                && lines[k - 3][j] == 'S'
            {
                count += 1;
            }
        }
    }
    // diagonal 1
    for i in 0..lines.len() - 3 {
        for j in 0..lines[i].len() - 3 {
            let k = lines.len() - 1 - i;
            let l = lines[i].len() - 1 - j;
            if lines[i][j] == 'X'
                && lines[i + 1][j + 1] == 'M'
                && lines[i + 2][j + 2] == 'A'
                && lines[i + 3][j + 3] == 'S'
            {
                count += 1;
            }
            if lines[k][l] == 'X'
                && lines[k - 1][l - 1] == 'M'
                && lines[k - 2][l - 2] == 'A'
                && lines[k - 3][l - 3] == 'S'
            {
                count += 1;
            }
        }
    }
    // diagonal 2
    for i in 3..lines.len() {
        for j in 0..lines[i].len() - 3 {
            let k = i - 3;
            let l = lines[i].len() - 1 - j;
            if lines[i][j] == 'X'
                && lines[i - 1][j + 1] == 'M'
                && lines[i - 2][j + 2] == 'A'
                && lines[i - 3][j + 3] == 'S'
            {
                count += 1;
            }
            if lines[k][l] == 'X'
                && lines[k + 1][l - 1] == 'M'
                && lines[k + 2][l - 2] == 'A'
                && lines[k + 3][l - 3] == 'S'
            {
                count += 1;
            }
        }
    }
    println!("Count: {}", count);
}

fn part2(lines: &Vec<Vec<char>>) {
    let mut count = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[i].len() - 1 {
            if lines[i][j] != 'A' {
                continue;
            }
            let mut sub_count = 0;
            if lines[i - 1][j - 1] == 'M' && lines[i + 1][j + 1] == 'S' {
                sub_count += 1;
            } else if lines[i + 1][j + 1] == 'M' && lines[i - 1][j - 1] == 'S' {
                sub_count += 1;
            }
            if lines[i + 1][j - 1] == 'M' && lines[i - 1][j + 1] == 'S' {
                sub_count += 1;
            } else if lines[i - 1][j + 1] == 'M' && lines[i + 1][j - 1] == 'S' {
                sub_count += 1;
            }
            if sub_count == 2 {
                count += 1;
            }
        }
    }
    println!("Count: {}", count);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
