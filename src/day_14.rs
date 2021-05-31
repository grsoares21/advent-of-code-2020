use im::HashMap;
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

fn replace_char_at(text: String, index: usize, new_char: char) -> String {
    return text
        .chars()
        .enumerate()
        .map(|(i, character)| if i == index { new_char } else { character })
        .collect();
}

fn write_all_memory_for_masked_address(
    masked_address_value: String,
    value: i64,
    current_memory: HashMap<String, i64>,
) -> HashMap<String, i64> {
    fn write_all_memory_for_each_char(
        current_index: usize,
        masked_address: String,
        current_memory: HashMap<String, i64>,
        value: i64,
    ) -> HashMap<String, i64> {
        if current_index == masked_address.len() {
            return current_memory.update(masked_address, value);
        }

        if masked_address.chars().nth(current_index).unwrap() == 'X' {
            let masked_address_replaced_with_zero =
                replace_char_at(masked_address.clone(), current_index, '0');
            let masked_address_replaced_with_one =
                replace_char_at(masked_address, current_index, '1');

            let memory_for_address_with_zero = write_all_memory_for_masked_address(
                masked_address_replaced_with_zero,
                value,
                current_memory,
            );
            let final_memory = write_all_memory_for_masked_address(
                masked_address_replaced_with_one,
                value,
                memory_for_address_with_zero,
            );

            return final_memory;
        }

        return write_all_memory_for_each_char(
            current_index + 1,
            masked_address,
            current_memory,
            value,
        );
    }
    return write_all_memory_for_each_char(0, masked_address_value, current_memory, value);
}

fn second_challenge(instructions: Vec<Instruction>) -> i64 {
    let memory: HashMap<String, i64> = HashMap::new();

    fn run_instructions(
        current_mask: &str,
        left_instructions: &[Instruction],
        memory: HashMap<String, i64>,
    ) -> HashMap<String, i64> {
        if left_instructions.len() == 0 {
            return memory;
        }

        let (head_inst, left_insts) = left_instructions.split_at(1);

        match &head_inst[0] {
            Instruction::MaskChange(new_mask) => {
                return run_instructions(new_mask, left_insts, memory)
            }
            Instruction::MemoryChange(memory_address, new_memory_value) => {
                let memory_address_binary = format!("{:036b}", memory_address);

                let new_masked_address: String = current_mask
                    .chars()
                    .enumerate()
                    .map(|(i, character)| match character {
                        'X' => 'X',                                           // replace by memory value
                        '0' => memory_address_binary.chars().nth(i).unwrap(), // replace by memory value
                        '1' => '1',
                        _ => panic!("Invalid mask value"),
                    })
                    .collect();

                let new_memory = write_all_memory_for_masked_address(
                    new_masked_address,
                    *new_memory_value,
                    memory.clone(),
                );
                let memory = run_instructions(current_mask, left_insts, new_memory);

                return memory;
            }
        }
    }

    let resulting_memory = run_instructions("", instructions.as_slice(), memory);

    return resulting_memory.values().fold(0, |a, b| a + b);
}

pub fn day_14() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let instructions = parse_input(&input);
    println!("{}", second_challenge(instructions));
}
