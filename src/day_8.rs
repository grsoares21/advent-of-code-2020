use std::collections::HashSet;
use std::io::{self, Read};

pub fn day_8() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let instructions: Vec<(&str, i32)> = input
        .split("\n")
        .map(|instruction_str| {
            let split_instruction: Vec<&str> = instruction_str.split_whitespace().collect();
            (
                split_instruction[0],
                split_instruction[1].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut acc = 0;
    let mut i: i32 = 0;

    // Challenge 1 specific
    let mut executed_instructions = HashSet::new();
    loop {
        if executed_instructions.contains(&i) {
            break;
        }
        executed_instructions.insert(i);

        match instructions[i as usize] {
            ("jmp", value) => i += value,
            ("nop", _) => i += 1,
            ("acc", value) => {
                acc += value;
                i += 1;
            }
            _ => panic!("Unidentified instruction"),
        }
    }
    println!("First challenge result: {}", acc);

    // Challenge 2 specific
    for j in 0..instructions.len() {
        let mut modified_instructions = instructions.clone();
        match instructions[j] {
            ("jmp", value) => modified_instructions[j] = ("nop", value),
            ("nop", value) => modified_instructions[j] = ("jmp", value),
            _ => {}
        }

        acc = 0;
        i = 0;
        executed_instructions = HashSet::new();
        let mut found_instruction = false;
        loop {
            if executed_instructions.contains(&i) {
                break;
            }
            executed_instructions.insert(i);

            match modified_instructions[i as usize] {
                ("jmp", value) => i += value,
                ("nop", _) => i += 1,
                ("acc", value) => {
                    acc += value;
                    i += 1;
                }
                _ => panic!("Unidentified instruction"),
            }

            if i as usize == instructions.len() {
                found_instruction = true;
                break;
            }
        }

        if found_instruction {
            println!("Second challenge result: {}", acc);
            break;
        }
    }
}
