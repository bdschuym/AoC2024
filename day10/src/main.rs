use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut topographic_map: Vec<Vec<i8>> = Vec::new();
    let mut zero_positions: Vec<(i32, i32)> = Vec::new();

    let mut y = 0usize;
    let mut width = 0i32;
    let mut height = 0i32;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                width = l.len() as i32;
                let mut x = 0usize;
                topographic_map.push(Vec::new());
                for char in l.chars() {
                    if char == '0' {
                        zero_positions.push((y as i32, x as i32));
                    }
                    if char == '.' {
                        topographic_map[y as usize].push(-1);
                    } else {
                        topographic_map[y as usize].push(char.to_digit(10).unwrap() as i8);
                    }
                    x += 1;
                }
            }
            y += 1;
            height += 1;
        }
    }
    println!(
        "topographic map: {:?}, zero positions: {:?}, height: {}, width: {}",
        topographic_map, zero_positions, height, width
    );
    part1and2(&topographic_map, &zero_positions, width, height);
}

fn part1and2(
    topographic_map: &Vec<Vec<i8>>,
    zero_positions: &Vec<(i32, i32)>,
    width: i32,
    height: i32,
) {
    let mut num_found = 0usize;
    let mut num_found_part2 = 0usize;
    for zero_position in zero_positions {
        let mut path: Vec<(i32, i32)> = vec![(0, 0); 10];
        let mut directions: Vec<i8> = vec![0; 10];
        path[0] = *zero_position;
        directions[0] = -1;
        let mut path_index = 0i8;
        let mut nines_found: HashSet<(i32, i32)> = HashSet::new();
        while path_index >= 0 {
            directions[path_index as usize] += 1;
            if directions[path_index as usize] > 3 {
                path_index -= 1;
                continue;
            }
            let next_position = match directions[path_index as usize] {
                0 => (path[path_index as usize].0, path[path_index as usize].1 - 1),
                1 => (path[path_index as usize].0 - 1, path[path_index as usize].1),
                2 => (path[path_index as usize].0, path[path_index as usize].1 + 1),
                3 => (path[path_index as usize].0 + 1, path[path_index as usize].1),
                _ => panic!("Invalid direction!"),
            };
            if next_position.0 < 0
                || next_position.0 >= height
                || next_position.1 < 0
                || next_position.1 >= width
            {
                continue;
            }
            if topographic_map[next_position.0 as usize][next_position.1 as usize] != path_index + 1
            {
                continue;
            }
            if path_index == 8 {
                nines_found.insert(next_position);
                num_found_part2 += 1;
                continue;
            }
            path_index += 1;
            path[path_index as usize] = next_position;
            directions[path_index as usize] = -1;
        }
        num_found += nines_found.len();
    }
    println!("Part 1: {}; part 2: {}", num_found, num_found_part2);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
