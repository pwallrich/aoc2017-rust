use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input =
        fs::read_to_string("day18/input.txt").expect("Should have been able to read the file");

    // let input =
    //     // "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2";
    // "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d";
    part2(&input);
}

fn part1(input: &str) {
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut idx: i32 = 0;
    let instructions: Vec<Vec<&str>> = input
        .split("\n")
        .map(|x| x.split_whitespace().collect())
        .collect();

    let mut frequencies: Vec<i64> = Vec::new();
    while idx >= 0 && (idx as usize) < instructions.len() {
        let instruction = &instructions[idx as usize];
        let output = perform_step(instruction, &mut registers);
        if let Some(register) = output.recover {
            if frequencies.len() != 0 {
                println!("recovered frequency: {}", frequencies.last().unwrap());
                println!("frequencies: {:?}", frequencies.len());
                return;
            }
        } else if let Some(freq) = output.frequency {
            frequencies.push(freq);
        }
        idx += output.idx_offset;
    }
    panic!("didn't recover");
}

struct ProgramState {
    registers: HashMap<char, i64>,
    idx: i32,
    sent: VecDeque<i64>,
    is_waiting: Option<char>,
}

fn part2(input: &str) {
    let mut states = vec![
        ProgramState {
            registers: HashMap::new(),
            idx: 0,
            sent: VecDeque::new(),
            is_waiting: None,
        },
        ProgramState {
            registers: HashMap::new(),
            idx: 0,
            sent: VecDeque::new(),
            is_waiting: None,
        },
    ];
    states[0].registers.insert('p', 0);
    states[1].registers.insert('p', 1);

    let instructions: Vec<Vec<&str>> = input
        .split("\n")
        .map(|x| x.split_whitespace().collect())
        .collect();

    let mut sent_count = 0;
    let mut print_count = 0;
    loop {
        if states[0].is_waiting != None
            && states[1].sent.len() == 0
            && states[1].is_waiting != None
            && states[0].sent.len() == 0
        {
            break;
        }

        for i in 0..2 {
            let mut recover_count = 0;
            let other = (i + 1) % 2;
            println!();
            loop {
                if let Some(register) = states[i].is_waiting {
                    if let Some(op) = states[other].sent.pop_front() {
                        recover_count += 1;
                        if i == 1 {
                            println!("{recover_count}: Recovered {op}");
                        }
                        states[i].is_waiting = None;
                        states[i].registers.insert(register, op);
                    } else {
                        break;
                    }
                }
                let state = states.get_mut(i).unwrap();

                let instruction = &instructions[state.idx as usize];
                let output = perform_step_2(instruction, &mut state.registers);

                if let Some(register) = output.recover {
                    state.is_waiting = Some(register);
                } else if let Some(freq) = output.frequency {
                    if i == 1 {
                        println!("sent {freq}");
                    }
                    state.sent.push_back(freq);
                    if i == 1 {
                        sent_count += 1;
                    }
                }
                state.idx += output.idx_offset;
            }
        }
    }
    println!("{sent_count}")
}

struct StepOutput {
    idx_offset: i32,
    frequency: Option<i64>,
    recover: Option<char>,
}

fn perform_step(instruction: &Vec<&str>, registers: &mut HashMap<char, i64>) -> StepOutput {
    match instruction[0] {
        "set" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "add" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = registers.get(&to_set).unwrap_or(&0);
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, curr + value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "mul" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = registers.get(&to_set).unwrap_or(&0);
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, curr * value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "mod" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = registers.get(&to_set).unwrap_or(&0);
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, curr % value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "snd" => {
            let to_send = instruction[1].chars().nth(0).unwrap();
            let mut frequency = None;
            if let Some(value) = registers.get(&to_send) {
                frequency = Some(*value);
            }
            return StepOutput {
                idx_offset: 1,
                frequency: frequency,
                recover: None,
            };
        }
        "rcv" => {
            let to_send = instruction[1].chars().nth(0).unwrap();
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: Some(to_send),
            };
        }
        "jgz" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = registers.get(&to_set).unwrap_or(&0);
            if curr <= &0 {
                return StepOutput {
                    idx_offset: 1,
                    frequency: None,
                    recover: None,
                };
            }
            let value = get_value_or_from_register(&registers, instruction[2]);
            return StepOutput {
                idx_offset: value as i32,
                frequency: None,
                recover: None,
            };
        }
        _ => panic!("Unknown instruction"),
    }
}

fn perform_step_2(instruction: &Vec<&str>, registers: &mut HashMap<char, i64>) -> StepOutput {
    match instruction[0] {
        "set" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "add" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = registers.get(&to_set).unwrap_or(&0);
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, curr + value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "mul" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = registers.get(&to_set).unwrap_or(&0);
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, curr * value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "mod" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = registers.get(&to_set).unwrap_or(&0);
            let value = get_value_or_from_register(registers, instruction[2]);
            registers.insert(to_set, curr % value);
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: None,
            };
        }
        "snd" => {
            let to_send = get_value_or_from_register(registers, instruction[1]);
            let frequency = Some(to_send);
            return StepOutput {
                idx_offset: 1,
                frequency: frequency,
                recover: None,
            };
        }
        "rcv" => {
            let to_send = instruction[1].chars().nth(0).unwrap();
            return StepOutput {
                idx_offset: 1,
                frequency: None,
                recover: Some(to_send),
            };
        }
        "jgz" => {
            let to_set = instruction[1].chars().nth(0).unwrap();
            let curr = if to_set.is_digit(10) {
                to_set.to_digit(10).unwrap() as i64
            } else {
                *registers.get(&to_set).unwrap_or(&0)
            };
            if curr <= 0 {
                return StepOutput {
                    idx_offset: 1,
                    frequency: None,
                    recover: None,
                };
            }
            let value = get_value_or_from_register(&registers, instruction[2]);
            return StepOutput {
                idx_offset: value as i32,
                frequency: None,
                recover: None,
            };
        }
        _ => panic!("Unknown instruction"),
    }
}

fn get_value_or_from_register(registers: &HashMap<char, i64>, input: &str) -> i64 {
    if let Ok(value) = input.parse::<i64>() {
        return value;
    }
    let register_of_value = input.chars().nth(0).unwrap();
    return *registers.get(&register_of_value).unwrap();
}
