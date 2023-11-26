use std::{collections::HashMap, fs};

fn main() {
    let input =
        fs::read_to_string("day24/input.txt").expect("Should have been able to read the file");

    // let input = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

    let mut pieces: Vec<(i32, i32)> = Vec::new();
    for row in input.split("\n") {
        let split: Vec<&str> = row.split("/").collect();
        let first = split[0].parse::<i32>().unwrap();
        let second = split[1].parse::<i32>().unwrap();
        pieces.push((first, second));
    }

    let start_points = pieces.iter().filter(|x| x.0 == 0 || x.1 == 0);

    let mut max_value = 0;
    let mut max_length: usize = 0;
    for start_point in start_points {
        let open_end = if start_point.0 == 0 {
            start_point.1
        } else {
            start_point.0
        };
        let current = build_bridge_longest(vec![start_point.clone()], open_end, &pieces, 0);
        if current.1 > max_length {
            max_length = current.1;
            max_value = current.0;
        } else if current.1 == max_length && current.0 > max_value {
            max_value = current.0;
        }
    }
    println!("{max_value}");
}

fn build_bridge(current: Vec<(i32, i32)>, open_end: i32, pieces: &Vec<(i32, i32)>) -> i32 {
    println!("{:?}", current);
    if current.len() == pieces.len() {
        return current.iter().map(|x| x.0 + x.1).sum();
    }
    let mut max_value = 0;
    let matching: Vec<&(i32, i32)> = pieces
        .iter()
        .filter(|p| {
            if p.0 != open_end && p.1 != open_end {
                return false;
            }
            if current.contains(p) {
                return false;
            }
            return true;
        })
        .collect();
    if matching.len() == 0 {
        return current.iter().map(|x| x.0 + x.1).sum();
    }
    for piece in matching {
        let mut current = current.clone();
        current.push(piece.clone());
        let open_end = if piece.0 == open_end {
            piece.1
        } else {
            piece.0
        };
        let current_max = build_bridge(current, open_end, pieces);
        if current_max > max_value {
            max_value = current_max;
        }
    }
    return max_value;
}

fn build_bridge_longest(
    current: Vec<(i32, i32)>,
    open_end: i32,
    pieces: &Vec<(i32, i32)>,
    count: i32,
) -> (i32, usize) {
    println!("{:?}, {:?}", count, current.len());
    if current.len() == pieces.len() {
        return (current.iter().map(|x| x.0 + x.1).sum(), current.len());
    }
    let mut max_value = 0;
    let mut max_length = 0;
    let matching: Vec<&(i32, i32)> = pieces
        .iter()
        .filter(|p| {
            if p.0 != open_end && p.1 != open_end {
                return false;
            }
            if current.contains(p) {
                return false;
            }
            return true;
        })
        .collect();
    if matching.len() == 0 {
        return (current.iter().map(|x| x.0 + x.1).sum(), current.len());
    }
    for piece in matching {
        let mut current = current.clone();
        current.push(piece.clone());
        let open_end = if piece.0 == open_end {
            piece.1
        } else {
            piece.0
        };
        let current = build_bridge_longest(current, open_end, pieces, count + 1);
        if current.1 > max_length {
            max_length = current.1;
            max_value = current.0;
        } else if current.1 == max_length && current.0 > max_value {
            max_value = current.0;
        }
    }
    return (max_value, max_length);
}
