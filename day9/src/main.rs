use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    // let input = "<!!!>>";
    // let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
    // let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let res = calculate_groups_value(&input.chars().collect::<Vec<char>>(), 0, 0);
    println!("{}", res.0)
}

fn part2(input: &str) {
    let res = calculate_garbage_value(&input.chars().collect::<Vec<char>>(), 0);
    println!("{}", res.0)
}

fn calculate_groups_value(input: &Vec<char>, start_idx: usize, start_val: i32) -> (i32, usize) {
    println!("starting at {:?}", start_idx);
    let mut idx = start_idx;
    let mut sum = 0;
    let mut is_garbage = false;
    while idx < input.len() {
        match (is_garbage, input[idx]) {
            (false, '{') => {
                let (total, pos) = calculate_groups_value(input, idx + 1, start_val + 1);
                sum += total;
                idx = pos;
                println!("got {:?}, now: {}, at: {}", total, sum, idx);
            }
            (false, '}') => return (sum + start_val, idx),
            (true, '!') => idx += 1,
            (false, '<') => is_garbage = true,
            (_, '>') => is_garbage = false,
            _ => (),
        }
        idx += 1
    }

    return (sum + start_val, idx);
}

fn calculate_garbage_value(input: &Vec<char>, start_idx: usize) -> (i32, usize) {
    println!("starting at {:?}", start_idx);
    let mut idx = start_idx;
    let mut sum = 0;
    let mut is_garbage = false;
    while idx < input.len() {
        match (is_garbage, input[idx]) {
            (false, '{') => {
                let (total, pos) = calculate_garbage_value(input, idx + 1);
                sum += total;
                idx = pos;
                println!("got {:?}, now: {}, at: {}", total, sum, idx);
            }
            (false, '}') => return (sum, idx),
            (true, '!') => idx += 1,
            (false, '<') => is_garbage = true,
            (_, '>') => is_garbage = false,
            (true, _) => sum += 1,
            _ => (),
        }
        idx += 1
    }

    return (sum, idx);
}
