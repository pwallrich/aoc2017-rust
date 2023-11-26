use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input =
        fs::read_to_string("day12/input.txt").expect("Should have been able to read the file");

    // let input = "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";

    let mut connections: HashMap<i32, HashSet<i32>> = HashMap::new();

    for row in input.lines() {
        let numbers: Vec<i32> = row
            .replace(" <-> ", ", ")
            .split(", ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if connections.contains_key(&numbers[0]) {
            connections
                .get_mut(&numbers[0])
                .unwrap()
                .extend(numbers.iter().skip(1))
        } else {
            connections.insert(
                numbers[0],
                HashSet::from_iter(numbers.clone().iter().skip(1).map(|x| *x)),
            );
        }

        for number in numbers.clone().iter().skip(1) {
            if connections.contains_key(&number) {
                connections.get_mut(&number).unwrap().insert(numbers[0]);
            } else {
                connections.insert(*number, HashSet::from([numbers[0]]));
            }
        }
    }

    let mut connected: Vec<i32> = Vec::from([0]);

    // println!("{:?}", connections);

    for (key, numbers) in connections.clone() {
        println!("testing {}", key);
        let mut seen = HashSet::from([&key]);
        let mut next: Vec<&i32> = numbers.iter().filter(|x| **x != key).collect();
        while !next.is_empty() {
            let next_val = next.pop().unwrap();
            if seen.contains(next_val) {
                continue;
            }
            seen.insert(next_val);
            if *next_val == 0 {
                connected.push(key);
                break;
            }
            next.extend(&connections[next_val]);
        }
    }

    println!("part1: {}", connected.len());

    let mut groups: Vec<HashSet<i32>> = Vec::new();
    for (key, numbers) in connections.clone() {
        if groups.iter().any(|x| x.contains(&key)) {
            continue;
        }

        let group = get_group(key, &HashSet::new(), &connections);
        groups.push(group);
    }

    println!("{}", groups.len())
}

fn get_group(
    starting_at: i32,
    seen: &HashSet<i32>,
    connections: &HashMap<i32, HashSet<i32>>,
) -> HashSet<i32> {
    if seen.contains(&starting_at) {
        return seen.clone();
    }
    let mut group = connections[&starting_at].clone();
    let mut new_seen = seen.clone();
    new_seen.insert(starting_at);

    for value in group.clone() {
        let indirect = get_group(value, &new_seen, connections);
        group.extend(indirect);
    }

    return group;
}
