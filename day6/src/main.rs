use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // let input = "0  2  7  0";

    let rows: Vec<i32> = input
        .split_whitespace()
        .map(|row| row.parse::<i32>().unwrap())
        .collect();

    run(rows);
}

fn run(input: Vec<i32>) {
    let mut values = input.clone();
    let mut text = values
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
        .join("");

    let mut seen = HashMap::new();
    let mut round = 0;
    while !seen.contains_key(&text) {
        round += 1;
        println!("{}", seen.len());
        seen.insert(text, round);
        let mut max = 0;
        let mut idx = 0;
        for (curr, val) in values.iter().enumerate() {
            if *val > max {
                max = *val;
                idx = curr;
            }
        }
        values[idx] = 0;
        idx = (idx + 1) % values.len();

        for _ in 0..max {
            values[idx] += 1;
            idx = (idx + 1) % values.len();
        }
        text = values
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join("");
    }
    let diff = round - seen[&text] + 1;

    println!("part1: {}, part 2: {}", seen.len(), diff);
}
