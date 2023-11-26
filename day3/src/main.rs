use core::panic;
use std::{collections::HashMap, default};

use util::Point;

fn main() {
    let input = 265149;
    // let input = 1024;
    part1(input);
    part2(input);
}

struct Spiral {
    layer: i32,
    leg: i32,
    point: Point,
}

impl Spiral {
    pub fn new() -> Self {
        Self {
            layer: 1,
            leg: 0,
            point: Point { x: 0, y: 0 },
        }
    }
}

impl Iterator for Spiral {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.leg {
            0 => {
                self.point.x += 1;
                if self.point.x == self.layer {
                    self.leg += 1;
                }
            }
            1 => {
                self.point.y += 1;
                if self.point.y == self.layer {
                    self.leg += 1;
                }
            }
            2 => {
                self.point.x -= 1;
                if -self.point.x == self.layer {
                    self.leg += 1;
                }
            }
            3 => {
                self.point.y -= 1;
                if -self.point.y == self.layer {
                    self.leg = 0;
                    self.layer += 1;
                }
            }
            _ => {
                panic!("Unexpected leg");
            }
        }
        return Some(self.point);
    }
}

fn part1(target: i32) {
    let mut iterator = Spiral::new();
    let mut next: Point = Point { x: 0, y: 0 };

    for i in 2..=target {
        next = iterator.next().unwrap();
    }

    println!("{:?}", next);
    println!("{}", next.x.abs() + next.y.abs());

    // println!("{:?}", grid);
    // for y in -2..=2 {
    //     for x in -2..=2 {
    //         let point = grid.get(&Point { x: x, y: y }).unwrap();
    //         print!("{point}");
    //     }
    //     println!()
    // }
    // let mut point = Point { x: 0, y: 0 };
    // let mut grid = HashMap::new();

    // let mut curr = 1;
    // let mut count = 0;
    // while curr * curr < target {
    //     println!("{}", curr);
    //     curr += 2;
    //     count += 1;
    // }

    // for i in 2..=count {
    //     let root = 2 * i - 1;
    //     for i in 0..(root * root) - (root - 2) * (root - 2) {
    //         let position =
    //     }
    // }

    // let mut res = curr / 2;
    // let dist = curr / 2;
    // if target >= curr * (curr - 1) + 1 {
    //     // bottom row
    //     let edge = (curr * (curr - 1)) + 1;
    //     res += (target - (edge + dist)).abs();
    // } else if target >= curr * (curr - 2) + 2 {
    //     // left row
    //     let edge = (curr * (curr - 2)) + 2;
    //     res += (target - (edge + dist)).abs();
    // } else if target >= curr * (curr - 3) + 3 {
    //     // top row
    //     let edge = (curr * (curr - 3)) + 3;
    //     res += (target - (edge + dist)).abs();
    // } else {
    //     // right row
    //     let edge = ((curr - 2) * (curr - 2));
    //     res += (target - (edge + dist)).abs();
    // }
    // bottom right = curr * curr
    // bottom left = curr * (curr - 1) + 1
    // top left = curr * (curr - 2) + 2
    // top right = curr * (curr - 3) + 3

    // downwards = (curr * curr) - (curr / 2)
    // upwards = curr * (curr - 3) + 3 + (curr / 2)

    //  37  36  35  34  33  32  31
    //  38  17  16  15  14  13  30
    //  39  18   5   4   3  12  29
    //  40  19   6   1   2  11  28
    //  41  20   7   8   9  10  27
    //  42  21  22  23  24  25  26
    //  43  44  45  46  47  48  49
    // println!("{}", res);
}

fn part2(target: i32) {
    let mut grid = HashMap::new();

    let mut iterator = Spiral::new();
    let mut next: Point = Point { x: 0, y: 0 };

    grid.insert(next, 1);
    for i in 2..=target {
        next = iterator.next().unwrap();
        let mut sum = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                let next_point = Point {
                    x: next.x + x,
                    y: next.y + y,
                };
                match grid.get(&next_point) {
                    Some(t) => sum += t,
                    _ => (),
                }
            }
        }
        grid.insert(next, sum);
        if sum > target {
            println!("{sum}");
            break;
        }
    }

    // println!("{:?}", grid);
    for y in -2..=2 {
        for x in -2..=2 {
            let point = grid.get(&Point { x: x, y: y }).unwrap_or_else(|| &-1);
            print!(" {point} ");
        }
        println!()
    }

    // println!("{:?}", next);
    // println!("{}", next.x.abs() + next.y.abs());
}
