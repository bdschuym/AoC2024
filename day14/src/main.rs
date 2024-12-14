use console::Term;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Clone)]
struct Robot {
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

fn main() {
    let path = "./src/input_example.txt";
    let width = 11;
    let height = 7;
    //let path = "./src/input.txt";
    //let width = 101;
    //let height = 103;
    let mut robots: Vec<Robot> = Vec::new();
    let re = Regex::new(r"p=([0-9]*),([0-9]*) v=(-?[0-9]*),(-?[0-9]*)").unwrap();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line2 = line.unwrap();
            let caps = re.captures(&line2).unwrap();
            let robot = Robot {
                position_x: caps[1].parse().unwrap(),
                position_y: caps[2].parse().unwrap(),
                velocity_x: caps[3].parse().unwrap(),
                velocity_y: caps[4].parse().unwrap(),
            };
            robots.push(robot);
        }
    }
    /*for robot in robots {
        println!(
            "p={},{} v={},{}",
            robot.position_x, robot.position_y, robot.velocity_x, robot.velocity_y
        );
    }*/
    let execute_part1 = false;
    if execute_part1 {
        part1(&mut robots, width, height);
    } else {
        part2(&mut robots, width, height);
    }
}

fn part2(robots: &mut Vec<Robot>, width: i32, height: i32) {
    assert!(width % 2 == 1);
    assert!(height % 2 == 1);
    //print_robot_positions(robots, width, height);
    for second in 0..100000 {
        for robot in robots.iter_mut() {
            robot.position_x += robot.velocity_x;
            robot.position_y += robot.velocity_y;
            if robot.position_x < 0 {
                robot.position_x += width;
            }
            if robot.position_y < 0 {
                robot.position_y += height;
            }
            if robot.position_x >= width {
                robot.position_x -= width;
            }
            if robot.position_y >= height {
                robot.position_y -= height;
            }
        }
        // For this I had to take a hint that there was a regularity in the output every n seconds.
        // for my puzzle it was every 103 seconds. Then just brute force click enter until your eyes
        // see a christmas tree...
        if (second - 30) % 103 != 0 {
            continue;
        }
        println!("second: {}", second + 1);
        print_robot_positions(robots, width, height);
        wait_for_keypress();
    }
}

fn part1(robots: &mut Vec<Robot>, width: i32, height: i32) {
    assert!(width % 2 == 1);
    assert!(height % 2 == 1);
    //print_robot_positions(robots, width, height);
    for _second in 0..100 {
        for robot in robots.iter_mut() {
            robot.position_x += robot.velocity_x;
            robot.position_y += robot.velocity_y;
            if robot.position_x < 0 {
                robot.position_x += width;
            }
            if robot.position_y < 0 {
                robot.position_y += height;
            }
            if robot.position_x >= width {
                robot.position_x -= width;
            }
            if robot.position_y >= height {
                robot.position_y -= height;
            }
        }
        print_robot_positions(robots, width, height);
    }
    print_robot_positions(robots, width, height);
    // calculate safety factor
    let quadrant_width = width / 2;
    let quadrant_height = height / 2;
    let mut num_quadrant1 = 0i32;
    let mut num_quadrant2 = 0i32;
    let mut num_quadrant3 = 0i32;
    let mut num_quadrant4 = 0i32;
    for robot in robots.iter_mut() {
        if robot.position_x < quadrant_width && robot.position_y < quadrant_height {
            num_quadrant1 += 1;
        } else if robot.position_x > quadrant_width && robot.position_y < quadrant_height {
            num_quadrant2 += 1;
        } else if robot.position_x < quadrant_width && robot.position_y > quadrant_height {
            num_quadrant3 += 1;
        } else if robot.position_x > quadrant_width && robot.position_y > quadrant_height {
            num_quadrant4 += 1;
        }
    }
    let safety_factor = num_quadrant1 * num_quadrant2 * num_quadrant3 * num_quadrant4;
    println!("safety factor: {}", safety_factor);
}

fn print_robot_positions(robots: &Vec<Robot>, width: i32, height: i32) {
    println!("=====================");
    for y in 0..height {
        for x in 0..width {
            let mut num_robots = 0;
            for robot in robots.iter() {
                if robot.position_x == x && robot.position_y == y {
                    num_robots += 1;
                }
            }
            if num_robots > 0 {
                print!("{}", num_robots);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn wait_for_keypress() {
    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(_character) = stdout.read_char() {
            return;
        }
    }
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
