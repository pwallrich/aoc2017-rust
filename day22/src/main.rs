use std::{
    collections::{HashMap, HashSet},
    fs,
};

use util::Point;

fn main() {
    let input =
        fs::read_to_string("day22/input.txt").expect("Should have been able to read the file");

    // let input = "..#\n#..\n...";
    let rows: Vec<&str> = input.split("\n").collect();
    let dimensions = rows.len();
    assert!(dimensions % 2 == 1);
    let center = dimensions / 2;

    let mut grid: HashMap<Point, Node_state> = HashMap::new();
    let mut burst_count = 0;
    for (y, row) in rows.iter().rev().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '#' {
                continue;
            }
            grid.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                Node_state::Infected,
            );
        }
    }
    let mut curr = Point {
        x: center as i32,
        y: center as i32,
    };
    let mut facing = Point { x: 0, y: 1 };
    for i in 0..10_000_000 {
        if i % 100_000 == 0 {
            println!("iteration {i}");
        }
        let current_state = grid.get(&curr).unwrap_or(&Node_state::Clean);
        let next_state = current_state.next_state();

        match current_state {
            Node_state::Weakened => {}
            Node_state::Infected => facing = turn_right(facing),
            Node_state::Flagged => facing = reverse_direction(facing),
            Node_state::Clean => facing = turn_left(facing),
        }

        if matches!(next_state, Node_state::Clean) {
            grid.remove(&curr);
        } else if matches!(next_state, Node_state::Infected) {
            burst_count += 1;
            grid.insert(curr, next_state);
        } else {
            grid.insert(curr, next_state);
        }
        // println!("{burst_count}");
        curr = Point {
            x: curr.x + facing.x,
            y: curr.y + facing.y,
        };
    }
    println!("{burst_count}");
}

#[derive(Debug)]
enum Node_state {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Node_state {
    fn next_state(&self) -> Node_state {
        match self {
            Self::Clean => Self::Weakened,
            Self::Weakened => Self::Infected,
            Self::Infected => Self::Flagged,
            Self::Flagged => Self::Clean,
        }
    }
}
fn turn_right(point: Point) -> Point {
    match point {
        Point { x: 0, y: 1 } => Point { x: 1, y: 0 },
        Point { x: 0, y: -1 } => Point { x: -1, y: 0 },
        Point { x: 1, y: 0 } => Point { x: 0, y: -1 },
        Point { x: -1, y: 0 } => Point { x: 0, y: 1 },
        _ => panic!(),
    }
}

fn turn_left(point: Point) -> Point {
    match point {
        Point { x: 0, y: 1 } => Point { x: -1, y: 0 },
        Point { x: 0, y: -1 } => Point { x: 1, y: 0 },
        Point { x: 1, y: 0 } => Point { x: 0, y: 1 },
        Point { x: -1, y: 0 } => Point { x: 0, y: -1 },
        _ => panic!(),
    }
}

fn reverse_direction(point: Point) -> Point {
    Point {
        x: -1 * point.x,
        y: -1 * point.y,
    }
}
