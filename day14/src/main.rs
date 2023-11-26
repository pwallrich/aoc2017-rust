use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    num::ParseIntError,
};

use util::{get_adjacent_points_without_diagonals, knot_hash, Point};

fn main() {
    let input = "hfdlxzhv";

    // let input = "flqrgnkx";

    let hashes = (0..128).map(|x| {
        let input = format!("{input}-{x}");
        knot_hash(&input)
    });
    let mut grid: HashSet<Point> = HashSet::new();
    let mut res = 0;
    for (y, hash) in hashes.enumerate() {
        let decoded = decode_hex(&hash).unwrap();
        let mut row = Vec::new();
        let mut x = 0;
        // println!("{:?}", decode_hex(&hash));
        for val in decoded {
            let bin = to_binary(val);
            for b in bin {
                let point = Point { x: x, y: y as i32 };
                if b {
                    grid.insert(point);
                    res += 1;
                }
                row.push(b);
                x += 1;
            }
        }
    }

    // pretty_print(grid, 8, 12);
    println!("{res}");
    let mut group: Vec<Vec<Point>> = Vec::new();
    let mut seen: HashSet<Point> = HashSet::new();
    for item in &grid {
        let mut idx = 0;
        let mut curr: Vec<Point> = Vec::from([*item]);
        if seen.contains(item) {
            continue;
        }
        while idx < curr.len() {
            let next = curr[idx];
            idx += 1;
            // println!("checking {:?}", next);
            if seen.contains(&next) {
                continue;
            }
            seen.insert(next);
            for adj in get_adjacent_points_without_diagonals(&next) {
                if !grid.contains(&adj) {
                    continue;
                }
                if seen.contains(&adj) {
                    continue;
                }
                if curr.contains(&adj) {
                    continue;
                }
                curr.push(adj);
            }
        }
        group.push(curr);
    }

    println!("{}", group.len());
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn to_binary(value: u8) -> Vec<bool> {
    let mut mask = 128u8; // assuming rightmost bit first
    return (0..8)
        .map(|i| {
            let is_set = value & mask != 0;
            mask >>= 1; // assuming rightmost bit
            return is_set;
        })
        .collect();
}

fn pretty_print(grid: HashSet<Point>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let point = Point { x: x, y: y };
            print!("{}", if grid.contains(&point) { "#" } else { "." })
        }
        println!()
    }
    println!()
}
