use std::fs;

use util::{get_adjacent_points_without_diagonals, Point};

fn main() {
    let input =
        fs::read_to_string("day19/input.txt").expect("Should have been able to read the file");

    // let input = "     |          \n     |  +--+   \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ ";
    let rows: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();
    let x = rows
        .iter()
        .nth(0)
        .unwrap()
        .iter()
        .position(|x| x == &'|')
        .unwrap();
    let mut current = Point { x: x as i32, y: 0 };
    // 0 -> down
    let mut dir = Point { x: 0, y: 1 };
    let mut result: String = "".to_string();
    let mut count = 0;
    loop {
        let next = Point {
            x: current.x + dir.x,
            y: current.y + dir.y,
        };
        let next_char = rows[next.y as usize][next.x as usize];
        println!("next iter: {:?}, {next_char}, {:?}", next, dir);
        count += 1;
        if next_char == '+' {
            //change dir
            let mut found = false;
            for adj in get_adjacent_points_without_diagonals(&next) {
                if adj == current {
                    continue;
                }
                if adj.y as usize >= rows.len() || adj.x as usize >= rows[adj.y as usize].len() {
                    continue;
                }
                if rows[adj.y as usize][adj.x as usize] != ' ' {
                    let dir_x = adj.x - next.x;
                    let dir_y = adj.y - next.y;
                    dir = Point { x: dir_x, y: dir_y };

                    found = true;
                    break;
                }
            }
            if found {
                current = next;
                continue;
            } else {
                panic!("finished, found {result}");
            }
        }
        if next_char.is_alphabetic() {
            result.push(next_char);
        }
        if next_char == ' ' {
            println!("finished: {result}, {count}");
            return;
        }
        current = next;
    }
}
