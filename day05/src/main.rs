use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "./src/input_example.txt";
    //let path = "./src/input.txt";
    let mut orderings: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut processing_orderings = true;
    let mut updates = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                if processing_orderings {
                    if l.is_empty() {
                        processing_orderings = false;
                        continue;
                    }
                    let pages: Vec<i64> = l.split("|").map(|s| s.parse::<i64>().unwrap()).collect();
                    assert_eq!(2, pages.len(), "2 entries required!");
                    orderings
                        .entry(pages[0])
                        .or_insert_with(Vec::new)
                        .push(pages[1]);
                } else {
                    let pages: Vec<i64> = l.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
                    updates.push(pages);
                }
            }
        }
    }
    let execute_part1 = false;
    if execute_part1 {
        part1(&orderings, &mut updates);
    } else {
        part2(&orderings, &mut updates);
    }
}

fn part1(orderings: &HashMap<i64, Vec<i64>>, updates: &mut Vec<Vec<i64>>) {
    let mut sum = 0i64;
    for update in updates {
        let mut update_elems: HashMap<i64, usize> = HashMap::new();
        let mut in_order = true;
        for i in 0..update.len() {
            let page = update[i];
            if orderings.contains_key(&page) {
                let next_pages = orderings.get(&page).unwrap();
                for next_page in next_pages {
                    if update_elems.contains_key(next_page) {
                        in_order = false;
                        break;
                    }
                }
            }
            update_elems.insert(page, i);
        }
        if in_order {
            sum += update[update.len() / 2];
        }
    }
    println!("Sum of middle: {:?}", sum);
}

fn part2(orderings: &HashMap<i64, Vec<i64>>, updates: &Vec<Vec<i64>>) {
    let mut sum = 0i64;
    for update in updates {
        // first part is the same as part1()
        let mut update_elems: HashMap<i64, usize> = HashMap::new();
        let mut in_order = true;
        for i in 0..update.len() {
            let page = update[i];
            if orderings.contains_key(&page) {
                let next_pages = orderings.get(&page).unwrap();
                for next_page in next_pages {
                    if update_elems.contains_key(next_page) {
                        in_order = false;
                        break;
                    }
                }
            }
            update_elems.insert(page, i);
        }
        if !in_order {
            // would be cleaner in a separate function, I know. But it's getting late.
            // fixed_update contains the reoredered update so that the ordering rules are satisfied.
            let mut fixed_update: Vec<i64> = vec![-1; update.len()];
            for i in 0..update.len() {
                let page = update[i];
                if orderings.contains_key(&page) {
                    let next_pages = orderings.get(&page).unwrap();
                    let mut min_index = usize::MAX;
                    // see if there's a page already present for which there's an ordering rule
                    // that the current page should be before it. Find the first such page.
                    for next_page in next_pages {
                        // possible optimization: replace linear search
                        if let Some(pos) = fixed_update.iter().position(|&x| x == *next_page) {
                            if pos < min_index {
                                min_index = pos;
                            }
                        }
                    }
                    if min_index != usize::MAX {
                        // insert the current page right before the found page.
                        for j in (min_index + 1..fixed_update.len()).rev() {
                            fixed_update[j] = fixed_update[j - 1];
                        }
                        fixed_update[min_index] = page;
                    } else {
                        // no ordering rule for this page at this stage
                        fixed_update[i] = page;
                    }
                } else {
                    // no ordering rule for this page
                    fixed_update[i] = page;
                }
            }
            //println!("Fixed update: {:?}", fixed_update);
            sum += fixed_update[fixed_update.len() / 2];
        }
    }
    println!("Sum of middle: {:?}", sum);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}
