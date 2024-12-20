use core::hash::{Hash, Hasher};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let use_example_input = true;
    let path = match use_example_input {
        true => "./src/input_example.txt",
        false => "./src/input.txt",
    };
    let minimum_step_save = match use_example_input {
        true => 20,
        false => 100,
    };
    let execute_part1 = false;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_position: Option<(i32, i32)> = None;
    let mut end_position: Option<(i32, i32)> = None;
    let mut height = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            if let Some(x) = line.find('S') {
                start_position = Some((height, x as i32));
            } else if let Some(x) = line.find('E') {
                end_position = Some((height, x as i32));
            }
            grid.push(line.chars().collect());
            height += 1;
        }
    }
    let start_position = start_position.unwrap();
    let end_position = end_position.unwrap();
    if execute_part1 {
        part1(&grid, start_position, end_position, minimum_step_save);
    } else {
        part2(&grid, start_position, end_position, minimum_step_save);
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    x_prev: i32,
    y_prev: i32,
    weight: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position {}
// custom hash function, so that x_previous and y_previous are not considered
impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// For me, part2 was explained very poorly:
// - the text states: "Each cheat has a distinct start position (the position where
//   the cheat is activated, just before the first move that is allowed to go through walls)"
//   I read this as logically meaning that you enter a wall right after the cheat's start position.
//   But that turns out not to be a requirement.
// - I also initially worked to a solution where all wall elements are visited consecutively. That
//   turned out not to be a requirement either.
// - when the cheat is enabled, it turns out that you are also allowed to move through the already
//   walked path. There is no explicit rule stating that you cannot cross the same tile twice, but
//   I find such a solution quite ineligant and did not consider it at first.
//   See !has_path_without_crossing() below to find examples.
// - there's also no mention of the fact that all empty spots are part of the path from S to E.
//   In my solution below, I use this fact.
// It's probably just me, but I found this puzzle disappointing.
fn part2(
    grid: &Vec<Vec<char>>,
    start_position: (i32, i32),
    end_position: (i32, i32),
    minimum_step_save: i32,
) {
    let path = determine_path(grid, start_position, end_position);
    let path_weight = path.len() as i32 - 1; // 84 for the example
    let max_weight = path_weight - minimum_step_save;
    let mut count = 0;
    println!("Path weight: {}", path_weight);
    for i in 0..path.len() - 1 {
        for j in i + minimum_step_save as usize + 2..path.len() {
            let cheat_weight = (path[i].0 - path[j].0).abs() + (path[i].1 - path[j].1).abs();
            if cheat_weight > 20 {
                continue;
            }
            let weight = i as i32 + cheat_weight + (path_weight - j as i32);
            if weight <= max_weight {
                count += 1;
                if false && !has_path_without_crossing(grid, &path[0..i], path[i], path[j]) {
                    // For example: (2, 1)->(9, 1) (weight=14) is a valid cheat, but it crosses itself:
                    // (3,1)->(2, 1)-S>(3, 1)->(4, 1)->(5, 1)->(6, 1)->(7, 1)->(8, 1)-E>(9, 1)->(9,2)->(9,3)->(8,3)->(7,3)->(7,4)->(7,5)
                    //    => position (3,1) is visited twice.
                    println!(
                        "No path without crossing itself: {:?}->{:?} (weight={})",
                        path[i], path[j], weight
                    );
                }
            }
        }
    }
    println!("Number of valid cheats: {}", count);
}

fn has_path_without_crossing(
    grid: &Vec<Vec<char>>,
    path: &[(i32, i32)],
    from: (i32, i32),
    to: (i32, i32),
) -> bool {
    if from == to {
        return true;
    }
    if path.contains(&from) {
        return false;
    }
    // try first down or up
    if from.0 < to.0 {
        if from.0 < grid.len() as i32 - 1 {
            if has_path_without_crossing(grid, path, (from.0 + 1, from.1), to) {
                return true;
            }
        }
    } else if from.0 > to.0 {
        if from.0 > 0 {
            if has_path_without_crossing(grid, path, (from.0 - 1, from.1), to) {
                return true;
            }
        }
    }
    if from.1 < to.1 {
        if from.1 < grid[0].len() as i32 - 1 {
            if has_path_without_crossing(grid, path, (from.0, from.1 + 1), to) {
                return true;
            }
        }
    } else if from.1 > to.1 {
        if from.1 > 0 {
            if has_path_without_crossing(grid, path, (from.0, from.1 - 1), to) {
                return true;
            }
        }
    }
    false
}

///////////////////Part 1///////////////////

fn part1(
    grid: &Vec<Vec<char>>,
    start_position: (i32, i32),
    end_position: (i32, i32),
    minimum_step_save: i32,
) {
    let path = determine_path(grid, start_position, end_position);
    let mut num_valid_cheats = 0;
    let max_weight = path.len() as i32 - minimum_step_save;
    for i in 0..path.len() - 1 {
        let current_position = path[i];
        for next_position_coords in [
            (current_position.0 - 1, current_position.1),
            (current_position.0 + 1, current_position.1),
            (current_position.0, current_position.1 - 1),
            (current_position.0, current_position.1 + 1),
        ] {
            if next_position_coords.0 < 0
                || next_position_coords.1 < 0
                || next_position_coords.0 >= grid.len() as i32
                || next_position_coords.1 >= grid[0].len() as i32
            {
                continue;
            }
            if grid[next_position_coords.0 as usize][next_position_coords.1 as usize] != '#' {
                continue;
            }
            let weight =
                determine_path_weight(grid, &path[0..i + 1], next_position_coords, end_position);
            if let Some(weight) = weight {
                if weight <= max_weight {
                    num_valid_cheats += 1;
                }
            }
        }
    }
    println!("Number of valid cheats: {}", num_valid_cheats);
}

fn determine_path(
    grid: &Vec<Vec<char>>,
    start_position: (i32, i32),
    end_position: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut priority_queue: BinaryHeap<Reverse<Position>> = BinaryHeap::new();
    priority_queue.push(Reverse(Position {
        x: start_position.1,
        y: start_position.0,
        x_prev: -1,
        y_prev: -1,
        weight: 0,
    }));
    let mut visited: HashSet<Position> = HashSet::new();
    while priority_queue.len() > 0 {
        let Reverse(current_position) = priority_queue.pop().unwrap();
        if visited.contains(&current_position) {
            continue;
        }
        visited.insert(current_position);
        if (current_position.y, current_position.x) == end_position {
            //println!("Found: {}", current_position.weight);
            //print_grid2(grid, &visited, current_position);
            break;
        }
        for next_position_coords in [
            (current_position.y - 1, current_position.x),
            (current_position.y + 1, current_position.x),
            (current_position.y, current_position.x - 1),
            (current_position.y, current_position.x + 1),
        ] {
            if next_position_coords.0 < 0
                || next_position_coords.1 < 0
                || next_position_coords.0 >= grid.len() as i32
                || next_position_coords.1 >= grid[0].len() as i32
            {
                continue;
            }
            if grid[next_position_coords.0 as usize][next_position_coords.1 as usize] == '#' {
                continue;
            }
            let next_position = Position {
                x: next_position_coords.1,
                y: next_position_coords.0,
                x_prev: current_position.x,
                y_prev: current_position.y,
                weight: current_position.weight + 1,
            };
            if !visited.contains(&next_position) {
                priority_queue.push(Reverse(next_position));
            }
        }
    }
    if let Some(found_position) = visited.get(&Position {
        x: end_position.1,
        y: end_position.0,
        x_prev: -1,
        y_prev: -1,
        weight: 0,
    }) {
        let mut path: Vec<(i32, i32)> = vec![(0, 0); found_position.weight as usize + 1];
        let mut current_position = found_position;
        let mut index = path.len() - 1;
        loop {
            path[index] = (current_position.y, current_position.x);
            if current_position.x_prev == -1 {
                assert!(index == 0);
                break;
            }
            index -= 1;
            current_position = &visited
                .get(&Position {
                    x: current_position.x_prev,
                    y: current_position.y_prev,
                    x_prev: -1,
                    y_prev: -1,
                    weight: 0,
                })
                .unwrap();
        }
        //print_grid(grid, &path);
        return path;
    }
    return Vec::new();
}

fn determine_path_weight(
    grid: &Vec<Vec<char>>,
    already_visited: &[(i32, i32)],
    start_position: (i32, i32),
    end_position: (i32, i32),
) -> Option<i32> {
    let mut priority_queue: BinaryHeap<Reverse<Position>> = BinaryHeap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(Position {
        x: already_visited[0].1,
        y: already_visited[0].0,
        x_prev: -1,
        y_prev: -1,
        weight: 0,
    });
    for i in 1..already_visited.len() {
        let (y, x) = already_visited[i];
        let (y_prev, x_prev) = already_visited[i - 1];
        visited.insert(Position {
            x,
            y,
            x_prev,
            y_prev,
            weight: i as i32,
        });
    }
    priority_queue.push(Reverse(Position {
        x: start_position.1,
        y: start_position.0,
        x_prev: already_visited[already_visited.len() - 1].1,
        y_prev: already_visited[already_visited.len() - 1].0,
        weight: already_visited.len() as i32,
    }));
    while priority_queue.len() > 0 {
        let Reverse(current_position) = priority_queue.pop().unwrap();
        if visited.contains(&current_position) {
            continue;
        }
        visited.insert(current_position);
        if (current_position.y, current_position.x) == end_position {
            println!("Found: {}", current_position.weight);
            if current_position.weight <= 10 {
                print_grid2(grid, &visited, current_position);
            }
            return Some(current_position.weight);
        }
        for next_position_coords in [
            (current_position.y - 1, current_position.x),
            (current_position.y + 1, current_position.x),
            (current_position.y, current_position.x - 1),
            (current_position.y, current_position.x + 1),
        ] {
            if next_position_coords.0 < 0
                || next_position_coords.1 < 0
                || next_position_coords.0 >= grid.len() as i32
                || next_position_coords.1 >= grid[0].len() as i32
            {
                continue;
            }
            if grid[next_position_coords.0 as usize][next_position_coords.1 as usize] == '#' {
                continue;
            }
            let next_position = Position {
                x: next_position_coords.1,
                y: next_position_coords.0,
                x_prev: current_position.x,
                y_prev: current_position.y,
                weight: current_position.weight + 1,
            };
            if !visited.contains(&next_position) {
                priority_queue.push(Reverse(next_position));
            }
        }
    }
    return None;
}

#[allow(dead_code)]
fn print_grid2(grid: &Vec<Vec<char>>, visited: &HashSet<Position>, mut current_position: Position) {
    let mut path: Vec<(i32, i32)> = vec![(0, 0); current_position.weight as usize + 1];
    let mut index = path.len() - 1;
    loop {
        path[index] = (current_position.y, current_position.x);
        if current_position.x_prev == -1 {
            assert!(index == 0);
            break;
        }
        index -= 1;
        current_position = *visited
            .get(&Position {
                x: current_position.x_prev,
                y: current_position.y_prev,
                x_prev: -1,
                y_prev: -1,
                weight: 0,
            })
            .unwrap();
    }
    print_grid(grid, &path);
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>, path: &Vec<(i32, i32)>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if path.contains(&(y as i32, x as i32)) {
                print!("@");
            } else {
                print!("{}", grid[y][x]);
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
