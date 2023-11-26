use itertools::{self, Itertools};
use std::cmp;
use std::fs;

fn main() {
    // let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let vals: Vec<Vec<i32>> = input
        .split_terminator("\n")
        .map(|row| {
            row.split_whitespace()
                .map(|item| item.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    part1(&vals);
    part2(&vals);
}

fn part1(input: &Vec<Vec<i32>>) {
    let mut res = 0;
    for row in input {
        let max = row.iter().max().unwrap();
        let min = row.iter().min().unwrap();
        res += max - min;
    }
    println!("{}", res)
}

fn part2(input: &Vec<Vec<i32>>) {
    let mut res = 0;
    for row in input {
        for combination in row.into_iter().combinations(2) {
            let first = cmp::max(combination[0], combination[1]);
            let second = cmp::min(combination[0], combination[1]);
            if first % second == 0 {
                res += first / second;
                break;
            }
        }
    }
    println!("{}", res)
}
