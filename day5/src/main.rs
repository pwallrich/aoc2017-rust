use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // let input = "0\n3\n0\n1\n-3";

    let rows: Vec<i32> = input
        .split("\n")
        .map(|row| row.parse::<i32>().unwrap())
        .collect();

    part1(rows.clone());
    part2(rows.clone());
}

fn part1(input: Vec<i32>) {
    let mut values = input;
    let mut idx: i32 = 0;
    let mut count = 0;
    while idx >= 0 && idx < values.len() as i32 {
        count += 1;
        let val = values[idx as usize];
        values[idx as usize] += 1;
        idx += val;
    }

    println!("{count}")
}

fn part2(input: Vec<i32>) {
    let mut values = input;
    let mut idx: i32 = 0;
    let mut count = 0;
    while idx >= 0 && idx < values.len() as i32 {
        count += 1;
        let val = values[idx as usize];
        if val >= 3 {
            values[idx as usize] -= 1;
        } else {
            values[idx as usize] += 1;
        }
        idx += val;
    }

    println!("{count}")
}
