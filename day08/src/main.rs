use std::{
    collections::HashMap,
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut antenna_positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut y = 0i32;
    let mut width = 0usize;
    let mut height = 0usize;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                width = l.len();
                let mut x = 0i32;
                for char in l.chars() {
                    if char != '.' {
                        antenna_positions
                            .entry(char)
                            .or_insert_with(Vec::new)
                            .push((y, x));
                    }
                    x += 1;
                }
            }
            y += 1;
            height += 1;
        }
    }
    println!(
        "antenna_positions: {:?}, height: {}, width: {}",
        antenna_positions, height, width
    );
    let execute_part1 = false;
    if execute_part1 {
        part1(&antenna_positions, width, height);
    } else {
        part2(&antenna_positions, width, height);
    }
}

fn part1(antenna_positions: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_antenna, positions) in antenna_positions {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (y1, x1) = positions[i];
                let (y2, x2) = positions[j];
                let y_distance = y2 - y1;
                let x_distance = x2 - x1;
                for antinode in [
                    (y1 - y_distance, x1 - x_distance),
                    (y2 + y_distance, x2 + x_distance),
                ]
                .into_iter()
                {
                    if antinode.0 >= 0
                        && antinode.0 < height as i32
                        && antinode.1 >= 0
                        && antinode.1 < width as i32
                    {
                        antinodes.insert(antinode);
                    }
                }
                // antinodes inbetween (0 found in input example and seem to be excluded by the explanation)
                /*if (x_distance / 3) * 3 == x_distance && (y_distance / 3) * 3 == y_distance {
                    let (y3, x3) = (y1 + y_distance / 3, x1 + x_distance / 3);
                    let (y4, x4) = (y1 + 2 * y_distance / 3, x1 + 2 * x_distance / 3);
                    antinodes.insert((y3, x3));
                    antinodes.insert((y4, x4));
                }*/
            }
        }
    }
    println!("Part 1: {}", antinodes.len());
}

fn part2(antenna_positions: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_antenna, positions) in antenna_positions {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (y1, x1) = positions[i];
                let (y2, x2) = positions[j];
                for (y_distance, x_distance) in [(y2 - y1, x2 - x1), (y1 - y2, x1 - x2)].into_iter()
                {
                    let (mut y1_now, mut x1_now) = positions[i];
                    while y1_now >= 0
                        && y1_now < height as i32
                        && x1_now >= 0
                        && x1_now < width as i32
                    {
                        antinodes.insert((y1_now, x1_now));
                        y1_now += y_distance;
                        x1_now += x_distance;
                    }
                }
            }
        }
    }
    println!("Part 2: {}", antinodes.len());
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
