use std::{
    cmp::{max, min},
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let execute_part1 = false;
    let mut reading_grid = true;
    let mut height = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut robot: (i32, i32) = (0, 0);
    let mut moves: Vec<char> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line2 = line.unwrap();
            if line2.len() == 0 {
                assert!(reading_grid);
                reading_grid = false;
                continue;
            }
            if reading_grid {
                let row: Vec<char> = line2.chars().collect();
                if !execute_part1 {
                    grid.push(Vec::new())
                }
                if !execute_part1 {
                    for (i, c) in row.iter().enumerate() {
                        match c {
                            '#' => {
                                grid[height].push('#');
                                grid[height].push('#');
                            }
                            '@' => {
                                robot = (height as i32, i as i32 * 2);
                                grid[height].push('@');
                                grid[height].push('.');
                            }
                            'O' => {
                                grid[height].push('[');
                                grid[height].push(']');
                            }
                            '.' => {
                                grid[height].push('.');
                                grid[height].push('.');
                            }
                            _ => {
                                panic!("Unexpected character");
                            }
                        }
                    }
                }
                height += 1;
                if execute_part1 {
                    grid.push(row);
                }
            } else {
                for c in line2.chars() {
                    moves.push(c);
                }
            }
        }
    }
    //print_grid(&grid);
    //println!("{:?}", moves);
    if execute_part1 {
        part1(&mut grid, robot, &moves);
    } else {
        part2(&mut grid, robot, &moves);
    }
    //print_grid(&grid);
}

fn part2(grid: &mut Vec<Vec<char>>, robot: (i32, i32), moves: &Vec<char>) {
    let mut robot = robot;
    for m in moves {
        let new_position = calculate_new_position(robot, *m, grid).unwrap();
        match grid[new_position.0 as usize][new_position.1 as usize] {
            '.' => {
                grid[robot.0 as usize][robot.1 as usize] = '.';
                grid[new_position.0 as usize][new_position.1 as usize] = '@';
                robot = new_position;
            }
            '#' => {}
            '[' | ']' => {
                robot = part2_move_boxes(new_position, *m, grid);
            }
            _a => {
                eprintln!("Unexpected character: {}", _a);
                panic!("Unexpected character");
            }
        }
        //print_grid(&grid);
    }
    let mut coordinates_sum = 0i64;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '[' {
                coordinates_sum += y as i64 * 100 + x as i64;
            }
        }
    }
    println!("coordinates_sum: {}", coordinates_sum);
}

fn part2_move_boxes(position: (i32, i32), m: char, grid: &mut Vec<Vec<char>>) -> (i32, i32) {
    let mut first_position = calculate_new_position(position, m, grid);
    match m {
        '<' | '>' => {
            'first_position: while first_position.is_some() {
                match grid[first_position.unwrap().0 as usize][first_position.unwrap().1 as usize] {
                    '.' => {
                        break 'first_position;
                    }
                    '#' => {
                        break 'first_position;
                    }
                    '[' | ']' => {}
                    _ => {
                        panic!("Unexpected character");
                    }
                }
                first_position = calculate_new_position(first_position.unwrap(), m, grid);
            }
            // This is cleaner, but the compiler warns for a potential bug and won't compile:
            // "`let` expressions in this position are unstable", see https://github.com/rust-lang/rust/issues/53667
            // That issue has been open since 2018, yikes...
            // if let Some(first_position) = first_position
            //     && grid[first_position.0 as usize][first_position.1 as usize] == '.'

            if !first_position.is_none()
                && grid[first_position.unwrap().0 as usize][first_position.unwrap().1 as usize]
                    == '.'
            {
                let first_position = first_position.unwrap();
                match m {
                    '<' => {
                        for x in (first_position.1..position.1).step_by(2) {
                            grid[position.0 as usize][x as usize] = '[';
                            grid[position.0 as usize][x as usize + 1] = ']';
                        }
                        grid[position.0 as usize][position.1 as usize] = '@';
                        grid[position.0 as usize][position.1 as usize + 1] = '.';
                    }
                    '>' => {
                        for x in ((position.1 + 1)..first_position.1).step_by(2) {
                            grid[position.0 as usize][x as usize] = '[';
                            grid[position.0 as usize][x as usize + 1] = ']';
                        }
                        grid[position.0 as usize][position.1 as usize] = '@';
                        grid[position.0 as usize][position.1 as usize - 1] = '.';
                    }
                    _ => panic!("Unexpected character"),
                }
                return position;
            }
        }
        '^' => {
            let box_side = grid[position.0 as usize][position.1 as usize];
            let mut to_move_up: Vec<(i32, i32)> = match box_side {
                '[' => vec![(position.1, position.1 + 1)],
                ']' => vec![(position.1 - 1, position.1)],
                _ => {
                    panic!("Unexpected character")
                }
            };
            let mut y = position.0 - 1;
            'iter: while y > 0 {
                let last_to_move_up = to_move_up[to_move_up.len() - 1];
                let mut next_to_move_up: (i32, i32) = (grid[0].len() as i32, -1);
                for x in last_to_move_up.0..last_to_move_up.1 + 1 {
                    let grid_element = grid[y as usize][x as usize];
                    match grid_element {
                        '#' => {
                            break 'iter;
                        }
                        '[' => {
                            next_to_move_up =
                                (min(next_to_move_up.0, x), max(next_to_move_up.1, x + 1));
                        }
                        ']' => {
                            next_to_move_up =
                                (min(next_to_move_up.0, x - 1), max(next_to_move_up.1, x));
                        }
                        '.' => {}
                        _ => {
                            panic!("Unexpected character");
                        }
                    }
                }
                // all dots?
                if next_to_move_up == (grid[0].len() as i32, -1) {
                    for move_up in to_move_up.iter().rev() {
                        for x in move_up.0..move_up.1 + 1 {
                            grid[y as usize][x as usize] = grid[y as usize + 1][x as usize];
                            grid[y as usize + 1][x as usize] = '.';
                        }
                        y += 1;
                    }
                    grid[position.0 as usize][position.1 as usize] = '@';
                    if box_side == '[' {
                        grid[position.0 as usize][position.1 as usize + 1] = '.';
                    } else {
                        grid[position.0 as usize][position.1 as usize - 1] = '.';
                    }
                    grid[position.0 as usize + 1][position.1 as usize] = '.';
                    return position;
                }
                to_move_up.push(next_to_move_up);
                y -= 1;
            }
        }
        'v' => {
            let box_side = grid[position.0 as usize][position.1 as usize];
            let mut to_move_down: Vec<(i32, i32)> = match box_side {
                '[' => vec![(position.1, position.1 + 1)],
                ']' => vec![(position.1 - 1, position.1)],
                _ => {
                    panic!("Unexpected character")
                }
            };
            let mut y = position.0 + 1;
            'iter: while y > 0 {
                let last_to_move_down = to_move_down[to_move_down.len() - 1];
                let mut next_to_move_down: (i32, i32) = (grid[0].len() as i32, -1);
                for x in last_to_move_down.0..last_to_move_down.1 + 1 {
                    let grid_element = grid[y as usize][x as usize];
                    match grid_element {
                        '#' => {
                            break 'iter;
                        }
                        '[' => {
                            next_to_move_down =
                                (min(next_to_move_down.0, x), max(next_to_move_down.1, x + 1));
                        }
                        ']' => {
                            next_to_move_down =
                                (min(next_to_move_down.0, x - 1), max(next_to_move_down.1, x));
                        }
                        '.' => {}
                        _ => {
                            panic!("Unexpected character");
                        }
                    }
                }
                // all dots?
                if next_to_move_down == (grid[0].len() as i32, -1) {
                    for move_up in to_move_down.iter().rev() {
                        for x in move_up.0..move_up.1 + 1 {
                            grid[y as usize][x as usize] = grid[y as usize - 1][x as usize];
                            grid[y as usize - 1][x as usize] = '.';
                        }
                        y -= 1;
                    }
                    grid[position.0 as usize][position.1 as usize] = '@';
                    if box_side == '[' {
                        grid[position.0 as usize][position.1 as usize + 1] = '.';
                    } else {
                        grid[position.0 as usize][position.1 as usize - 1] = '.';
                    }
                    grid[position.0 as usize - 1][position.1 as usize] = '.';
                    return position;
                }
                to_move_down.push(next_to_move_down);
                y += 1;
            }
        }
        _ => panic!("Unexpected character"),
    }
    return match m {
        '<' => (position.0, position.1 + 1),
        '>' => (position.0, position.1 - 1),
        '^' => (position.0 + 1, position.1),
        'v' => (position.0 - 1, position.1),
        _ => panic!("Unexpected character"),
    };
}

fn part1(grid: &mut Vec<Vec<char>>, robot: (i32, i32), moves: &Vec<char>) {
    let mut robot = robot;
    for m in moves {
        let new_position = calculate_new_position(robot, *m, grid).unwrap();
        match grid[new_position.0 as usize][new_position.1 as usize] {
            '.' => {
                grid[robot.0 as usize][robot.1 as usize] = '.';
                grid[new_position.0 as usize][new_position.1 as usize] = '@';
                robot = new_position;
            }
            '#' => {}
            'O' => {
                // determine first empty space in the direction of the move
                let mut first_position = calculate_new_position(new_position, *m, grid);
                'first_position: while first_position.is_some() {
                    match grid[first_position.unwrap().0 as usize]
                        [first_position.unwrap().1 as usize]
                    {
                        '.' => {
                            break 'first_position;
                        }
                        '#' => {
                            break 'first_position;
                        }
                        'O' => {}
                        _ => {
                            panic!("Unexpected character");
                        }
                    }
                    first_position = calculate_new_position(first_position.unwrap(), *m, grid);
                }
                if !first_position.is_none()
                    && grid[first_position.unwrap().0 as usize][first_position.unwrap().1 as usize]
                        == '.'
                {
                    grid[robot.0 as usize][robot.1 as usize] = '.';
                    grid[new_position.0 as usize][new_position.1 as usize] = '@';
                    grid[first_position.unwrap().0 as usize][first_position.unwrap().1 as usize] =
                        'O';
                    robot = new_position;
                }
            }
            _ => {
                panic!("Unexpected character");
            }
        }
        //print_grid(&grid);
    }
    let mut coordinates_sum = 0i64;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                coordinates_sum += y as i64 * 100 + x as i64;
            }
        }
    }
    println!("coordinates_sum: {}", coordinates_sum);
}

fn calculate_new_position(
    position: (i32, i32),
    m: char,
    grid: &Vec<Vec<char>>,
) -> Option<(i32, i32)> {
    let new_position = match m {
        '<' => (position.0, position.1 - 1),
        '>' => (position.0, position.1 + 1),
        '^' => (position.0 - 1, position.1),
        'v' => (position.0 + 1, position.1),
        _ => panic!("Unexpected character"),
    };
    if new_position.0 < 0
        || new_position.1 < 0
        || new_position.0 >= grid.len() as i32
        || new_position.1 >= grid[0].len() as i32
    {
        return None;
    }
    Some(new_position)
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
