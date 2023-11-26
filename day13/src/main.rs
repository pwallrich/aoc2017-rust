use std::{cmp::max, collections::HashMap, fs};

fn main() {
    let input =
        fs::read_to_string("day13/input.txt").expect("Should have been able to read the file");

    // let input = "0: 3\n1: 2\n4: 4\n6: 4";

    let mut scanner = HashMap::new();
    let mut max_count = 0;

    for row in input.lines() {
        let values: Vec<i32> = row.split(": ").map(|x| x.parse::<i32>().unwrap()).collect();
        scanner.insert(values[0], values[1]);
        max_count = max(values[0], max_count);
    }

    part1(max_count, &scanner);
    part2(max_count, &scanner)
}

fn part1(max_count: i32, scanner: &HashMap<i32, i32>) {
    let mut state = vec![0; (max_count + 1) as usize];
    let mut direction = vec![1; (max_count + 1) as usize];

    let mut res = 0;
    for i in 0..=max_count {
        println!("{:?}", state);
        if state[i as usize] == 0 && scanner.contains_key(&i) {
            res += scanner[&i] * i;
        }

        // create next state
        for scan in scanner {
            let mut next = state[*scan.0 as usize] + direction[*scan.0 as usize];
            if next + 1 >= *scan.1 || next == 0 {
                direction[*scan.0 as usize] *= -1;
            }
            state[*scan.0 as usize] = next;
        }
    }
    println!("{res}")
}

fn part2(max_count: i32, scanner: &HashMap<i32, i32>) {
    let mut state = vec![0; (max_count + 1) as usize];
    let mut direction = vec![1; (max_count + 1) as usize];

    let mut delay = 0;
    let mut iterations: Vec<(i32, i32)> = Vec::new();
    loop {
        println!("{:?}", delay);

        iterations.push((0, delay));
        let mut inidices_to_remove = Vec::new();
        for (idx, iteration) in iterations.clone().iter().enumerate() {
            if state[(iteration.0) as usize] == 0 && scanner.contains_key(&iteration.0) {
                inidices_to_remove.push(idx);
            } else {
                iterations[idx].0 += 1;
            }
            if iterations[idx].0 > max_count {
                println!("{:?}", iterations[idx]);
                return;
            }
        }

        iterations = iterations
            .iter()
            .enumerate()
            .filter_map(|x| {
                if inidices_to_remove.contains(&x.0) {
                    None
                } else {
                    Some(*x.1)
                }
            })
            .collect();

        // create next state
        for scan in scanner {
            let mut next = state[*scan.0 as usize] + direction[*scan.0 as usize];
            if next + 1 >= *scan.1 || next == 0 {
                direction[*scan.0 as usize] *= -1;
            }
            state[*scan.0 as usize] = next;
        }
        delay += 1;
    }
}
