use std::collections::{HashMap, HashSet};

fn main() {
    let mut curr_state = 'A';
    let mut tape: HashSet<i32> = HashSet::new();
    let mut index = 0;
    for i in 0..12_261_543 {
        let res = touring(curr_state, tape.contains(&index));
        if res.0 {
            tape.insert(index);
        } else {
            tape.remove(&index);
        }
        curr_state = res.2;
        index += res.1;
        if i % 10_000 == 0 {
            println!("{i}: {curr_state}, {index}, {:?}", tape.len());
        }
    }

    println!("{:?}", tape.len())
    // match
}

fn test_states(state: char, value: bool) -> (bool, i32, char) {
    match (state, value) {
        ('A', false) => (true, 1, 'B'),
        ('A', true) => (false, -1, 'B'),
        ('B', false) => (true, -1, 'A'),
        ('B', true) => (true, 1, 'A'),
        _ => panic!("unknown state"),
    }
    //     In state A:
    //   If the current value is 0:
    //     - Write the value 1.
    //     - Move one slot to the right.
    //     - Continue with state B.
    //   If the current value is 1:
    //     - Write the value 0.
    //     - Move one slot to the left.
    //     - Continue with state B.

    // In state B:
    //   If the current value is 0:
    //     - Write the value 1.
    //     - Move one slot to the left.
    //     - Continue with state A.
    //   If the current value is 1:
    //     - Write the value 1.
    //     - Move one slot to the right.
    //     - Continue with state A.
}

fn touring(state: char, value: bool) -> (bool, i32, char) {
    match (state, value) {
        ('A', false) => (true, 1, 'B'),
        ('A', true) => (false, -1, 'C'),

        ('B', false) => (true, -1, 'A'),
        ('B', true) => (true, 1, 'C'),

        ('C', false) => (true, 1, 'A'),
        ('C', true) => (false, -1, 'D'),

        ('D', false) => (true, -1, 'E'),
        ('D', true) => (true, -1, 'C'),

        ('E', false) => (true, 1, 'F'),
        ('E', true) => (true, 1, 'A'),

        ('F', false) => (true, 1, 'A'),
        ('F', true) => (true, 1, 'E'),
        _ => panic!("unknown state"),
    }
}
