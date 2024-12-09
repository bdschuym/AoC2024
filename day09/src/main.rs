use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";

    let mut disk_map: Vec<i32> = Vec::new();
    let mut input: String = "".to_string();
    let mut file_lengths: HashMap<i32, i32> = HashMap::new();
    let mut empty_spaces: Vec<(usize, i32)> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                input = l;
                break;
            }
        }
    }
    let mut file_id = 0i32;
    let mut empty_space = false;
    let mut block_index = 0usize;
    for digit in input.chars().map(|c| c.to_digit(10).unwrap()) {
        if empty_space {
            // use -1 to indicate empty space
            disk_map.extend(std::iter::repeat(-1i32).take(digit as usize));
            empty_spaces.push((block_index, digit as i32));
        } else {
            disk_map.extend(std::iter::repeat(file_id).take(digit as usize));
            file_lengths.insert(file_id, digit as i32);
            file_id += 1;
        }
        block_index += digit as usize;
        empty_space = !empty_space;
    }
    let execute_part1 = false;
    if execute_part1 {
        part1(&mut disk_map);
    } else {
        let execute_optimized = true;
        if execute_optimized {
            part2_optimized(&mut disk_map, file_lengths, &mut empty_spaces);
        } else {
            part2(&mut disk_map, file_lengths);
        }
    }
}

fn part1(disk_map: &mut Vec<i32>) {
    let mut free_space_index = 0usize;
    for i in (0..disk_map.len()).rev() {
        if disk_map[i] == -1 {
            continue;
        }
        while disk_map[free_space_index] != -1 && free_space_index < i {
            free_space_index += 1;
        }
        if free_space_index >= i {
            break;
        }
        disk_map.swap(i, free_space_index);
        free_space_index += 1;
    }
    println!("disk_map after: {:?}", disk_map);
    let mut sum = 0i64;
    for i in 0..disk_map.len() {
        if disk_map[i] == -1 {
            break;
        }
        sum += disk_map[i] as i64 * (i as i64);
    }
    println!("sum: {}", sum);
}

fn part2(disk_map: &mut Vec<i32>, file_lengths: HashMap<i32, i32>) {
    let mut i = (disk_map.len() - 1) as i32;
    while i >= 0 {
        let file_id = disk_map[i as usize];
        if file_id == -1 {
            i -= 1;
            continue;
        }
        let file_size = *file_lengths.get(&file_id).unwrap();
        // find the first empty space that is big enough
        // optimization possible
        for j in 0..i {
            let mut found = true;
            for k in 0..file_size {
                if disk_map[(j + k) as usize] != -1 {
                    found = false;
                    break;
                }
            }
            if found {
                for k in 0..file_size {
                    disk_map[(j + k) as usize] = file_id;
                    disk_map[(i - file_size + 1 + k) as usize] = -1;
                }
                break;
            }
        }
        i -= file_size;
        //println!("disk_map inbetween: {:?}", disk_map);
    }
    //println!("disk_map after: {:?}", disk_map);
    let mut sum = 0i64;
    for i in 0..disk_map.len() {
        if disk_map[i] != -1 {
            sum += disk_map[i] as i64 * (i as i64);
        }
    }
    println!("sum: {}", sum);
}

fn part2_optimized(
    disk_map: &mut Vec<i32>,
    file_lengths: HashMap<i32, i32>,
    empty_spaces: &mut Vec<(usize, i32)>,
) {
    let mut i = (disk_map.len() - 1) as i32;
    while i >= 0 {
        let file_id = disk_map[i as usize];
        if file_id == -1 {
            i -= 1;
            continue;
        }
        let file_size = *file_lengths.get(&file_id).unwrap();
        // find the first empty space that is big enough
        for (empty_spaces_index, (j, size)) in empty_spaces.iter().enumerate() {
            if *j >= i as usize {
                break;
            }
            if *size >= file_size {
                for k in 0..file_size {
                    disk_map[*j + (k as usize)] = file_id;
                    disk_map[(i - file_size + 1 + k) as usize] = -1;
                }
                empty_spaces[empty_spaces_index].0 += file_size as usize;
                empty_spaces[empty_spaces_index].1 -= file_size;
                break;
            }
        }
        i -= file_size;
        //println!("file_id:{}; disk_map inbetween: {:?}", file_id, disk_map);
        //println!("empty spaces: {:?}", empty_spaces);
    }
    //println!("disk_map after: {:?}", disk_map);
    let mut sum = 0i64;
    for i in 0..disk_map.len() {
        if disk_map[i] != -1 {
            sum += disk_map[i] as i64 * (i as i64);
        }
    }
    println!("sum: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
