use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example3.txt";
    //let path = "./src/input.txt";
    let mut garden: Vec<Vec<char>> = Vec::new();

    let mut width = 0usize;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line2: Vec<char> = line.unwrap().chars().collect();
            width = line2.len();
            garden.push(line2);
        }
    }
    let height = garden.len();
    let run_part1 = false;
    if run_part1 {
        part1(&mut garden, width, height);
    } else {
        part2(&mut garden, width, height);
    }
}

fn part2(garden: &mut Vec<Vec<char>>, width: usize, height: usize) {
    let mut sum = 0i64;
    for y in 0..height {
        for x in 0..width {
            if garden[y][x].is_lowercase() {
                // garden plot already processed
                continue;
            }
            let garden_plots = fill_region_part2(garden, x, y, width, height);
            sum += calculate_region_value(&garden_plots, garden, garden[y][x], width, height);
        }
    }
    println!("Sum: {}", sum);
}

fn calculate_region_value(
    garden_plots: &HashSet<(usize, usize)>,
    garden: &Vec<Vec<char>>,
    plant_label: char,
    width: usize,
    height: usize,
) -> i64 {
    let mut num_borders = 0i32;
    // 0: left border
    // 1: right border
    // 2: top border
    // 3: bottom border
    for border_type in 0..4 {
        let mut num_specific_borders = 0i32;
        let mut copied_garden_plots = garden_plots.clone();
        for garden_plot in garden_plots {
            if !copied_garden_plots.contains(&garden_plot) {
                continue;
            }
            copied_garden_plots.remove(&garden_plot);
            // only check those that have the considered border
            if border_type == 0 {
                if garden_plot.1 != 0 && garden[garden_plot.0][garden_plot.1 - 1] == plant_label {
                    continue;
                }
            } else if border_type == 1 {
                if garden_plot.1 != width - 1
                    && garden[garden_plot.0][garden_plot.1 + 1] == plant_label
                {
                    continue;
                }
            } else if border_type == 2 {
                if garden_plot.0 != 0 && garden[garden_plot.0 - 1][garden_plot.1] == plant_label {
                    continue;
                }
            } else {
                if garden_plot.0 != height - 1
                    && garden[garden_plot.0 + 1][garden_plot.1] == plant_label
                {
                    continue;
                }
            }
            // We found one garden plot with a border that is part of a side that we are looking for and haven't considered yet.
            // now remove all other garden plots that have a border that is part of the same side.
            num_specific_borders += 1;
            if border_type == 0 || border_type == 1 {
                // first move up the side
                let mut y = (garden_plot.0 as i32) - 1;
                while y >= 0 {
                    if !copied_garden_plots.contains(&(y as usize, garden_plot.1)) {
                        break;
                    }
                    copied_garden_plots.remove(&(y as usize, garden_plot.1));
                    if border_type == 0 {
                        if garden_plot.1 != 0
                            && garden[y as usize][garden_plot.1 - 1] == plant_label
                        {
                            break;
                        }
                    } else {
                        if garden_plot.1 != width - 1
                            && garden[y as usize][garden_plot.1 + 1] == plant_label
                        {
                            break;
                        }
                    }
                    y -= 1;
                }
                // then move down the side
                y = (garden_plot.0 as i32) + 1;
                while y < height as i32 {
                    if !copied_garden_plots.contains(&(y as usize, garden_plot.1)) {
                        break;
                    }
                    copied_garden_plots.remove(&(y as usize, garden_plot.1));
                    if border_type == 0 {
                        if garden_plot.1 != 0
                            && garden[y as usize][garden_plot.1 - 1] == plant_label
                        {
                            break;
                        }
                    } else {
                        if garden_plot.1 != width - 1
                            && garden[y as usize][garden_plot.1 + 1] == plant_label
                        {
                            break;
                        }
                    }
                    y += 1;
                }
            } else {
                // first move to the left on the side
                let mut x = (garden_plot.1 as i32) - 1;
                while x >= 0 {
                    if !copied_garden_plots.contains(&(garden_plot.0, x as usize)) {
                        break;
                    }
                    copied_garden_plots.remove(&(garden_plot.0, x as usize));
                    if border_type == 2 {
                        if garden_plot.0 != 0
                            && garden[garden_plot.0 - 1][x as usize] == plant_label
                        {
                            break;
                        }
                    } else {
                        if garden_plot.0 != height - 1
                            && garden[garden_plot.0 + 1][x as usize] == plant_label
                        {
                            break;
                        }
                    }
                    x -= 1;
                }
                // then move to the right on the side
                x = (garden_plot.1 as i32) + 1;
                while x < width as i32 {
                    if !copied_garden_plots.contains(&(garden_plot.0, x as usize)) {
                        break;
                    }
                    copied_garden_plots.remove(&(garden_plot.0, x as usize));
                    if border_type == 2 {
                        if garden_plot.0 != 0
                            && garden[garden_plot.0 - 1][x as usize] == plant_label
                        {
                            break;
                        }
                    } else {
                        if garden_plot.0 != height - 1
                            && garden[garden_plot.0 + 1][x as usize] == plant_label
                        {
                            break;
                        }
                    }
                    x += 1;
                }
            }
        }
        num_borders += num_specific_borders;
    }
    num_borders as i64 * garden_plots.len() as i64
}

// fills the region and returns all the garden plots in the region
fn fill_region_part2(
    garden: &mut Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let mut directions: Vec<i8> = vec![-1];
    let mut positions: Vec<(i32, i32)> = vec![(y as i32, x as i32)];
    let mut index = 0i32;
    let plant_label = garden[y][x];
    let visited_plant_label = plant_label.to_lowercase().next().unwrap();
    let position_changes: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut garden_plots: HashSet<(usize, usize)> = HashSet::new();
    while index >= 0 {
        let direction = directions[index as usize] + 1;
        if direction == 4 {
            index -= 1;
            continue;
        }
        directions[index as usize] = direction;
        let position = positions[index as usize];
        if direction == 0 {
            if garden[position.0 as usize][position.1 as usize] != plant_label {
                index -= 1;
                continue;
            }
            garden[position.0 as usize][position.1 as usize] = visited_plant_label;
            garden_plots.insert((position.0 as usize, position.1 as usize));
        }
        let new_position = (
            position.0 + (position_changes[direction as usize].0),
            position.1 + (position_changes[direction as usize].1),
        );
        if new_position.0 < 0
            || new_position.0 >= height as i32
            || new_position.1 < 0
            || new_position.1 >= width as i32
        {
            continue;
        }

        index += 1;
        if index == directions.len() as i32 {
            directions.push(-1);
            positions.push((new_position.0, new_position.1));
        } else {
            directions[index as usize] = -1;
            positions[index as usize] = (new_position.0, new_position.1);
        }
    }
    garden_plots
}

fn part1(garden: &mut Vec<Vec<char>>, width: usize, height: usize) {
    let mut sum = 0i64;
    for y in 0..height {
        for x in 0..width {
            if garden[y][x].is_lowercase() {
                // garden plot already processed
                continue;
            }
            sum += fill_region(garden, x, y, width, height);
        }
    }
    println!("Sum: {}", sum);
}

fn fill_region(
    garden: &mut Vec<Vec<char>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> i64 {
    let mut directions: Vec<i8> = vec![-1];
    let mut positions: Vec<(i32, i32)> = vec![(y as i32, x as i32)];
    let mut index = 0i32;
    let mut num_garden_plots = 0i64;
    let mut num_perimiter = 0i64;
    let plant_label = garden[y][x];
    let visited_plant_label = plant_label.to_lowercase().next().unwrap();
    let position_changes: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    while index >= 0 {
        let direction = directions[index as usize] + 1;
        if direction == 4 {
            index -= 1;
            continue;
        }
        directions[index as usize] = direction;
        let position = positions[index as usize];
        if direction == 0 {
            if garden[position.0 as usize][position.1 as usize] != plant_label {
                index -= 1;
                continue;
            }
            garden[position.0 as usize][position.1 as usize] = visited_plant_label;
            for position_change in &position_changes {
                let adjacent_position = (
                    position.0 + position_change.0,
                    position.1 + position_change.1,
                );
                if adjacent_position.0 < 0
                    || adjacent_position.0 >= height as i32
                    || adjacent_position.1 < 0
                    || adjacent_position.1 >= width as i32
                {
                    num_perimiter += 1;
                    continue;
                }
                let adjacent_label =
                    garden[adjacent_position.0 as usize][adjacent_position.1 as usize];
                if adjacent_label != plant_label && adjacent_label != visited_plant_label {
                    num_perimiter += 1;
                    continue;
                }
            }
            num_garden_plots += 1;
        }
        let new_position = (
            position.0 + (position_changes[direction as usize].0),
            position.1 + (position_changes[direction as usize].1),
        );
        if new_position.0 < 0
            || new_position.0 >= height as i32
            || new_position.1 < 0
            || new_position.1 >= width as i32
        {
            continue;
        }

        index += 1;
        if index == directions.len() as i32 {
            directions.push(-1);
            positions.push((new_position.0, new_position.1));
        } else {
            directions[index as usize] = -1;
            positions[index as usize] = (new_position.0, new_position.1);
        }
    }
    num_garden_plots * num_perimiter
}

#[allow(dead_code)]
fn print_garden(garden: &Vec<Vec<char>>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", garden[y][x]);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_garden_plots(garden_plots: &HashSet<(usize, usize)>, width: usize, height: usize) {
    println!("---------");
    for y in 0..height {
        for x in 0..width {
            if garden_plots.contains(&(y, x)) {
                print!("X");
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
