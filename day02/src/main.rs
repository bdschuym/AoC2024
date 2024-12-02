use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut reports = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let levels: Vec<i64> = l.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
                assert_ne!(0, levels.len(), "Empty report!");
                reports.push(levels);
            }
        }
    }
    println!("Starting.");
    let execute_part1 = false;
    if execute_part1 {
        part1(&reports);
    } else {
        // measured timings for an input file with 8192000 lines:
        //    part2_bruteforce: 1.25s-1.4s
        //    part2: 123ms-140ms
        let use_bruteforce = false;
        if use_bruteforce {
            part2_bruteforce(&mut reports);
        } else {
            part2(&mut reports);
        }
    }
}

fn is_safe(report: &Vec<i64>) -> bool {
    if report.len() == 1 {
        return true;
    }
    let mut ascending = false;
    if report[0] < report[1] {
        ascending = true;
    }
    let mut safe = true;
    for index in 1..report.len() {
        if ascending {
            if report[index - 1] >= report[index] {
                safe = false;
                break;
            }
            if report[index] - report[index - 1] > 3 {
                safe = false;
                break;
            }
        } else {
            if report[index - 1] <= report[index] {
                safe = false;
                break;
            }
            if report[index - 1] - report[index] > 3 {
                safe = false;
                break;
            }
        }
        if !safe {
            break;
        }
    }
    return safe;
}

fn part1(reports: &Vec<Vec<i64>>) {
    let mut num_safe = 0i64;
    for report in reports {
        if is_safe(report) {
            num_safe += 1;
        }
    }
    println!("Number of safe reports: {:?}", num_safe);
}

// linear time complexity
// lost quite some time debugging, because I was initially too lazy to write the easy bruteforce version...
fn part2(reports: &mut Vec<Vec<i64>>) {
    let mut num_safe = 0i64;
    for report in reports.iter_mut() {
        //println!("        Report: {:?}", report);
        if report.len() == 1 {
            num_safe += 1;
            continue;
        }
        if report.len() == 2 {
            // we can always remove 1 level
            num_safe += 1;
            continue;
        }
        let ascending: Option<bool>;
        // determine ascending or descending, based on at most the first 4 levels
        if (report[0] - report[1]) * (report[1] - report[2]) > 0 {
            if report[0] < report[1] {
                ascending = Some(true);
            } else if report[0] > report[1] {
                ascending = Some(false);
            } else {
                // 3 equal values in a row
                continue;
            }
        } else {
            if report.len() == 3 {
                if (report[0] - report[1]).abs() <= 3
                    || (report[1] - report[2]).abs() <= 3
                    || (report[0] - report[2]).abs() <= 3
                {
                    num_safe += 1;
                } else {
                }
                continue;
            }
            let mut num_descending = 0;
            let mut num_ascending = 0;
            for index in 1..4 {
                if report[index - 1] > report[index] {
                    num_descending += 1;
                } else if report[index - 1] < report[index] {
                    num_ascending += 1;
                }
            }
            if num_descending > num_ascending {
                ascending = Some(false);
            } else {
                ascending = Some(true);
            }
        }
        let mut skipped = false;
        // determine the base level that is required to be the first (after potential removal)
        if ascending.unwrap() {
            if report[0] >= report[1] {
                // we need to remove the first or second level
                skipped = true;
                if !(report[2] - report[1] <= 3 && report[2] - report[1] > 0)
                    && !(report[2] - report[0] <= 3 && report[2] - report[0] > 0)
                {
                    continue;
                }
                // the third level has to remain. If the difference between the third and second level is not ok, we move
                // the first level to the second level for the generic loop that follows.
                if !(report[2] - report[1] <= 3 && report[2] - report[1] > 0) {
                    report[1] = report[0];
                }
            } else if report[1] >= report[2] {
                // we need to remove the second or third level
                skipped = true;
                // if there is no fourth element, we can remove the third level
                if report.len() == 3 {
                    num_safe += 1;
                    continue;
                }
                // The first and fourth level have to remain.
                // If the difference between the fourth and third level is not ok, we move
                // the second level to the third level for the generic loop that follows.
                if !(report[3] - report[2] <= 3 && report[3] - report[2] > 0) {
                    report[2] = report[1];
                }
                // move the first element to the second element for the generic loop that follows.
                report[1] = report[0];
            } else {
                // 3 ascending levels in a row. Removing the second level is useless.
                // we might need to remove the first level, though.
                if (report[1] - report[0]) > 3 {
                    skipped = true;
                }
            }
        } else {
            if report[0] <= report[1] {
                // we need to remove the first or second level
                skipped = true;
                if !(report[1] - report[2] <= 3 && report[1] - report[2] > 0)
                    && !(report[0] - report[2] <= 3 && report[0] - report[2] > 0)
                {
                    continue;
                }
                // the third level has to remain. If the difference between the third and second level is not ok, we move
                // the first level to the second level for the generic loop that follows.
                if !(report[1] - report[2] <= 3 && report[1] - report[2] > 0) {
                    report[1] = report[0];
                }
            } else if report[1] <= report[2] {
                // we need to remove the second or third level
                skipped = true;
                // if there is no fourth element, we can remove the third level
                if report.len() == 3 {
                    num_safe += 1;
                    continue;
                }
                // The first and fourth level have to remain.
                // If the difference between the fourth and third level is not ok, we move
                // the second level to the third level for the generic loop that follows.
                if !(report[2] - report[3] <= 3 && report[2] - report[3] > 0) {
                    report[2] = report[1];
                }
                // move the first element to the second element for the generic loop that follows.
                report[1] = report[0];
            } else {
                // 3 descending levels in a row. Removing the second level is useless.
                // we might need to remove the first level, though.
                if (report[0] - report[1]) > 3 {
                    skipped = true;
                }
            }
        }

        let mut safe = true;
        for index in 2..report.len() {
            let mut local_safe = true;
            if ascending.unwrap() {
                if report[index - 1] >= report[index] {
                    local_safe = false;
                }
                if report[index] - report[index - 1] > 3 {
                    local_safe = false;
                }
            } else {
                if report[index - 1] <= report[index] {
                    local_safe = false;
                }
                if report[index - 1] - report[index] > 3 {
                    local_safe = false;
                }
            }
            if !local_safe {
                if skipped {
                    safe = false;
                    break;
                }
                // remove the level that causes the issue
                report[index] = report[index - 1];
                skipped = true;
            }
        }
        if safe {
            num_safe += 1;
        }
    }
    println!("Number of safe reports: {:?}", num_safe);
}

fn part2_bruteforce(reports: &mut Vec<Vec<i64>>) {
    let mut num_safe = 0i64;
    for report in reports.iter_mut() {
        if is_safe(report) {
            num_safe += 1;
        } else {
            for index in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(index);
                if is_safe(&new_report) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }
    println!("Number of safe reports: {:?}", num_safe);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
