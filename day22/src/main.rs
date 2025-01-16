use std::{
    collections::HashMap,
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let use_example_input = true;
    let execute_part1 = false;
    let path = match use_example_input {
        false => "./src/input.txt",
        true => match execute_part1 {
            true => "./src/input_example.txt",
            false => "./src/input_example2.txt",
        },
    };
    let mut initial_secrets: Vec<u64> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            initial_secrets.push(line.unwrap().parse().unwrap());
        }
    }
    if execute_part1 {
        part1(&initial_secrets);
    } else {
        part2(&initial_secrets);
    }
}

fn part2(initial_secrets: &Vec<u64>) {
    const NUM_ITERATIONS: usize = 2000;
    let mut tuple_sums: HashMap<[i8; 4], u32> = HashMap::new();
    for secret in initial_secrets {
        let mut new_secret = *secret;
        let mut differences_tuple: [i8; 4] = [0, 0, 0, 0];
        let mut visited_tuples: HashSet<[i8; 4]> = HashSet::new();
        for i in 0..NUM_ITERATIONS {
            let old_secret = new_secret;
            new_secret = hash(new_secret);
            let new_difference = ((new_secret % 10) as i64 - (old_secret % 10) as i64) as i8;
            // shift the tuple to the left and insert the new difference
            if false {
                differences_tuple.copy_within(1.., 0);
            } else {
                // manually shifting the tuple is faster. I guess copy_within isn't inlined or isn't optimized for small arrays
                differences_tuple[0] = differences_tuple[1];
                differences_tuple[1] = differences_tuple[2];
                differences_tuple[2] = differences_tuple[3];
            }
            differences_tuple[3] = new_difference;
            if i >= 3 && !visited_tuples.contains(&differences_tuple) {
                let num_bananas = (new_secret % 10) as u32;
                tuple_sums
                    .entry(differences_tuple)
                    .and_modify(|e| *e += num_bananas)
                    .or_insert(num_bananas);
                visited_tuples.insert(differences_tuple);
            }
        }
    }
    let max_bananas = tuple_sums.values().max().unwrap();
    println!("max_bananas: {}", max_bananas);
}

fn part1(initial_secrets: &Vec<u64>) {
    let mut sum: u64 = 0;
    for secret in initial_secrets {
        let mut new_secret = *secret;
        for _ in 0..2000 {
            new_secret = hash(new_secret);
        }
        sum += new_secret;
    }
    println!("sum: {}", sum);
}

fn hash(mut secret: u64) -> u64 {
    if false {
        secret ^= secret * 64;
        secret %= 16777216;
        secret ^= secret / 32;
        secret %= 16777216;
        secret ^= secret * 2048;
        secret %= 16777216;
    } else {
        // this approach is faster (cargo build --release). I'm not sure why the compiler isn't capable of figuring
        // out this optimization.
        secret ^= secret << 6;
        secret &= (1 << 24) - 1;
        secret ^= secret >> 5;
        secret &= (1 << 24) - 1;
        secret ^= secret << 11;
        secret &= (1 << 24) - 1;
    }
    secret
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
