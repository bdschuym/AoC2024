use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut sums: Vec<i64> = Vec::new();
    let mut operands: Vec<Vec<i64>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let splitted: Vec<&str> = l.split(": ").collect();
                assert_eq!(2, splitted.len(), "2 entries required!");
                sums.push(splitted[0].parse::<i64>().unwrap());
                operands.push(
                    splitted[1]
                        .split(" ")
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect(),
                );
            }
        }
    }
    let execute_part1 = false;
    if execute_part1 {
        part1(&sums, &operands);
    } else {
        part2(&sums, &operands);
    }
}

fn part1(sums: &Vec<i64>, operands: &Vec<Vec<i64>>) {
    let mut count = 0;
    for i in 0..sums.len() {
        // this is the stack (no recursion)
        let mut sub_result = vec![0; operands[i].len()];
        let mut operators: Vec<i32> = vec![0; operands[i].len()]; // first element is dummy

        let mut j = 1;
        sub_result[0] = operands[i][0];
        while j > 0 {
            if operators[j] == 0 {
                sub_result[j] = sub_result[j - 1] + operands[i][j];
            } else {
                sub_result[j] = sub_result[j - 1] * operands[i][j];
            }
            if j == operands[i].len() - 1 {
                if sub_result[j] == sums[i] {
                    count += sums[i];
                    //println!("Found: {} {}, total sum: {}", i, sums[i], count);
                    break;
                }
                j = go_backwards_part1(&mut operators, j);
            } else if sub_result[j] > sums[i] {
                // gotcha: if you use >=, you miss out on the last element equal to 1, with a correct sub_result
                // you'd also miss out on the last element equal to 0, but there's no input with an operand equal to 0
                j = go_backwards_part1(&mut operators, j);
            } else {
                j += 1;
                operators[j] = 0;
            }
        }
    }
    println!("Part 1: {}", count);
}

fn part2(sums: &Vec<i64>, operands: &Vec<Vec<i64>>) {
    let mut count = 0;
    for i in 0..sums.len() {
        // this is the stack (no recursion)
        let mut sub_result = vec![0; operands[i].len()];
        let mut operators: Vec<i32> = vec![0; operands[i].len()]; // first element is dummy

        let mut j = 1;
        sub_result[0] = operands[i][0];
        while j > 0 {
            if operators[j] == 0 {
                sub_result[j] = sub_result[j - 1] + operands[i][j];
            } else if operators[j] == 1 {
                sub_result[j] = sub_result[j - 1] * operands[i][j];
            } else {
                sub_result[j] = sub_result[j - 1] * multiplier(operands[i][j]) + operands[i][j];
            }
            if j == operands[i].len() - 1 {
                if sub_result[j] == sums[i] {
                    count += sums[i];
                    break;
                }
                j = go_backwards_part2(&mut operators, j);
            } else if sub_result[j] > sums[i] {
                // gotcha: if you use >=, you miss out on the last element equal to 1, with a correct sub_result
                // you'd also miss out on the last element equal to 0, but there's no input with an operand equal to 0
                j = go_backwards_part2(&mut operators, j);
            } else {
                j += 1;
                operators[j] = 0;
            }
        }
    }
    println!("Part 1: {}", count);
}

fn multiplier(n: i64) -> i64 {
    let mut m = 10;
    let mut num = n / 10;
    while num > 0 {
        num /= 10;
        m *= 10;
    }
    m
}

fn go_backwards_part1(operators: &mut Vec<i32>, j: usize) -> usize {
    if j <= 0 {
        return 0;
    }
    if operators[j] == 0 {
        operators[j] = 1;
        return j;
    }
    return go_backwards_part1(operators, j - 1);
}

fn go_backwards_part2(operators: &mut Vec<i32>, j: usize) -> usize {
    if j <= 0 {
        return 0;
    }
    operators[j] += 1;
    if operators[j] <= 2 {
        return j;
    }
    return go_backwards_part2(operators, j - 1);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
