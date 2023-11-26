mod knot_hash;

pub fn knot_hash(input: &str) -> String {
    return knot_hash::_knot_hash(input);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn get_adjacent_points_without_diagonals(point: &Point) -> [Point; 4] {
    return [
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: point.x,
            y: point.y + 1,
        },
    ];
}
