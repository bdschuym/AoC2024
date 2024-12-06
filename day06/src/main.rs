use std::fmt;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut obstacle_positions: HashSet<(i64, i64)> = HashSet::new();
    let mut guard_position = (-1i64, -1i64);
    let mut guard_direction = Direction::Right;
    let mut width = -1i64;
    let mut height = 0i64;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                if width == -1 {
                    width = l.len() as i64;
                } else {
                    assert_eq!(
                        width,
                        l.len() as i64,
                        "All lines must have the same length!"
                    );
                }
                for (i, c) in l.chars().enumerate() {
                    if c == '#' {
                        obstacle_positions.insert((height, i as i64));
                    } else if c != '.' {
                        if guard_position == (-1, -1) {
                            guard_position = (height, i as i64);
                        } else {
                            panic!("Multiple guards found!");
                        }
                        match c {
                            '^' => guard_direction = Direction::Up,
                            'v' => guard_direction = Direction::Down,
                            '>' => guard_direction = Direction::Right,
                            '<' => guard_direction = Direction::Left,
                            _ => panic!("Invalid guard direction!"),
                        }
                    }
                }
            }
            height += 1;
        }
    }
    println!("Width: {}, Height: {}", width, height);
    println!(
        "Guard position: {:?}, guard direction: {}, obstacle_positions: {:?}",
        guard_position, guard_direction, obstacle_positions
    );
    let visited_positions = part1(
        &obstacle_positions,
        guard_position,
        guard_direction,
        width,
        height,
    );
    part2(
        &mut obstacle_positions,
        guard_position,
        guard_direction,
        &visited_positions,
        width,
        height,
    );
}

fn part1(
    obstacle_positions: &HashSet<(i64, i64)>,
    mut guard_position: (i64, i64),
    mut guard_direction: Direction,
    width: i64,
    height: i64,
) -> HashSet<(i64, i64)> {
    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();
    visited_positions.insert(guard_position.clone());
    while guard_position.0 > 0
        && guard_position.0 < height
        && guard_position.1 > 0
        && guard_position.1 < width
    {
        let next_position: (i64, i64) = match guard_direction {
            Direction::Up => (guard_position.0 - 1, guard_position.1),
            Direction::Down => (guard_position.0 + 1, guard_position.1),
            Direction::Right => (guard_position.0, guard_position.1 + 1),
            Direction::Left => (guard_position.0, guard_position.1 - 1),
        };
        if obstacle_positions.contains(&next_position) {
            match guard_direction {
                Direction::Up => {
                    guard_direction = Direction::Right;
                }
                Direction::Down => {
                    guard_direction = Direction::Left;
                }
                Direction::Right => {
                    guard_direction = Direction::Down;
                }
                Direction::Left => {
                    guard_direction = Direction::Up;
                }
            }
        } else {
            guard_position = next_position;
            visited_positions.insert(next_position);
        }
    }
    println!(
        "Number of visited positions: {}",
        visited_positions.len() - 1
    );
    //print_map(&visited_positions, &obstacle_positions, width, height);
    return visited_positions;
}

fn part2(
    obstacle_positions: &mut HashSet<(i64, i64)>,
    guard_position: (i64, i64),
    guard_direction: Direction,
    visited_positions: &HashSet<(i64, i64)>,
    width: i64,
    height: i64,
) {
    let mut count = 0i64;
    // Since only 1 extra obstacle is added, it suffices to iterate over all visited positions without the added obstacle
    for (i, j) in visited_positions.iter() {
        if obstacle_positions.contains(&(*i, *j)) {
            continue;
        } else if (*i, *j) == guard_position {
            continue;
        }
        obstacle_positions.insert((*i, *j));
        let has_loop = has_loop(
            obstacle_positions,
            guard_position,
            guard_direction,
            width,
            height,
        );
        if has_loop {
            count += 1;
        }
        obstacle_positions.remove(&(*i, *j));
    }
    println!("Number of ways to create a loop: {}", count);
}

fn has_loop(
    obstacle_positions: &HashSet<(i64, i64)>,
    mut guard_position: (i64, i64),
    mut guard_direction: Direction,
    width: i64,
    height: i64,
) -> bool {
    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();
    let mut visited_positions_and_directions: HashSet<(i64, i64, Direction)> = HashSet::new();
    visited_positions.insert(guard_position.clone());
    visited_positions_and_directions.insert((guard_position.0, guard_position.1, guard_direction));
    let mut loop_found = false;
    while guard_position.0 > 0
        && guard_position.0 < height
        && guard_position.1 > 0
        && guard_position.1 < width
    {
        let next_position = match guard_direction {
            Direction::Up => (guard_position.0 - 1, guard_position.1),
            Direction::Down => (guard_position.0 + 1, guard_position.1),
            Direction::Right => (guard_position.0, guard_position.1 + 1),
            Direction::Left => (guard_position.0, guard_position.1 - 1),
        };
        if obstacle_positions.contains(&next_position) {
            match guard_direction {
                Direction::Up => {
                    guard_direction = Direction::Right;
                }
                Direction::Down => {
                    guard_direction = Direction::Left;
                }
                Direction::Right => {
                    guard_direction = Direction::Down;
                }
                Direction::Left => {
                    guard_direction = Direction::Up;
                }
            }
        } else {
            if visited_positions_and_directions.contains(&(
                next_position.0,
                next_position.1,
                guard_direction,
            )) {
                loop_found = true;
                break;
            }
            guard_position = next_position;
            visited_positions.insert(next_position);
            visited_positions_and_directions.insert((
                next_position.0,
                next_position.1,
                guard_direction,
            ));
        }
    }
    return loop_found;
}

#[allow(dead_code)]
fn print_map(
    obstacle_positions: &HashSet<(i64, i64)>,
    visited_positions: &HashSet<(i64, i64)>,
    width: i64,
    height: i64,
) {
    for i in 0..height {
        for j in 0..width {
            if visited_positions.contains(&(i, j)) {
                print!("X");
            }
            if obstacle_positions.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
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
