use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Operation {
    AND,
    OR,
    XOR,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Operation::AND => "AND",
            Operation::OR => "OR",
            Operation::XOR => "XOR",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Gate {
    operand1: String,
    operand2: String,
    operation: Operation,
}

fn main() {
    // note: the example input is not meaningful for part2
    let use_example_input = true;
    let execute_part1 = true;
    let path = match use_example_input {
        false => "./src/input.txt",
        true => "./src/input_example.txt",
    };
    let mut gate_outputs: HashMap<Gate, String> = HashMap::new();
    let mut connections: HashMap<String, Gate> = HashMap::new();
    let mut values: HashMap<String, bool> = HashMap::new();
    let mut reading_values = true;
    let mut num_z = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let line = line.unwrap();
            if line.len() == 0 {
                reading_values = false;
                continue;
            }
            if reading_values {
                let parts: Vec<&str> = line.split(":").collect();
                assert!(parts.len() == 2);
                let key = parts[0].to_string();
                let value = parts[1].trim().to_string() == "1";
                values.insert(key, value);
            } else {
                let parts: Vec<&str> = line.split("->").collect();
                assert!(parts.len() == 2);
                let key = parts[1].trim().to_string();
                if key.starts_with("z") {
                    num_z += 1;
                }
                let parts: Vec<&str> = parts[0].split_whitespace().collect();
                assert!(parts.len() == 3);
                let operand1 = parts[0].to_string();
                let operand2 = parts[2].to_string();
                let operation = match parts[1] {
                    "AND" => Operation::AND,
                    "OR" => Operation::OR,
                    "XOR" => Operation::XOR,
                    _ => panic!("Invalid operation"),
                };
                let gate = Gate {
                    operand1,
                    operand2,
                    operation,
                };
                gate_outputs.insert(gate.clone(), key.clone());
                connections.insert(key, gate);
            }
        }
    }
    if execute_part1 {
        part1(&connections, &mut values, num_z);
    } else {
        part2(&mut connections, &mut gate_outputs, num_z);
    }
}

fn part1(connections: &HashMap<String, Gate>, values: &mut HashMap<String, bool>, num_z: u32) {
    // proceed backwards (prevents having to calculate values that we don't need)
    let mut result: u64 = 0;
    let mut bits: String = String::new();

    for i in (0..num_z).rev() {
        let key = format!("z{:02}", i);
        let value = calculate_gate_output(key, connections, values);
        bits.push_str(if value { "1" } else { "0" });
        result <<= 1;
        result += value as u64;
    }
    println!("result: {}; {}", result, bits);
}

fn calculate_gate_output(
    key: String,
    connections: &HashMap<String, Gate>,
    values: &mut HashMap<String, bool>,
) -> bool {
    let connection = connections.get(&key).unwrap();
    let operand1 = match values.get(&connection.operand1) {
        Some(value) => *value,
        None => calculate_gate_output(connection.operand1.clone(), connections, values),
    };
    let operand2 = match values.get(&connection.operand2) {
        Some(value) => *value,
        None => calculate_gate_output(connection.operand2.clone(), connections, values),
    };
    let result = match connection.operation {
        Operation::AND => operand1 & operand2,
        Operation::OR => operand1 | operand2,
        Operation::XOR => operand1 ^ operand2,
    };
    values.insert(key, result);
    result
}

// For my input:
// swaps discovered by looking at the carry formula:
// BAD: gkc AND qqw -> z11; swap z11 with wpd
// BAD: x15 XOR y15 -> jqf; swap jqf with skh
// BAD: carry_37=z37 -> swap z37 with wts
// swap discovered in the z formula:
// swap mdd and z19
// Answer: jqf,mdd,skh,wpd,wts,z11,z19,z37
//
// A first look at the input data (and other people's solutions ;-)) showed that the circuit was
// supposed to be a full adder, using only and, or and xor gates. Meaning:
//     carry_n = (x_n AND y_n) OR (carry_{n-1} AND (x_n XOR y_n))
//     z_n = ((x_n XOR y_n) XOR carry_{n-1})
//     z_{N+1} = carry_N
// where:
//   - there are N+1 input bits and therefore N+2 output bits (including the carry bit)
//   - carry_n is the carry bit for the n-th bit
//   - z_n is the n-th bit of the sum
// This function was used to find mismatches. The swaps were then written down and applied to the input file.
// As others have stated, the swaps in the input were not complicated, making it easy to find them by hand.
// A more complicated swap would be e.g. to swap the a carry output wire with an (x_i AND y_i) output wire.
// I couldn't come up with a generic solution that doesn't take too long.
fn part2(
    connections: &mut HashMap<String, Gate>,
    gate_outputs: &mut HashMap<Gate, String>,
    num_z: u32,
) {
    let (carry_0_gate, carry_0_wire) =
        get_gate_and_wire(&"x00".to_string(), Operation::AND, gate_outputs).unwrap();
    println!("carry_00={}={}", carry_0_wire, print_gate(&carry_0_gate));
    let mut previous_carry_wire = carry_0_wire.clone();
    let mut carry_wires: Vec<String> = Vec::new();
    carry_wires.push(carry_0_wire);

    // first the carry wire calculation, as this is needed for the z wire calculation
    for i in 1..num_z - 1 {
        let x_key = format!("x{:02}", i);
        // assume port x_n XOR y_n has the right output wire
        let (x_or_gate, x_or_wire) =
            get_gate_and_wire(&x_key, Operation::XOR, gate_outputs).unwrap();
        // find an AND gate with this wire as input
        let right_part_gate_option = get_gate_and_wire(&x_or_wire, Operation::AND, gate_outputs);
        if right_part_gate_option.is_none() {
            println!(
                "{:02}: No AND gate for {}: that output must be the wrong name for the XOR gate",
                i, x_or_wire
            );
            return;
        }
        let (right_part_gate, right_part_wire) = right_part_gate_option.unwrap();
        let obtained_previous_carry_wire = if right_part_gate.operand1 == x_or_wire {
            right_part_gate.operand2.clone()
        } else {
            right_part_gate.operand1.clone()
        };
        // find an OR gate with this wire as input
        let carry_gate_option = get_gate_and_wire(&right_part_wire, Operation::OR, gate_outputs);
        if carry_gate_option.is_none() {
            println!(
                "{:02}: No OR gate for {}, which is input for {}",
                i,
                right_part_wire,
                print_gate(&right_part_gate)
            );
            return;
        }
        let (carry_gate, carry_wire) = carry_gate_option.unwrap();
        let bits_input_wire = if carry_gate.operand1 == right_part_wire {
            carry_gate.operand2.clone()
        } else {
            carry_gate.operand1.clone()
        };
        let bits_input_gate = connections.get(&bits_input_wire).unwrap();
        println!(
            "carry_{:02}={}=({}) OR ({} AND ({}))",
            i,
            carry_wire,
            print_gate(&bits_input_gate),
            obtained_previous_carry_wire,
            print_gate(&x_or_gate)
        );
        if previous_carry_wire != obtained_previous_carry_wire {
            println!(
                "{:02}: wrong previous carry wire: {} != {}",
                i, previous_carry_wire, obtained_previous_carry_wire
            );
            return;
        }
        previous_carry_wire = carry_wire.clone();
        carry_wires.push(carry_wire);
    }
    for i in 1..num_z - 1 {
        // an XOR gate with the carry bit will exist
        let (z_n_gate, z_n_wire) =
            get_gate_and_wire(&carry_wires[i as usize - 1], Operation::XOR, gate_outputs).unwrap();
        let z_key = format!("z{:02}", i);
        if z_n_wire != z_key {
            println!("{}: {} != {}", i, z_n_wire, z_key);
            return;
        }
        let other_input = if z_n_gate.operand1 == carry_wires[i as usize - 1] {
            z_n_gate.operand2.clone()
        } else {
            z_n_gate.operand1.clone()
        };
        // I didn't have to add a check to see if the other gate was correct (the output wire of x_i XOR yIi)
        let other_gate = connections.get(&other_input).unwrap();
        println!(
            "{}=({}) XOR {}",
            z_n_wire,
            print_gate(other_gate),
            carry_wires[i as usize - 1]
        );
    }
}

fn print_gate(gate: &Gate) -> String {
    format!("{} {} {}", gate.operand1, gate.operation, gate.operand2)
}

fn get_gate_and_wire(
    operand: &String,
    operation: Operation,
    gate_output_wires: &HashMap<Gate, String>,
) -> Option<(Gate, String)> {
    for (gate, wire) in gate_output_wires.iter() {
        if gate.operand1 != *operand && gate.operand2 != *operand {
            continue;
        }
        if gate.operation != operation {
            continue;
        }
        return Some((gate.clone(), wire.clone()));
    }
    None
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
