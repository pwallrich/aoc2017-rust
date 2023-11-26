use std::{
    collections::{HashMap, VecDeque},
    fs,
    hash::Hash,
};

fn main() {
    let input =
        fs::read_to_string("day16/input.txt").expect("Should have been able to read the file");

    let start = "abcdefghijklmnop";
    let after_first = part1(&input, &start.chars().collect());

    part2(&input, &start.chars().collect());
    return;
}

fn part1(input: &str, start: &Vec<char>) -> Vec<char> {
    println!("{:?}", start);
    let mut curr = start.clone();

    let instructions = input.split(",");

    for instr in instructions {
        println!("{instr}");
        match &instr[..1] {
            "s" => {
                let amount = instr[1..].parse::<usize>().unwrap();
                curr.rotate_right(amount as usize);
            }
            "x" => {
                let vals: Vec<&str> = instr[1..].split("/").collect();
                let first = vals[0].parse::<usize>().unwrap();
                let second = vals[1].parse::<usize>().unwrap();
                curr.swap(first, second);
            }
            "p" => {
                let vals: Vec<&str> = instr[1..].split("/").collect();

                let first_index = curr
                    .iter()
                    .position(|x| x == &vals[0].chars().nth(0).unwrap())
                    .unwrap();
                let second_index = curr
                    .iter()
                    .position(|x| x == &vals[1].chars().nth(0).unwrap())
                    .unwrap();
                curr.swap(first_index, second_index);
            }
            _ => panic!("unknown instruction"),
        }
        // println!("{:?}", curr);
    }
    return curr;
}

fn part2(input: &str, start: &Vec<char>) {
    println!("{:?}", start);
    let mut curr = start.clone();
    let mut seen: HashMap<String, i32> = HashMap::new();
    let first = start.iter().collect();
    seen.insert(first, 0);

    let instructions: Vec<&str> = input.split(",").collect();
    for i in 1..1_000_000_000 {
        for instr in &instructions {
            // println!("{instr}");
            match &instr[..1] {
                "s" => {
                    let amount = instr[1..].parse::<usize>().unwrap();
                    curr.rotate_right(amount as usize);
                }
                "x" => {
                    let vals: Vec<&str> = instr[1..].split("/").collect();
                    let first = vals[0].parse::<usize>().unwrap();
                    let second = vals[1].parse::<usize>().unwrap();
                    curr.swap(first, second);
                }
                "p" => {
                    let vals: Vec<&str> = instr[1..].split("/").collect();

                    let first_index = curr
                        .iter()
                        .position(|x| x == &vals[0].chars().nth(0).unwrap())
                        .unwrap();
                    let second_index = curr
                        .iter()
                        .position(|x| x == &vals[1].chars().nth(0).unwrap())
                        .unwrap();
                    curr.swap(first_index, second_index);
                }
                _ => panic!("unknown instruction"),
            }
        }

        let string: String = curr.iter().collect();
        if seen.contains_key(&string) {
            println!("I'm here {i}, {}", string);
            let after_one_billion = 1_000_000_000 % i;

            let result = seen
                .iter()
                .find_map(|x| {
                    if *x.1 == after_one_billion {
                        Some(x.0)
                    } else {
                        None
                    }
                })
                .unwrap();
            println!("{}", result);
            return;
        }
        seen.insert(string.to_string(), i);
    }
    let res: String = curr.iter().collect();
    println!("{res}");
    // return curr;
}
