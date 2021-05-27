use regex::Regex;
use std::io::{self, Read};

enum Instruction {
    MaskChange(String),
    MemoryChange(i64, i64),
}

fn parse_input(input: &String) -> Vec<Instruction> {
    let lines: Vec<&str> = input.split('\n').collect();

    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|line| {
            let instruction: Vec<&str> = line.split(" = ").collect();

            if instruction[0] == "mask" {
                return Instruction::MaskChange(String::from(instruction[1]));
            }

            let memory_change_regex = Regex::new(r"^mem\[(\d+)\]$").unwrap();
            let memory_change_captures = memory_change_regex.captures(instruction[0]).unwrap();
            let memory_address = memory_change_captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap();

            let memory_value = instruction[1].parse::<i64>().unwrap();

            return Instruction::MemoryChange(memory_address, memory_value);
        })
        .collect();

    return instructions;
}

fn first_challenge(instructions: Vec<Instruction>) -> i64 {
    let memory: Vec<i64> = [0; 65536].to_vec();

    fn run_instructions(
        current_mask: &str,
        left_instructions: &[Instruction],
        memory: Vec<i64>,
    ) -> Vec<i64> {
        if left_instructions.len() == 0 {
            return memory;
        }

        let (head_inst, left_insts) = left_instructions.split_at(1);

        match &head_inst[0] {
            Instruction::MaskChange(new_mask) => {
                return run_instructions(new_mask, left_insts, memory)
            }
            Instruction::MemoryChange(memory_address, new_memory_value) => {
                let memory_value_binary = format!("{:036b}", new_memory_value);

                let new_masked_value: String = current_mask
                    .chars()
                    .enumerate()
                    .map(|(i, character)| match character {
                        'X' => memory_value_binary.chars().nth(i).unwrap(), // replace by memory value
                        _ => character,
                    })
                    .collect();

                let new_value = i64::from_str_radix(new_masked_value.as_str(), 2).unwrap();
                let new_value_address = *memory_address as usize;

                return run_instructions(
                    current_mask,
                    left_insts,
                    memory
                        .iter()
                        .enumerate()
                        .map(|(i, memory_cell)| {
                            if i == new_value_address {
                                new_value
                            } else {
                                *memory_cell
                            }
                        })
                        .collect(),
                );
            }
        }
    }

    let resulting_memory = run_instructions("", instructions.as_slice(), memory);

    return resulting_memory.iter().fold(0, |a, b| a + b);
}

pub fn second_challenge(input: &String) {}

pub fn day_14() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let instructions = parse_input(&input);
    println!("{}", first_challenge(instructions));
    //second_challenge(&input);
}
