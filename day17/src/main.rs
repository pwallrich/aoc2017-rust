use std::{
    collections::{HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

fn main() {
    let input = 304;

    // let input = 3;

    let mut curr = 0;
    let mut pos = 0;
    for i in 1..=50_000_000 {
        let buffer_length = i;
        pos = (input + pos) % buffer_length;
        if pos == 0 {
            curr = i;
        }
        pos += 1;
    }

    println!("{curr}");
}
