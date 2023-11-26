use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // let input = "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)\n";

    let rows: Vec<&str> = input.lines().collect();
    part1(&rows);
    part2(&rows);
}

fn part1(input: &Vec<&str>) {
    let mut nodes: HashSet<&str> = HashSet::new();
    let mut edges: HashSet<&str> = HashSet::new();
    for row in input {
        let in_out: Vec<&str> = row.split(" -> ").collect();
        let name = in_out[0].split_whitespace().nth(0).unwrap();
        nodes.insert(name);
        if in_out.len() < 2 {
            continue;
        }
        for edge in in_out[1].split(", ") {
            edges.insert(edge);
        }
    }

    for node in nodes.difference(&edges) {
        println!("{},", node);
    }
}

fn part2(input: &Vec<&str>) {
    let mut nodes: HashMap<&str, (i32, Vec<&str>)> = HashMap::new();
    let mut edges: HashSet<&str> = HashSet::new();
    for row in input {
        let in_out: Vec<&str> = row.split(" -> ").collect();
        let name_value: Vec<&str> = in_out[0].split_whitespace().collect();
        let name = name_value[0];
        let brackets: &[_] = &['(', ')'];
        let value = name_value[1].trim_matches(brackets).parse::<i32>().unwrap();
        let node_edges = match in_out.len() {
            2 => {
                let mut node_edges: Vec<&str> = Vec::new();
                for edge in in_out[1].split(", ") {
                    node_edges.push(edge);
                    edges.insert(edge);
                }
                node_edges
            }
            _ => Vec::new(),
        };

        nodes.insert(name, (value, node_edges));
    }
    let first = nodes
        .iter()
        .filter(|x| !edges.contains::<str>(x.0))
        .nth(0)
        .unwrap();

    let mut weights: HashMap<&str, i32> = HashMap::new();
    for node in nodes.clone() {
        if node.0 == "ruqgy" {
            println!()
        }
        let value = getValue(&nodes, &edges, node.0);
        weights.insert(node.0, value);
    }

    // let mut curr = first;
    for node in nodes.clone() {
        // check weights
        let mut node_weights: HashMap<i32, i32> = HashMap::new();

        if node.1 .1.len() == 0 {
            continue;
        }
        let mut values: HashMap<&str, i32> = HashMap::new();
        for edge in node.1 .1.clone() {
            let value = weights[edge];
            values.insert(edge, value);
            node_weights.insert(
                value,
                1 + if node_weights.contains_key(&value) {
                    node_weights[&value]
                } else {
                    0
                },
            );
        }
        if node_weights.len() != 1 {
            println!("{:?}, {:?}, {:?}", node_weights, node.0, values);
            // return;
        }
    }
    panic!()
}

fn getValue(nodes: &HashMap<&str, (i32, Vec<&str>)>, edges: &HashSet<&str>, current: &str) -> i32 {
    let mut sum = nodes[current].0;
    for edge in nodes[current].1.clone() {
        sum += getValue(nodes, edges, edge);
    }
    return sum;
}
