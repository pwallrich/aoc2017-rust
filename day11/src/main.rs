use std::{cmp::min, collections::HashSet, fmt::DebugTuple, fs};

use util::Point;

fn main() {
    let input =
        fs::read_to_string("day11/input.txt").expect("Should have been able to read the file");

    // let input = "se,sw,se,sw,sw";

    let (grid, last_point) = generateGrid(&input.split(",").collect());
    let distance = calculate_distance(Point { x: 0, y: 0 }, last_point);
    let max = grid
        .iter()
        .map(|x| calculate_distance(Point { x: 0, y: 0 }, *x))
        .max()
        .unwrap();
    println!("{max}");
    // println!("{:?}", grid);
    // println!("{:?}", distance)
}

fn generateGrid(input: &Vec<&str>) -> (HashSet<Point>, Point) {
    let mut curr = Point { x: 0, y: 0 };
    let mut grid = HashSet::from([curr]);
    for row in input {
        let offset = match *row {
            "n" => Point { x: 1, y: 1 },
            // "e" => Point { x: 1, y: 0 },
            "s" => Point { x: -1, y: -1 },
            // "w" => Point { x: -1, y: 0 },
            "ne" => Point { x: 1, y: 0 },
            "se" => Point { x: 0, y: -1 },
            "sw" => Point { x: -1, y: -0 },
            "nw" => Point { x: 0, y: 1 },
            _ => panic!(),
        };
        curr.x += offset.x;
        curr.y += offset.y;
        println!("{:?}", curr);
        grid.insert(curr);
    }
    return (grid, curr);
}

fn calculate_distance(a: Point, b: Point) -> i32 {
    // compute distance as we would on a normal grid
    let distance = Point {
        x: a.x - b.x,
        y: a.y - b.y,
    };

    // compensate for grid deformation
    // grid is stretched along (-n, n) line so points along that line have
    // a distance of 2 between them instead of 1

    // to calculate the shortest path, we decompose it into one diagonal movement(shortcut)
    // and one straight movement along an axis
    let mut diagonalMovement: Point = Point { x: 0, y: 0 };
    let lesserCoord = min(distance.x.abs(), distance.y.abs());
    diagonalMovement.x = if distance.x < 0 {
        -lesserCoord
    } else {
        lesserCoord
    }; // keep the sign
    diagonalMovement.y = if distance.y < 0 {
        -lesserCoord
    } else {
        lesserCoord
    }; // keep the sign

    // one of x or y should always be 0 because we are calculating a straight
    // line along one of the axis
    let straightMovement = Point {
        x: distance.x - diagonalMovement.x,
        y: distance.y - diagonalMovement.y,
    };

    // calculate distance
    let straightDistance = straightMovement.x.abs() + straightMovement.y.abs();
    let mut diagonalDistance = diagonalMovement.x.abs();

    // if we are traveling diagonally along the stretch deformation we double
    // the diagonal distance
    if ((diagonalMovement.x < 0 && diagonalMovement.y > 0)
        || (diagonalMovement.x > 0 && diagonalMovement.y < 0))
    {
        diagonalDistance *= 2;
    }

    return straightDistance + diagonalDistance;
}
