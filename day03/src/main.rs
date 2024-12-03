extern crate regex;

use regex::Regex;

use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let execute_part1 = false;
    if execute_part1 {
        if true {
            part1(String::from(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            ));
        } else {
            let input = read_file("input.txt");
            part1(input);
        }
    } else {
        if true {
            part2(String::from(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            ));
        } else {
            let input = read_file("input.txt");
            part2(input);
        }
    }
}

fn part1(input: String) {
    let re = Regex::new(r"mul\(([1-9][0-9]?[0-9]?),([1-9][0-9]?[0-9]?)\)").unwrap();

    let mut sum = 0i64;
    for cap in re.captures_iter(&input) {
        let factor1 = cap[1].parse::<i64>().unwrap();
        let factor2 = cap[2].parse::<i64>().unwrap();
        sum += factor1 * factor2;
    }
    println!("Sum: {}", sum);
}

fn part2(input: String) {
    let mut parsing_mul_arg1 = false;
    let mut parsing_mul_arg2 = false;
    let mut arg_len = 0;
    let mut arg1 = 0;
    let mut arg2 = 0;
    let mut sum = 0;
    let mut mul_enabled = true;
    let input_len = input.len();
    let mut index = 0;
    while index < input_len {
        let char = input[index..].chars().next().unwrap();
        if parsing_mul_arg1 || parsing_mul_arg2 {
            if char.is_digit(10) {
                if arg_len >= 3 {
                    parsing_mul_arg1 = false;
                    parsing_mul_arg2 = false;
                    index += 1;
                } else {
                    if parsing_mul_arg1 {
                        arg1 = arg1 * 10 + char.to_digit(10).unwrap();
                    } else {
                        arg2 = arg2 * 10 + char.to_digit(10).unwrap();
                    }
                    arg_len += 1;
                }
            } else if parsing_mul_arg1 && char == ',' && arg_len > 0 {
                parsing_mul_arg1 = false;
                parsing_mul_arg2 = true;
                arg_len = 0;
                arg2 = 0;
            } else if parsing_mul_arg2 && char == ')' && arg_len > 0 {
                parsing_mul_arg2 = false;
                sum += arg1 * arg2;
            } else {
                parsing_mul_arg1 = false;
                parsing_mul_arg2 = false;
                index -= 1;
            }
        } else if index + 4 <= input_len && "mul(" == &input[index..index + 4] && mul_enabled {
            parsing_mul_arg1 = true;
            index += 3;
            arg_len = 0;
            arg1 = 0;
            arg2 = 0;
        } else if index + 4 <= input_len && "do()" == &input[index..index + 4] {
            mul_enabled = true;
            index += 3;
        } else if index + 7 <= input_len && "don't()" == &input[index..index + 7] {
            mul_enabled = false;
            index += 6;
        }
        index += 1;
    }
    println!("Sum: {}", sum);
}

fn read_file(filename: &str) -> String {
    let mut buf = String::new();
    let file = File::open(filename).expect("Could not open file");
    io::BufReader::new(file).read_to_string(&mut buf).unwrap();
    return buf;
}
