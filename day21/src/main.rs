use std::{collections::HashSet, fs};

use util::Point;

fn main() {
    // let start = ".#.\n..#\n###";

    let mut grid: HashSet<Point> = HashSet::from([
        Point { x: 1, y: 0 },
        Point { x: 2, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 2, y: 2 },
    ]);

    let input =
        fs::read_to_string("day21/input.txt").expect("Should have been able to read the file");

    // let input = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";

    let rules = preprocess_rules(&input);

    let mut size = 3;
    for i in 0..18 {
        println!("iteration: {i}");
        let new_grid = next_step(size, &grid, &rules);
        size = if size % 2 == 0 {
            3 * size / 2
        } else {
            4 * size / 3
        };
        grid = new_grid;
    }
    println!("{}", grid.len());
}

fn next_step(
    size: i32,
    grid: &HashSet<Point>,
    rules: &Vec<(Vec<bool>, Vec<bool>)>,
) -> HashSet<Point> {
    let mut new_grid = HashSet::new();
    let width = if size % 2 == 0 {
        2
    } else if size % 3 == 0 {
        3
    } else {
        panic!("I don't know what to do now");
    };
    let points = if width == 2 {
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ]
    } else {
        // break up in 3 by 3 grids
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ]
    };

    for (y_offset, y) in (0..size).step_by(width).enumerate() {
        for (x_offset, x) in (0..size).step_by(width).enumerate() {
            let mut small_grid: Vec<bool> = Vec::new();
            for point in &points {
                let curr = Point {
                    x: point.x + x,
                    y: point.y + y,
                };
                small_grid.push(grid.contains(&curr));
            }
            // find pattern
            let mut next: Option<Vec<bool>> = None;
            for (i, rule) in rules.iter().enumerate() {
                if rule.0.len() != small_grid.len() {
                    continue;
                }
                if rule.0 == small_grid {
                    next = Some(rule.1.clone());
                    // println!("found {i}");
                    break;
                }
            }
            if next == None {
                panic!("no match found");
            }

            let new_width = if width == 2 { 3 } else { 4 };
            let next = next.unwrap();
            // println!("small_grid: {:?}", small_grid);
            // println!("next: {:?}", next);
            for relative_y in 0..new_width {
                for relative_x in 0..new_width {
                    if next[relative_y * new_width + relative_x] {
                        let point = Point {
                            x: x + (x_offset as i32) + relative_x as i32,
                            y: y + (y_offset as i32) + relative_y as i32,
                        };
                        // println!("point: {:?}", point);
                        new_grid.insert(point);
                    }
                }
            }
        }
    }
    return new_grid;
}

fn preprocess_rules(input: &str) -> Vec<(Vec<bool>, Vec<bool>)> {
    let mut rules: Vec<(Vec<bool>, Vec<bool>)> = Vec::new();
    for row in input.split("\n") {
        let in_out: Vec<&str> = row.split(" => ").collect();
        let from = in_out[0];
        let to = in_out[1];
        let mut from_set = Vec::new();
        let mut temp_rules = Vec::new();

        for (i, c) in from.chars().enumerate() {
            if c == '/' {
                continue;
            }
            from_set.push(c == '#');
        }
        let mut to_set = Vec::new();
        for (i, c) in to.chars().enumerate() {
            if c == '/' {
                continue;
            }
            to_set.push(c == '#');
        }
        temp_rules.push((from_set.clone(), to_set.clone()));

        let width = if from_set.len() == 9 { 3 } else { 2 };

        // turned 90
        let mut turned_90 = Vec::new();

        for x in 0..width {
            for y in (0..width).rev() {
                turned_90.push(from_set[y * width + x]);
            }
        }
        temp_rules.push((turned_90, to_set.clone()));

        // turned 180
        let mut turned_180 = Vec::new();
        for y in (0..width).rev() {
            for x in (0..width).rev() {
                turned_180.push(from_set[y * width + x]);
            }
        }
        temp_rules.push((turned_180, to_set.clone()));

        // turned 270
        let mut turned_270 = Vec::new();
        for x in (0..width).rev() {
            for y in (0..width) {
                turned_270.push(from_set[y * width + x]);
            }
        }
        temp_rules.push((turned_270, to_set.clone()));
        for (from, to) in temp_rules {
            rules.push((from.clone(), to.clone()));
            let mut mirrored = Vec::new();
            // mirrored
            for y in 0..width {
                for x in (0..width).rev() {
                    mirrored.push(from[y * width + x]);
                }
            }
            rules.push((mirrored, to.clone()));
        }
    }
    return rules;
}

// ..
// .#

// ..
// #.

// ..
// #.

// #.
// ..

// .#
// ..
