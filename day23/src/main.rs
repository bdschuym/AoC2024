use itertools::Itertools; // needed for the .join in print_max_connected_set()
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
        true => "./src/input_example.txt",
    };
    let mut connections: Vec<[u16; 2]> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            let vertices = line.split("-").collect::<Vec<&str>>();
            assert!(vertices.len() == 2);
            let vertex0 = (vertices[0].chars().nth(0).unwrap() as u16 - 'a' as u16) * 26
                + (vertices[0].chars().nth(1).unwrap() as u16 - 'a' as u16);
            let vertex1 = (vertices[1].chars().nth(0).unwrap() as u16 - 'a' as u16) * 26
                + (vertices[1].chars().nth(1).unwrap() as u16 - 'a' as u16);
            connections.push([vertex0, vertex1]);
        }
    }
    let mut connections_hashmap: HashMap<u16, HashSet<u16>> = HashMap::new();
    for connection in connections {
        connections_hashmap
            .entry(connection[0])
            .and_modify(|e| {
                e.insert(connection[1]);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(connection[1]);
                set
            });
        connections_hashmap
            .entry(connection[1])
            .and_modify(|e| {
                e.insert(connection[0]);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(connection[0]);
                set
            });
    }
    if execute_part1 {
        part1(&connections_hashmap);
    } else {
        part2(&connections_hashmap);
    }
}

fn part2(connections_hashmap: &HashMap<u16, HashSet<u16>>) {
    let mut max_connected_set: HashSet<u16> = HashSet::new();
    // holds vertices for which the max connected set was already determined
    // (essential to speed up the process)
    let mut ignored = HashSet::new();
    for (from, current_connected_set_neighbours) in connections_hashmap.iter() {
        let mut current_connected_set = HashSet::new();
        current_connected_set.insert(*from);
        obtain_max_connected_set(
            connections_hashmap,
            current_connected_set,
            current_connected_set_neighbours,
            &ignored,
            &mut max_connected_set,
        );
        ignored.insert(*from);
    }
    print_max_connected_set(&max_connected_set);
}

fn obtain_max_connected_set(
    connections_hashmap: &HashMap<u16, HashSet<u16>>,
    mut current_connected_set: HashSet<u16>,
    current_connected_set_neighbours: &HashSet<u16>,
    ignored: &HashSet<u16>,
    max_connected_set: &mut HashSet<u16>,
) {
    let mut new_ignored = ignored.clone();
    for neighbour in current_connected_set_neighbours {
        if current_connected_set.contains(neighbour) {
            continue;
        }
        if ignored.contains(neighbour) {
            continue;
        }
        let mut ok = true;
        for handled_vertex in current_connected_set.iter() {
            if !connections_hashmap
                .get(neighbour)
                .unwrap()
                .contains(handled_vertex)
            {
                ok = false;
                break;
            }
        }
        if !ok {
            new_ignored.insert(*neighbour);
            continue;
        }
        // neighbour can be added to the set: it contains connections to all vertices already in the set
        current_connected_set.insert(*neighbour);
        if current_connected_set.len() > max_connected_set.len() {
            max_connected_set.clear();
            max_connected_set.extend(current_connected_set.iter());
        }
        let mut new_current_connected_set_neighbours = current_connected_set_neighbours.clone();
        new_current_connected_set_neighbours.remove(neighbour);
        for neighbour_of_neighbour in connections_hashmap.get(neighbour).unwrap() {
            if !current_connected_set.contains(neighbour_of_neighbour) {
                new_current_connected_set_neighbours.insert(*neighbour_of_neighbour);
            }
        }
        obtain_max_connected_set(
            connections_hashmap,
            current_connected_set.clone(),
            &new_current_connected_set_neighbours,
            &new_ignored,
            max_connected_set,
        );
        current_connected_set.remove(neighbour);
        new_ignored.insert(*neighbour);
    }
}

fn part1(connections_hashmap: &HashMap<u16, HashSet<u16>>) {
    let mut cycles: HashSet<[u16; 3]> = HashSet::new();
    for (from, tos) in connections_hashmap.iter() {
        for to in tos {
            if let Some(to_tos) = connections_hashmap.get(to) {
                for to_to in to_tos {
                    if let Some(to_to_tos) = connections_hashmap.get(to_to) {
                        if to_to_tos.contains(from) {
                            let mut cycle = [*from, *to, *to_to];
                            cycle.sort();
                            cycles.insert(cycle);
                        }
                    }
                }
            }
        }
    }
    let mut num_t_cycles = 0;
    for cycle in cycles {
        for i in 0..3 {
            if cycle[i] / 26 == ('t' as u16 - 'a' as u16) {
                num_t_cycles += 1;
                if false {
                    println!(
                        "{}{},{}{},{}{}",
                        (cycle[0] / 26 + 'a' as u16) as u8 as char,
                        (cycle[0] % 26 + 'a' as u16) as u8 as char,
                        (cycle[1] / 26 + 'a' as u16) as u8 as char,
                        (cycle[1] % 26 + 'a' as u16) as u8 as char,
                        (cycle[2] / 26 + 'a' as u16) as u8 as char,
                        (cycle[2] % 26 + 'a' as u16) as u8 as char
                    );
                }
                break;
            }
        }
    }
    println!("num_t_cycles: {}", num_t_cycles);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}

fn print_max_connected_set(set: &HashSet<u16>) {
    let mut sorted_values: Vec<u16> = set.iter().copied().collect();
    sorted_values.sort();
    let result = sorted_values
        .iter()
        .map(|&value| {
            format!(
                "{}{}",
                (value / 26 + 'a' as u16) as u8 as char,
                (value % 26 + 'a' as u16) as u8 as char
            )
        })
        .join(",");

    println!("{}", result);
}

#[allow(dead_code)]
fn print_connections(connections: &Vec<[u16; 2]>) {
    for connection in connections {
        print_connection(connection);
    }
}

#[allow(dead_code)]
fn print_connection(connection: &[u16; 2]) {
    println!(
        "{}{}-{}{}",
        (connection[0] / 26 + 'a' as u16) as u8 as char,
        (connection[0] % 26 + 'a' as u16) as u8 as char,
        (connection[1] / 26 + 'a' as u16) as u8 as char,
        (connection[1] % 26 + 'a' as u16) as u8 as char
    );
}
