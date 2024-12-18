fn main() {
    let run_part1 = false;
    if run_part1 {
        part1();
    } else {
        part2();
    }
}

fn part2() {
    let program: Vec<i32> = vec![2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0];
    // Puzzle input:
    // Every loop, B is printed
    // B= A%8      (2,4)
    // B ^= 5      (1,5)
    // C = A>>B    (7,5)
    // B ^= C      (4,5)
    // A >>= 3     (0,3)
    // B ^= 6      (1,6)
    // PRINT B % 8 (5,5)
    // LOOP        (3,0)
    // Human-readable:
    // B1 = (A%8)^5
    // C = A >> B1
    // B = (B1^C)^6
    // A >>= 3
    // LOOP
    // The initial value of B and C doesn't matter.

    // calculate backwards
    // program length: 16, A is shifted 3 bits to the right each time => 16*3 = 48 bits;
    // One of bits 48,47,46 must be non-zero and bits >48 must be zero, otherwise the output length would be wrong.
    // The last a should be smaller than 2^3 and greater than 0.
    for last_a in 1..8 as i64 {
        let b1 = (last_a % 8) ^ 5;
        let c = last_a >> b1;
        let b = (b1 ^ c) ^ 6;
        if b % 8 == program[program.len() - 1] as i64 {
            recurse(&program, last_a << 3, program.len() - 2);
        }
    }
}

fn recurse(program: &Vec<i32>, a_base: i64, index: usize) {
    for add in 0..8 {
        let a = a_base + add;
        let b1 = (a % 8) ^ 5;
        let c = a >> b1;
        let b = (b1 ^ c) ^ 6;
        if b % 8 == program[index] as i64 {
            if index == 0 {
                println!("{}", a);
                std::process::exit(0);
            } else {
                recurse(program, a << 3, index - 1);
            }
        }
    }
}

fn part1() {
    let use_example_input = false;
    let (register_a, register_b, register_c) = match use_example_input {
        true => (729, 0, 0),
        false => (63281501, 0, 0),
    };
    let program = match use_example_input {
        true => vec![0, 1, 5, 4, 3, 0],
        false => vec![2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0],
    };
    println!(
        "{}",
        calculate_output(&program, register_a, register_b, register_c)
    );
}
fn calculate_output(
    program: &Vec<i32>,
    mut register_a: i64,
    mut register_b: i64,
    mut register_c: i64,
) -> String {
    let mut program_counter = 0usize;
    let mut output = String::new();
    while program_counter < program.len() {
        match program[program_counter] {
            0 => {
                // adv
                register_a >>= combo_value(
                    program[program_counter + 1],
                    register_a,
                    register_b,
                    register_c,
                );
            }
            1 => {
                // bxl
                register_b ^= program[program_counter + 1] as i64;
            }
            2 => {
                // bst
                register_b = combo_value(
                    program[program_counter + 1],
                    register_a,
                    register_b,
                    register_c,
                ) % 8;
                if register_b < 0 {
                    register_b += 8;
                }
            }
            3 => {
                // jnz
                if register_a != 0 {
                    program_counter = program[program_counter + 1] as usize;
                    continue;
                }
            }
            4 => {
                // bxc
                register_b ^= register_c;
            }
            5 => {
                // out
                let mut out = combo_value(
                    program[program_counter + 1],
                    register_a,
                    register_b,
                    register_c,
                ) % 8;
                if out < 0 {
                    out += 8;
                }
                output.push_str(&out.to_string());
                output.push_str(",");
            }
            6 => {
                // bdv
                register_b = register_a
                    >> combo_value(
                        program[program_counter + 1],
                        register_a,
                        register_b,
                        register_c,
                    );
            }
            7 => {
                // cdv
                register_c = register_a
                    >> combo_value(
                        program[program_counter + 1],
                        register_a,
                        register_b,
                        register_c,
                    );
            }
            _ => panic!("Unknown opcode"),
        }
        program_counter += 2;
    }
    output[..output.len() - 1].to_string()
}

fn combo_value(value: i32, register_a: i64, register_b: i64, register_c: i64) -> i64 {
    match value {
        0 | 1 | 2 | 3 => value as i64,
        4 => register_a,
        5 => register_b,
        6 => register_c,
        _ => panic!("bad operand"),
    }
}
