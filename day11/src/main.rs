use std::collections::HashMap;

// To experiment a bit with Rust and the borrow checker, I've optimized the code
// to reuse buffers. It's less straight-forward than how I would do it in e.g. C.
// The impact on performance for the 75-level depth is minimal. The other 2
// optimizations discussed below were crucial.
fn main() {
    let stones: Vec<i64> = vec![3, 386358, 86195, 85, 1267, 3752457, 0, 741];
    let mut calculated_lengths: HashMap<(i64, usize), usize> = HashMap::new();
    // reused buffers to avoid reallocation
    let mut stones_buffera: Vec<i64> = Vec::new();
    let mut stones_bufferb: Vec<i64> = Vec::new();
    println!(
        "Number of stones: {}",
        depth_first(
            &stones,
            0,
            &mut calculated_lengths,
            &mut stones_buffera,
            &mut stones_bufferb
        )
    );
}

// The trick to make this faster is to store the calculated lengths in a hashmap.
// The trick to not run out of memory is to use a depth first algorithm. A depth
// step size of 25 was proven to be okay by part 1 of the problem.
fn depth_first(
    stones: &Vec<i64>,
    depth: usize,
    calculated_lengths: &mut HashMap<(i64, usize), usize>,
    stones_buffera: &mut Vec<i64>,
    stones_bufferb: &mut Vec<i64>,
) -> usize {
    let depth_step_size = 25usize;
    let mut num = 0usize;
    // a buffer that can be reused in this recursion step
    let mut stones_bufferc: Vec<i64> = Vec::new();
    for k in 0..stones.len() {
        if calculated_lengths.contains_key(&(stones[k], depth)) {
            num += calculated_lengths.get(&(stones[k], depth)).unwrap();
            continue;
        }
        stones_buffera.clear();
        stones_buffera.push(stones[k]);
        let full_depth = depth + depth_step_size == 75;
        generate_stones(stones_buffera, depth_step_size, stones_bufferb);
        let mut extra_len = stones_bufferb.len();
        if !full_depth {
            stones_bufferc.clear();
            stones_bufferc.extend(stones_bufferb.iter());
            extra_len = depth_first(
                &stones_bufferc,
                depth + depth_step_size,
                calculated_lengths,
                stones_buffera,
                stones_bufferb,
            );
        }
        num += extra_len;
        calculated_lengths.insert((stones[k], depth), extra_len);
    }
    num
}

// the generated stones will be stored in stones_bufferb
fn generate_stones(
    mut stones_buffera: &mut Vec<i64>,
    depth_step_size: usize,
    mut stones_bufferb: &mut Vec<i64>,
) {
    let mut new_stones: &mut Vec<i64>;
    let mut stones: &Vec<i64>;
    for i in 0..depth_step_size {
        if i % 2 == 0 {
            new_stones = &mut stones_bufferb;
            stones = &stones_buffera;
        } else {
            new_stones = &mut stones_buffera;
            stones = &stones_bufferb;
        }
        /* This was my original code: it does the same thing as above, but the
         * borrow checker isn't smart enough to know it.
         * I wasted quite some time on this; copilot made me see the light.
        new_stones = match i % 2 == 0 {
            true => &mut stonesb,
            false => &mut stonesa,
        };
        stones = match i % 2 == 0 {
            true => &stonesa,
            false => &stonesb,
        };*/
        /* This was a later try at more succint code, but again the borrow checker wasn't smart enough.
        (new_stones, stones) = match i % 2 == 0 {
            true => (&mut stones_bufferb, &stones_buffera),
            false => (&mut stones_buffera, &stones_bufferb),
        };*/
        new_stones.clear();
        for j in 0..stones.len() {
            let stone_number = stones[j];
            if stone_number == 0 {
                new_stones.push(1);
            } else {
                let num = num_digits(stone_number);
                if num % 2 == 0 {
                    let denominator = 10i64.pow(num as u32 / 2);
                    let first_number = stone_number / denominator;
                    let second_number = stone_number % denominator;
                    new_stones.push(first_number);
                    new_stones.push(second_number);
                } else {
                    new_stones.push(stone_number * 2024);
                }
            }
        }
    }
}

fn num_digits(n: i64) -> usize {
    let mut num = n / 10;
    let mut num_digits: usize = 1;
    while num > 0 {
        num /= 10;
        num_digits += 1;
    }
    num_digits
}
