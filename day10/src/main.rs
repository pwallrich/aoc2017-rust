use util::{self, knot_hash};
fn main() {
    let input = "70,66,255,2,48,0,54,48,80,141,244,254,160,108,1,41";

    let start = (0..=255).collect();

    part2(input, start);
}

fn part1(input: &Vec<usize>, start: Vec<i32>) {
    let mut idx = 0;
    let mut values = start;

    for (instruction_idx, val) in input.iter().enumerate() {
        if val >= &values.len() {
            continue;
        }
        let mut temp = Vec::new();
        for offset in 0..*val {
            let i = (idx + offset) % values.len();
            temp.push(values[i])
        }
        temp.reverse();
        for offset in 0..*val {
            let i = (idx + offset) % values.len();
            values[i] = temp[offset];
        }
        idx += val + instruction_idx
    }
    println!("values: {:?}", values);
    println!("{}", values[0] * values[1])
}

fn part2(input: &str, start: Vec<i32>) {
    let res = knot_hash(input);
    println!("{res}");
}
