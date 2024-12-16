use console::Term;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let execute_part1 = false;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_position: Option<Position> = None;
    let mut height = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            if let Some(x) = line.find('S') {
                start_position = Some(Position {
                    x: x as i32,
                    y: height,
                    next_move_no_rotation: false,
                    weight: 0,
                    current_direction: '>',
                });
            }
            grid.push(line.chars().collect());
            height += 1;
        }
    }
    //print_grid(&grid);
    let start_position = start_position.unwrap();
    if execute_part1 {
        part1(&mut grid, start_position);
    } else {
        part2(
            &mut grid,
            PositionWithPrevious {
                x: start_position.x,
                y: start_position.y,
                direction: Direction::Right,
                x_prev: -1,
                y_prev: -1,
                direction_prev: Direction::None,
                weight: 0,
            },
        );
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    None,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct PositionWithPrevious {
    x: i32,
    y: i32,
    direction: Direction,
    x_prev: i32,
    y_prev: i32,
    direction_prev: Direction,
    weight: i32,
}

impl Ord for PositionWithPrevious {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for PositionWithPrevious {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(grid: &mut Vec<Vec<char>>, starting_position: PositionWithPrevious) {
    // [y][x][direction] -> Vec<(y, x, direction)>
    // Holds the various previous positions that reached this state, with the minimal weight.
    // There can be multiple such previous positions if there are different paths to the same state,
    // with the same minimal weight.
    let mut previous_positions: Vec<Vec<Vec<Vec<(i32, i32, Direction)>>>> =
        vec![vec![vec![vec![]; 4]; grid[0].len()]; grid.len()];
    // the determined minimal weight to reach the states
    let mut minimal_weights: Vec<Vec<Vec<i32>>> =
        vec![vec![vec![i32::MAX; 4]; grid[0].len()]; grid.len()];
    // allows picking the next state with the lowest weight
    let mut priority_queue: BinaryHeap<Reverse<PositionWithPrevious>> = BinaryHeap::new();
    priority_queue.push(Reverse(starting_position));
    // these are filled in when the first optimal path is found
    let mut best_path_weight: Option<i32> = None;
    let mut end_position: Option<(i32, i32)> = None;
    while priority_queue.len() > 0 {
        let Reverse(current_position) = priority_queue.pop().unwrap();
        let minimal_weight = minimal_weights[current_position.y as usize]
            [current_position.x as usize][current_position.direction as usize];
        if minimal_weight != i32::MAX {
            // the same state was already visited with the minimum weight
            // if the new path's weight is equal, we can add the new path
            if minimal_weight == current_position.weight {
                previous_positions[current_position.y as usize][current_position.x as usize]
                    [current_position.direction as usize]
                    .push((
                        current_position.y_prev,
                        current_position.x_prev,
                        current_position.direction_prev,
                    ));
            }
            continue;
        }
        // first time we encounter this state
        minimal_weights[current_position.y as usize][current_position.x as usize]
            [current_position.direction as usize] = current_position.weight;
        previous_positions[current_position.y as usize][current_position.x as usize]
            [current_position.direction as usize]
            .push((
                current_position.y_prev,
                current_position.x_prev,
                current_position.direction_prev,
            ));

        // determine the next viable states
        let next_position_straight = calculate_next_position_straight2(
            grid,
            current_position.y,
            current_position.x,
            current_position.direction,
        );
        let mut next_position_left = Option::None;
        if calculate_next_position_straight2(
            grid,
            current_position.y,
            current_position.x,
            rotate_left2(current_position.direction),
        )
        .is_some()
        {
            next_position_left = Some((current_position.y, current_position.x));
        }
        let mut next_position_right = Option::None;
        if calculate_next_position_straight2(
            grid,
            current_position.y,
            current_position.x,
            rotate_left2(rotate_left2(rotate_left2(current_position.direction))),
        )
        .is_some()
        {
            next_position_right = Some((current_position.y, current_position.x));
        }
        // turning back is never a good idea

        for next_position_and_weight in vec![
            (next_position_straight, 1, current_position.direction),
            (
                next_position_left,
                1000,
                rotate_left2(current_position.direction),
            ),
            (
                next_position_right,
                1000,
                rotate_left2(rotate_left2(rotate_left2(current_position.direction))),
            ),
        ] {
            if next_position_and_weight.0.is_none() {
                continue;
            }
            let next_position = next_position_and_weight.0.unwrap();

            if grid[next_position.0 as usize][next_position.1 as usize] == 'E' {
                // we found the end
                let weight = current_position.weight + next_position_and_weight.1;
                if best_path_weight.is_none() {
                    //println!("Found: {}", weight);
                    best_path_weight = Some(weight);
                }
                if end_position.is_none() {
                    end_position = Some((next_position.0, next_position.1));
                }
                if best_path_weight.unwrap() < weight {
                    /*println!(
                        "Path weight too high: {} < {}",
                        best_path_weight.unwrap(),
                        weight
                    );*/
                } else {
                    //println!("Found another: {}", weight);
                    // add the new path
                    previous_positions[next_position.0 as usize][next_position.1 as usize]
                        [next_position_and_weight.2 as usize]
                        .push((
                            current_position.y,
                            current_position.x,
                            current_position.direction,
                        ));
                }
                continue;
            }
            let minimal_weight = minimal_weights[next_position.0 as usize]
                [next_position.1 as usize][next_position_and_weight.2 as usize];
            let weight = current_position.weight + next_position_and_weight.1;
            if minimal_weight == i32::MAX {
                // The state's minimal weight hasn't been calculated yet.
                // The state might already be present in the priority queue with a different previous position.
                // The priority queue will pick the path with the lowest weight.
                priority_queue.push(Reverse(PositionWithPrevious {
                    x: next_position.1,
                    y: next_position.0,
                    direction: next_position_and_weight.2,
                    x_prev: current_position.x,
                    y_prev: current_position.y,
                    direction_prev: current_position.direction,
                    weight: weight,
                }));
            } else {
                // we already obtained a minimal weight for this state: check if the weight for the new path is the same
                assert!(minimal_weight <= weight);
                if minimal_weight == weight {
                    previous_positions[next_position.0 as usize][next_position.1 as usize]
                        [next_position_and_weight.2 as usize]
                        .push((
                            current_position.y,
                            current_position.x,
                            current_position.direction,
                        ));
                }
            }
        }
        //print_grid(&grid);
    }
    assert!(end_position.is_some());
    let mut previous_positions_to_visit: Vec<&Vec<(i32, i32, Direction)>> = Vec::new();
    previous_positions_to_visit.push(
        &previous_positions[end_position.unwrap().0 as usize][end_position.unwrap().1 as usize][0],
    );
    previous_positions_to_visit.push(
        &previous_positions[end_position.unwrap().0 as usize][end_position.unwrap().1 as usize][1],
    );
    previous_positions_to_visit.push(
        &previous_positions[end_position.unwrap().0 as usize][end_position.unwrap().1 as usize][2],
    );
    previous_positions_to_visit.push(
        &previous_positions[end_position.unwrap().0 as usize][end_position.unwrap().1 as usize][3],
    );
    let mut seat_spots: HashSet<(i32, i32)> = HashSet::new();
    seat_spots.insert(end_position.unwrap());
    while previous_positions_to_visit.len() > 0 {
        let current_previous_positions_to_visit = previous_positions_to_visit.pop().unwrap();
        for previous_position in current_previous_positions_to_visit {
            if previous_position.0 < 0 || previous_position.1 < 0 {
                continue;
            }
            seat_spots.insert((previous_position.0, previous_position.1));
            previous_positions_to_visit.push(
                &previous_positions[previous_position.0 as usize][previous_position.1 as usize]
                    [previous_position.2 as usize],
            );
        }
    }
    print_seat_spots(&grid, &seat_spots);
    println!("Num sitting spots: {}", seat_spots.len());
}

fn calculate_next_position_straight2(
    grid: &Vec<Vec<char>>,
    y: i32,
    x: i32,
    direction: Direction,
) -> Option<(i32, i32)> {
    let new_position = (
        match direction {
            Direction::Left | Direction::Right => y,
            Direction::Up => y - 1,
            Direction::Down => y + 1,
            _ => panic!("Invalid direction"),
        },
        match direction {
            Direction::Up | Direction::Down => x,
            Direction::Left => x - 1,
            Direction::Right => x + 1,
            _ => panic!("Invalid direction"),
        },
    );
    if new_position.0 < 0
        || new_position.0 >= grid.len() as i32
        || new_position.1 < 0
        || new_position.1 >= grid[0].len() as i32
        || grid[new_position.0 as usize][new_position.1 as usize] == '#'
    {
        return None;
    }
    Some(new_position)
}

fn rotate_left2(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
        _ => panic!("Invalid direction"),
    }
}

/////////////////part 1////////////////////

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    next_move_no_rotation: bool,
    current_direction: char, // '>': right, '<': left, '^': up, 'v': down
    weight: i32,
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

fn part1(grid: &mut Vec<Vec<char>>, position: Position) {
    let mut priority_queue: BinaryHeap<Reverse<Position>> = BinaryHeap::new();
    priority_queue.push(Reverse(position));
    while priority_queue.len() > 0 {
        let Reverse(current_position) = priority_queue.pop().unwrap();
        grid[current_position.y as usize][current_position.x as usize] = '#';
        let next_position_straight = calculate_next_position_straight(current_position);
        if grid[next_position_straight.y as usize][next_position_straight.x as usize] == 'E' {
            println!("Found: {}", next_position_straight.weight);
            return;
        } else if grid[next_position_straight.y as usize][next_position_straight.x as usize] != '#'
        {
            priority_queue.push(Reverse(next_position_straight));
        }
        if !current_position.next_move_no_rotation {
            let turned_left = Position {
                x: current_position.x,
                y: current_position.y,
                next_move_no_rotation: true,
                weight: current_position.weight + 1000,
                current_direction: rotate_left(current_position.current_direction),
            };
            let turned_left_next_position = calculate_next_position_straight(turned_left);
            if grid[turned_left_next_position.y as usize][turned_left_next_position.x as usize]
                != '#'
            {
                priority_queue.push(Reverse(turned_left));
            }
            let turned_right = Position {
                x: current_position.x,
                y: current_position.y,
                next_move_no_rotation: true,
                weight: current_position.weight + 1000,
                current_direction: rotate_left(rotate_left(rotate_left(
                    current_position.current_direction,
                ))),
            };
            let turned_right_next_position = calculate_next_position_straight(turned_right);
            if grid[turned_right_next_position.y as usize][turned_right_next_position.x as usize]
                != '#'
            {
                priority_queue.push(Reverse(turned_right));
            }
            // turning back is never a good idea
        }
        //print_grid(&grid);
    }
}

fn calculate_next_position_straight(current_position: Position) -> Position {
    Position {
        x: match current_position.current_direction {
            '^' | 'v' => current_position.x,
            '>' => current_position.x + 1,
            '<' => current_position.x - 1,
            _ => panic!("Invalid direction"),
        },
        y: match current_position.current_direction {
            '<' | '>' => current_position.y,
            '^' => current_position.y - 1,
            'v' => current_position.y + 1,
            _ => panic!("Invalid direction"),
        },
        next_move_no_rotation: false,
        weight: current_position.weight + 1,
        current_direction: current_position.current_direction,
    }
}

fn rotate_left(current_direction: char) -> char {
    match current_direction {
        '^' => '<',
        '<' => 'v',
        'v' => '>',
        '>' => '^',
        _ => panic!("Invalid direction"),
    }
}

fn print_seat_spots(grid: &Vec<Vec<char>>, seat_spots: &HashSet<(i32, i32)>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if seat_spots.contains(&(y as i32, x as i32)) {
                print!("O");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_path(
    grid: &Vec<Vec<char>>,
    visited: &Vec<Vec<Vec<Vec<(i32, i32, Direction)>>>>,
    mut position: (i32, i32, Direction),
    minimal_weights: &Vec<Vec<Vec<i32>>>,
) {
    let mut grid2 = grid.clone();
    loop {
        if position.0 < 0 || position.1 < 0 {
            break;
        }
        grid2[position.0 as usize][position.1 as usize] = 'O';
        println!(
            "{:?} : {}",
            position,
            minimal_weights[position.0 as usize][position.1 as usize][position.2 as usize]
        );
        if visited[position.0 as usize][position.1 as usize][position.2 as usize].len() == 0 {
            break;
        }
        position = visited[position.0 as usize][position.1 as usize][position.2 as usize][0];
    }
    print_grid(&grid2);
}

#[allow(dead_code)]
fn print_grid_with_visited(
    grid: &Vec<Vec<char>>,
    visited: &Vec<Vec<Vec<Vec<(i32, i32, Direction)>>>>,
    position: (i32, i32),
) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if y as i32 == position.0 && x as i32 == position.1 {
                print!("@");
            } else if visited[y][x][0].len() > 0 {
                print!("^");
            } else if visited[y][x][1].len() > 0 {
                print!("<");
            } else if visited[y][x][2].len() > 0 {
                print!("v");
            } else if visited[y][x][3].len() > 0 {
                print!(">");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
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

#[allow(dead_code)]
fn wait_for_keypress() {
    let stdout = Term::buffered_stdout();

    loop {
        if let Ok(_character) = stdout.read_char() {
            return;
        }
    }
}
