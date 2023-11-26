use std::{cmp::max, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";

    part1(input.lines());
    part2(input.lines());
}

fn part1<'a>(rows: impl Iterator<Item = &'a str>) {
    let mut registers: HashMap<&str, i32> = HashMap::new();

    for line in rows {
        let args: Vec<&str> = line.split_whitespace().collect();
        let register_to_manipulate = args[0];
        let instruction = args[1];
        let instruction_arg = args[2].parse::<i32>().unwrap();
        let register_to_compare = args[4];
        let num_to_compare = args[6].parse::<i32>().unwrap();

        let comparator = match args[5] {
            "<" => |x, y| x < y,
            ">" => |x, y| x > y,
            "<=" => |x, y| x <= y,
            ">=" => |x, y| x >= y,
            "==" => |x, y| x == y,
            "!=" => |x, y| x != y,
            _ => panic!(),
        };

        let to_compare = registers.get(register_to_compare).unwrap_or(&0);
        if !comparator(to_compare, &num_to_compare) {
            continue;
        }
        let value = registers.get(register_to_manipulate).unwrap_or(&0);

        match instruction {
            "inc" => _ = registers.insert(register_to_manipulate, value + instruction_arg),
            "dec" => _ = registers.insert(register_to_manipulate, value - instruction_arg),
            _ => panic!(),
        }
    }

    let result = registers.values().max();
    println!("{:?}", result);
}

fn part2<'a>(rows: impl Iterator<Item = &'a str>) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut maxVal: i32 = 0;
    for line in rows {
        let args: Vec<&str> = line.split_whitespace().collect();
        let register_to_manipulate = args[0];
        let instruction = args[1];
        let instruction_arg = args[2].parse::<i32>().unwrap();
        let register_to_compare = args[4];
        let num_to_compare = args[6].parse::<i32>().unwrap();

        let comparator = match args[5] {
            "<" => |x, y| x < y,
            ">" => |x, y| x > y,
            "<=" => |x, y| x <= y,
            ">=" => |x, y| x >= y,
            "==" => |x, y| x == y,
            "!=" => |x, y| x != y,
            _ => panic!(),
        };

        let to_compare = registers.get(register_to_compare).unwrap_or(&0);
        if !comparator(to_compare, &num_to_compare) {
            continue;
        }
        let value = registers.get(register_to_manipulate).unwrap_or(&0);

        let new = match instruction {
            "inc" => value + instruction_arg,
            "dec" => value - instruction_arg,
            _ => panic!(),
        };

        maxVal = max(new, maxVal);

        registers.insert(register_to_manipulate, new);
    }

    // let result = registers.values().max();
    println!("{:?}", maxVal);
}
