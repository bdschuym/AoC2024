pub fn part1(keys: &Vec<[u8; 5]>, locks: &Vec<[u8; 5]>) -> u32 {
    let mut num_pairs = 0;
    for key in keys {
        for lock in locks {
            let mut valid = true;
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    valid = false;
                    break;
                }
            }
            if valid {
                num_pairs += 1;
            }
        }
    }
    num_pairs
}

// Functional version of part1. Run cargo bench to see the performance difference
// between both versions. The functional version is slower, probably because of
// the extra iterators involved (+-9ns for part1, +-13ns for part1_functional).
#[allow(dead_code)]
pub fn part1_functional(keys: &Vec<[u8; 5]>, locks: &Vec<[u8; 5]>) -> u32 {
    keys.iter()
        .flat_map(|key| {
            locks
                .iter()
                .filter(|lock| key.iter().zip(lock.iter()).all(|(&k, &l)| k + l <= 5))
        })
        .count() as u32
}
