use core::hash::{Hash, Hasher};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    // settings for the example
    let path = "./src/input_example.txt";
    let index = 12; // arbitrary index from the puzzle text. Was quite confusing, imho.
    let dimension = 7;
    // settings for the puzzle input
    //let path = "./src/input.txt";
    //let index = 1024; // arbitrary index from the puzzle text. Was quite confusing, imho.
    //let dimension = 71; // argh, I first had this set to 70, which yielded no results.

    let execute_part1 = true;
    let mut corrupted_bytes: Vec<(i32, i32)> = Vec::new();
    let mut line_index = 0;
    let mut saved_lines: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            let mut iter = line.split(",");
            let x: i32 = iter.next().unwrap().parse().unwrap();
            let y: i32 = iter.next().unwrap().parse().unwrap();
            saved_lines.push(line);
            line_index += 1;
            corrupted_bytes.push((y, x));
            if execute_part1 && line_index == index {
                break;
            }
        }
    }
    if execute_part1 {
        let mut corrupted_bytes_hashed: HashSet<(i32, i32)> = HashSet::new();
        for (y, x) in corrupted_bytes.iter() {
            corrupted_bytes_hashed.insert((*y, *x));
        }
        println!(
            "Steps: {}",
            part1(&corrupted_bytes_hashed, dimension).unwrap()
        );
    } else {
        // binary search wasn't necessary
        let mut corrupted_bytes_hashed: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..index {
            corrupted_bytes_hashed.insert(corrupted_bytes[i]);
        }
        for i in index..corrupted_bytes.len() {
            corrupted_bytes_hashed.insert(corrupted_bytes[i]);
            if part1(&corrupted_bytes_hashed, dimension).is_none() {
                println!("{}", saved_lines[i]);
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    x_previous: i32,
    y_previous: i32,
    weight: i32,
}
// important detail: the hash set won't work properly unless I let Eq ignore x_previous and y_previous
// The documentation (https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq) states the
// following condition: k1 == k2 -> hash(k1) == hash(k2)
// But if this were the only condition, I would not need to implement the Eq trait.
// Lost like 30 minutes on this detail.
// The hash set will probably first check if the hashes are the same (using Hash) and then check for
// equality of the key, using Eq. This is then the reason why I need to implement Eq.
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

fn part1(corrupted_bytes: &HashSet<(i32, i32)>, dimension: usize) -> Option<i32> {
    let mut heap: BinaryHeap<Reverse<Position>> = BinaryHeap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    heap.push(Reverse(Position {
        x: 0,
        y: 0,
        weight: 0,
        x_previous: -1,
        y_previous: -1,
    }));
    while let Some(Reverse(position)) = heap.pop() {
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);
        for next_position_coords in [
            (position.y - 1, position.x),
            (position.y + 1, position.x),
            (position.y, position.x - 1),
            (position.y, position.x + 1),
        ] {
            if next_position_coords.0 < 0
                || next_position_coords.1 < 0
                || next_position_coords.0 >= dimension as i32
                || next_position_coords.1 >= dimension as i32
            {
                continue;
            }
            let next_position = Position {
                x: next_position_coords.1,
                y: next_position_coords.0,
                weight: position.weight + 1,
                x_previous: position.x,
                y_previous: position.y,
            };
            if visited.contains(&next_position) {
                continue;
            }
            if next_position_coords.0 == dimension as i32 - 1
                && next_position_coords.1 == dimension as i32 - 1
            {
                //print_grid_with_path(&corrupted_bytes, dimension, &visited, next_position);
                return Some(next_position.weight);
            }
            if !corrupted_bytes.contains(&(next_position_coords.0, next_position_coords.1)) {
                heap.push(Reverse(next_position));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn print_grid_with_path(
    corrupted_bytes: &HashSet<(i32, i32)>,
    dimension: usize,
    visited: &HashSet<Position>,
    mut position: Position,
) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; dimension]; dimension];
    for corrupted_byte in corrupted_bytes {
        grid[corrupted_byte.0 as usize][corrupted_byte.1 as usize] = '#';
    }
    grid[0][0] = 'O';
    while position.weight > 0 {
        assert!(!corrupted_bytes.contains(&(position.y, position.x)));
        grid[position.y as usize][position.x as usize] = 'O';
        let previous_position = Position {
            x: position.x_previous,
            y: position.y_previous,
            weight: position.weight - 1,
            x_previous: -1,
            y_previous: -1,
        };
        assert!(visited.contains(&previous_position));
        position = *visited.get(&previous_position).unwrap();
    }
    for y in 0..dimension {
        for x in 0..dimension {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_grid(corrupted_bytes: &HashSet<(i32, i32)>, dimension: usize) {
    for y in 0..dimension {
        for x in 0..dimension {
            if corrupted_bytes.contains(&(y as i32, x as i32)) {
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
