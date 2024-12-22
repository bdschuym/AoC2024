use std::{collections::HashMap, i64};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Direction {
    fn values() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::A,
        ]
        .iter()
        .copied()
    }
}

impl TryFrom<i32> for Direction {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Right),
            4 => Ok(Direction::A),
            _ => Err("Invalid value for Direction"),
        }
    }
}

fn main() {
    let use_example_input = false;
    let execute_part1 = false;
    let codes_input: Vec<&str> = if use_example_input {
        vec!["029A", "980A", "179A", "456A", "379A"]
    } else {
        vec!["169A", "279A", "540A", "869A", "789A"]
    };
    let recursion_depth: i8 = if use_example_input {
        2
    } else {
        if execute_part1 {
            2
        } else {
            25
        }
    };
    let codes: Vec<Vec<i8>> = codes_input
        .iter()
        .map(|code| {
            code.chars()
                .map(|c| c.to_digit(11).unwrap() as i8)
                .collect()
        })
        .collect();
    // determine all valid minimal paths from one numeric keypad button to another
    let mut numeric_keypad_paths: Vec<Vec<Vec<Vec<Direction>>>> = Vec::new();
    for i in 0..11 {
        let row_i: i8 = match i {
            7 | 8 | 9 => 0,
            4 | 5 | 6 => 1,
            1 | 2 | 3 => 2,
            _ => 3,
        };
        let col_i: i8 = match i {
            7 | 4 | 1 => 0,
            8 | 5 | 2 | 0 => 1,
            9 | 6 | 3 | 10 => 2,
            _ => panic!("not possible"),
        };
        let mut vec_i: Vec<Vec<Vec<Direction>>> = Vec::new();
        for j in 0..11 {
            let row_j = match j {
                7 | 8 | 9 => 0,
                4 | 5 | 6 => 1,
                1 | 2 | 3 => 2,
                _ => 3,
            };
            let col_j = match j {
                7 | 4 | 1 => 0,
                8 | 5 | 2 | 0 => 1,
                9 | 6 | 3 | 10 => 2,
                _ => panic!("not possible"),
            };
            let mut vec_j: Vec<Vec<Direction>> = Vec::new();
            let distance = (row_i - row_j).abs() + (col_i - col_j).abs();
            let mut path: Vec<i8> = vec![0; distance as usize];
            loop {
                let mut y = row_i;
                let mut x = col_i;
                let mut directions: Vec<Direction> = Vec::new();
                for p in &path {
                    if x == 0 && y == 3 {
                        break; // the empty button
                    }
                    if *p == 0 {
                        // horizontal move
                        if x == col_j {
                            break;
                        }
                        if x < col_j {
                            directions.push(Direction::Right);
                            x += 1;
                        } else {
                            directions.push(Direction::Left);
                            x -= 1;
                        }
                    } else {
                        // vertical move
                        if y == row_j {
                            break;
                        }
                        if y < row_j {
                            directions.push(Direction::Down);
                            y += 1;
                        } else {
                            directions.push(Direction::Up);
                            y -= 1;
                        }
                    }
                }
                if directions.len() == distance as usize {
                    vec_j.push(directions);
                }
                let mut ix = distance - 1;
                while ix >= 0 {
                    if path[ix as usize] == 0 {
                        path[ix as usize] = 1;
                        break;
                    }
                    path[ix as usize] = 0;
                    ix -= 1;
                }
                if ix < 0 {
                    break;
                }
            }
            vec_i.push(vec_j);
        }
        numeric_keypad_paths.push(vec_i);
    }
    println!("numeric_keypad_paths:");
    // determine all valid minimal paths from one directional keypad button to another
    let mut directional_keypad_paths: Vec<Vec<Vec<Vec<Direction>>>> = Vec::new();
    for i in Direction::values() {
        let row_i: i8 = match i {
            Direction::Up | Direction::A => 0,
            _ => 1,
        };
        let col_i: i8 = match i {
            Direction::Left => 0,
            Direction::Up | Direction::Down => 1,
            _ => 2,
        };
        let mut vec_i: Vec<Vec<Vec<Direction>>> = Vec::new();
        for j in Direction::values() {
            let row_j: i8 = match j {
                Direction::Up | Direction::A => 0,
                _ => 1,
            };
            let col_j: i8 = match j {
                Direction::Left => 0,
                Direction::Up | Direction::Down => 1,
                _ => 2,
            };
            let mut vec_j: Vec<Vec<Direction>> = Vec::new();
            let distance = (row_i - row_j).abs() + (col_i - col_j).abs();
            let mut path: Vec<i8> = vec![0; distance as usize];
            loop {
                let mut y = row_i;
                let mut x = col_i;
                let mut directions: Vec<Direction> = Vec::new();
                for p in &path {
                    if x == 0 && y == 0 {
                        break; // the empty button
                    }
                    if *p == 0 {
                        // horizontal move
                        if x == col_j {
                            break;
                        }
                        if x < col_j {
                            directions.push(Direction::Right);
                            x += 1;
                        } else {
                            directions.push(Direction::Left);
                            x -= 1;
                        }
                    } else {
                        // vertical move
                        if y == row_j {
                            break;
                        }
                        if y < row_j {
                            directions.push(Direction::Down);
                            y += 1;
                        } else {
                            directions.push(Direction::Up);
                            y -= 1;
                        }
                    }
                }
                if directions.len() == distance as usize {
                    vec_j.push(directions);
                }
                let mut ix = distance - 1;
                while ix >= 0 {
                    if path[ix as usize] == 0 {
                        path[ix as usize] = 1;
                        break;
                    }
                    path[ix as usize] = 0;
                    ix -= 1;
                }
                if ix < 0 {
                    break;
                }
            }
            vec_i.push(vec_j);
        }
        directional_keypad_paths.push(vec_i);
    }
    println!("directional_keypad_paths:");

    let state_numeric_keypad = 10;
    let mut result: i64 = 0;

    let mut cache: Vec<HashMap<Vec<Direction>, i64>> =
        vec![HashMap::new(); recursion_depth as usize + 1];
    for code in codes {
        println!("Code: {:?}", code);
        let minimal_path_len = iterate_numeric_keyboard_paths(
            state_numeric_keypad,
            &code,
            &numeric_keypad_paths,
            &directional_keypad_paths,
            Vec::new(),
            &mut cache,
            recursion_depth,
        );
        println!("Minimal path length: {}", minimal_path_len);
        let mut multiplier: i64 = 0;
        for i in 0..code.len() - 1 {
            multiplier *= 10;
            multiplier += code[i] as i64;
        }
        result += multiplier * minimal_path_len as i64;
        /*for path in state.minimal_paths {
            print_directions(&path);
        }*/
    }
    println!("Result: {}", result);
}

fn iterate_numeric_keyboard_paths(
    state_numeric_keypad: i8,
    code: &[i8],
    numeric_keypad_paths: &Vec<Vec<Vec<Vec<Direction>>>>,
    directional_keypad_paths: &Vec<Vec<Vec<Vec<Direction>>>>,
    subpath: Vec<Direction>,
    cache: &mut Vec<HashMap<Vec<Direction>, i64>>,
    recursion_depth: i8,
) -> i64 {
    // this can be done simpler, without recursion, as is done in iterate_directional_keyboard_paths
    if code.len() == 0 {
        if true {
            let mut path_len = 0i64;
            let mut full_subpath: Vec<Direction> = Vec::new();
            let mut code_with_initial_state = vec![Direction::A];
            code_with_initial_state.extend(subpath);
            for ix in 1..code_with_initial_state.len() {
                let (len, path) = iterate_directional_keyboard_paths(
                    code_with_initial_state[ix - 1],
                    &[code_with_initial_state[ix]],
                    &directional_keypad_paths,
                    recursion_depth,
                    cache,
                );
                path_len += len;
                full_subpath.extend(path);
            }
            print_directions(&full_subpath);
            return path_len;
        } else {
            let (len, path) = iterate_directional_keyboard_paths(
                Direction::A,
                &subpath,
                &directional_keypad_paths,
                recursion_depth,
                cache,
            );
            print_directions(&path);
            return len;
        }
    }
    let mut minimal_path_len = i64::MAX;
    for numeric_keyboard_path in
        numeric_keypad_paths[state_numeric_keypad as usize][code[0] as usize].iter()
    {
        let mut new_subpath = subpath.clone();
        new_subpath.extend(numeric_keyboard_path);
        new_subpath.push(Direction::A);
        let new_minimal_path_len = iterate_numeric_keyboard_paths(
            code[0],
            &code[1..],
            numeric_keypad_paths,
            directional_keypad_paths,
            new_subpath,
            cache,
            recursion_depth,
        );
        minimal_path_len = std::cmp::min(minimal_path_len, new_minimal_path_len);
    }
    minimal_path_len
}

fn iterate_directional_keyboard_paths(
    state_directional_keypad: Direction,
    code: &[Direction],
    directional_keypad_paths: &Vec<Vec<Vec<Vec<Direction>>>>,
    recursion_depth: i8,
    cache: &mut Vec<HashMap<Vec<Direction>, i64>>,
) -> (i64, Vec<Direction>) {
    assert!(code.len() != 0);
    let mut code_with_initial_state = vec![state_directional_keypad];
    code_with_initial_state.extend(code);
    if let Some(state) = cache[recursion_depth as usize].get(&code_with_initial_state) {
        return (*state, Vec::new());
    }
    let mut minimal_path_len = i64::MAX;
    let mut chosen_paths: Vec<i8> = vec![0; code_with_initial_state.len() - 1];
    let mut minimal_subpath: Vec<Direction> = Vec::new();
    loop {
        let mut new_subpath: Vec<Direction> = Vec::new();
        for i in 0..code_with_initial_state.len() - 1 {
            new_subpath.extend(
                directional_keypad_paths[code_with_initial_state[i] as usize]
                    [code_with_initial_state[i + 1] as usize][chosen_paths[i] as usize]
                    .iter(),
            );
            new_subpath.push(Direction::A);
        }

        let mut new_minimal_path_len = new_subpath.len() as i64;
        if recursion_depth > 1 {
            let mut new_subpath_with_initial_state = vec![Direction::A];
            new_subpath_with_initial_state.extend(new_subpath.clone());
            new_minimal_path_len = 0i64;
            new_subpath = Vec::new();
            for ix in 1..new_subpath_with_initial_state.len() {
                let (len, path) = iterate_directional_keyboard_paths(
                    new_subpath_with_initial_state[ix - 1],
                    &[new_subpath_with_initial_state[ix]],
                    directional_keypad_paths,
                    recursion_depth - 1,
                    cache,
                );
                new_minimal_path_len += len;
                new_subpath.extend(path);
            }
        }
        if new_minimal_path_len < minimal_path_len {
            minimal_subpath = new_subpath.clone();
            minimal_path_len = new_minimal_path_len;
        }
        // try all possible paths, without recursion to minimize the recursion depth.
        let mut ix = chosen_paths.len() as i8 - 1;
        while ix >= 0 {
            if chosen_paths[ix as usize]
                < directional_keypad_paths[code_with_initial_state[ix as usize] as usize]
                    [code_with_initial_state[ix as usize + 1] as usize]
                    .len() as i8
                    - 1
            {
                chosen_paths[ix as usize] += 1;
                break;
            }
            chosen_paths[ix as usize] = 0;
            ix -= 1;
        }
        if ix < 0 {
            break;
        }
    }
    // the cache is essential to avoid out-of-memory and long execution times
    cache[recursion_depth as usize].insert(code_with_initial_state.to_vec(), minimal_path_len);
    (minimal_path_len, minimal_subpath)
}

#[allow(dead_code)]
fn print_directions(subpath: &Vec<Direction>) {
    for direction in subpath {
        match direction {
            Direction::Up => print!("^"),
            Direction::Down => print!("v"),
            Direction::Left => print!("<"),
            Direction::Right => print!(">"),
            Direction::A => print!("A"),
        }
    }
    println!();
}
