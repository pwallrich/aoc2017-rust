use itertools::{self, Itertools};
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    // let input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";
    // let input = "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let rows: Vec<Vec<&str>> = input
        .split("\n")
        .map(|row| row.split_whitespace().collect())
        .collect();

    let mut count = 0;
    for row in rows {
        let mut set = HashSet::new();
        let mut valid = true;
        for word in row {
            if set.contains(word) {
                valid = false;
                break;
            }
            set.insert(word);
        }
        if valid {
            count += 1;
        }
    }
    println!("{count}");
}

fn part2(input: &str) {
    let rows: Vec<Vec<&str>> = input
        .split("\n")
        .map(|row| row.split_whitespace().collect())
        .collect();

    let mut count = 0;
    for row in rows {
        let mut set = HashSet::new();
        let mut valid = true;

        'outer: for word in row {
            let mut temp = HashSet::new();
            for perm in word.chars().permutations(word.len()) {
                if set.contains(&perm) {
                    valid = false;
                    break 'outer;
                }
                temp.insert(perm);
            }
            set.extend(temp);
        }
        if valid {
            count += 1;
        }
    }
    println!("{count}");
}
