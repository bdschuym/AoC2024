use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Clone)]
struct ClawMachine {
    a_move: (i64, i64),
    b_move: (i64, i64),
    prize: (i64, i64),
}

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut index = 0u8;
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let re_a = Regex::new(r"Button A: X\+([0-9]*), Y\+([0-9]*)").unwrap();
    let re_b = Regex::new(r"Button B: X\+([0-9]*), Y\+([0-9]*)").unwrap();
    let re_prize = Regex::new(r"Prize: X=([0-9]*), Y=([0-9]*)").unwrap();
    let mut current_claw_machine: ClawMachine = ClawMachine {
        a_move: (0, 0),
        b_move: (0, 0),
        prize: (0, 0),
    };
    let part2 = true;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line2 = line.unwrap();
            match index {
                0 => {
                    let capsa = re_a.captures(&line2);
                    let caps = capsa.unwrap();
                    current_claw_machine.a_move =
                        (caps[2].parse().unwrap(), caps[1].parse().unwrap());
                }
                1 => {
                    let caps = re_b.captures(&line2).unwrap();
                    current_claw_machine.b_move =
                        (caps[2].parse().unwrap(), caps[1].parse().unwrap());
                }
                2 => {
                    let caps = re_prize.captures(&line2).unwrap();
                    current_claw_machine.prize =
                        (caps[2].parse().unwrap(), caps[1].parse().unwrap());
                    if part2 {
                        current_claw_machine.prize.0 += 10000000000000;
                        current_claw_machine.prize.1 += 10000000000000;
                    }
                    claw_machines.push(current_claw_machine.clone());
                }
                _ => (),
            }
            index += 1;
            if index > 3 {
                index = 0;
            }
        }
    }
    //println!("claw machines: {:?}", claw_machines);
    let mut sum_tokens = 0i64;
    for claw_machine in claw_machines {
        let min_tokens =
            calculate_minimal_moves(claw_machine.a_move, claw_machine.b_move, claw_machine.prize);
        if let Some(value) = min_tokens {
            sum_tokens += value
        }
    }
    println!("sum tokens: {}", sum_tokens);
}

fn calculate_minimal_moves(
    a_move: (i64, i64),
    b_move: (i64, i64),
    prize: (i64, i64),
) -> Option<i64> {
    // Equations:
    // a * a_move.0 + b * b_move.0 = prize.0
    // a * a_move.1 + b * b_move.1 = prize.1
    // Solve for a:
    // a = (prize.0 - b * b_move.0) / a_move.0
    // Solve for b:
    // ((prize.0 - b * b_move.0) / a_move.0) * a_move.1 + b * b_move.1 = prize.1
    // b * b_move.1 - b * b_move.0 * a_move.1 / a_move.0 = prize.1 - prize.0 * a_move.1 / a_move.0
    // b = (prize.1 - prize.0 * a_move.1 / a_move.0) / (b_move.1 - b_move.0 * a_move.1 / a_move.0)
    // b = (prize.1 * a_move.0 - prize.0 * a_move.1) / (b_move.1 * a_move.0 - b_move.0 * a_move.1)
    // Cramer's rule would have been simpler...
    let b = (prize.1 * a_move.0 - prize.0 * a_move.1) / (b_move.1 * a_move.0 - b_move.0 * a_move.1);
    let a = (prize.0 - b * b_move.0) / a_move.0;
    if a * a_move.0 + b * b_move.0 == prize.0 && a * a_move.1 + b * b_move.1 == prize.1 {
        return Some(a * 3 + b);
    }
    return None;
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
