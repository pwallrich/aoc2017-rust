use std::fs;

fn main() {
    let input: Vec<char> = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .chars()
        .collect();
    // let input: Vec<char> = "123123".chars().collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<char>) {
    let mut res = input
        .windows(2)
        .filter_map(|pair| {
            if pair[0] == pair[1] {
                pair[0].to_digit(10)
            } else {
                Some(0)
            }
        })
        .reduce(|a, b| a + b)
        .unwrap();
    if input[0] == *input.last().unwrap() {
        let val = input[0].to_digit(10).unwrap();
        res += val;
    }
    println!("{}", res)
}

fn part2(input: &Vec<char>) {
    let mut sum: u32 = 0;
    for (idx, val) in input.iter().enumerate() {
        let to_compare = (idx + (input.len() / 2)) % input.len();
        if input[to_compare] == *val {
            sum += val.to_digit(10).unwrap();
        }
    }

    println!("{}", sum)
}
