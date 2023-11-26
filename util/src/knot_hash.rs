pub fn _knot_hash(input: &str) -> String {
    let mut idx = 0;
    let mut offset = 0;
    let mut instructions: Vec<usize> = input.chars().map(|x| (x as u32) as usize).collect();

    instructions.append(&mut [17, 31, 73, 47, 23].to_vec());

    let mut values: Vec<i32> = (0..=255).collect();

    for i in 0..64 {
        // println!("iteration {i}, {offset}, {idx}");

        for val in instructions.iter() {
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
            idx = (idx + (val + offset)) % values.len();
            offset += 1;
        }
    }
    let dense = dense_hash(values);
    return convert_hex(dense);
}

fn dense_hash(input: Vec<i32>) -> Vec<i32> {
    let mut dense = Vec::new();
    for chunk in input.chunks(16) {
        let mut curr = chunk[0];
        for val in chunk.into_iter().skip(1) {
            curr ^= val;
        }
        dense.push(curr);
    }
    return dense;
}

fn convert_hex(input: Vec<i32>) -> String {
    let mut res: String = String::from("");
    for item in input {
        let foo = format!("{:02x}", item);
        res.push_str(&foo);
    }
    return res;
}

// fn print_hex(input: Vec<i32>) {
//     for item in input {
//         // print!("{:02x}", item);
//         let foo = format!("{:02x}", item);
//         print!("{foo}");
//     }
//     println!();
// }
