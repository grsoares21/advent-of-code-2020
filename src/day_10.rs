use std::io::{self, Read};
use std::collections::HashMap;

pub fn day_10() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let mut input_data: Vec<i64> = input
        .split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut one_differences = 0;
    let mut three_differences = 0;

    let max_jolt_adapter = *input_data.iter().max().unwrap();
    input_data.push(max_jolt_adapter + 3);
    input_data.push(0);

    input_data.sort();
    for i in 0..input_data.len() - 1 {
        match input_data[i + 1] - input_data[i] {
            1 => one_differences += 1,
            3 => three_differences += 1,
            _ => {}
        }
    }

    println!("First challenge result: {}", one_differences * three_differences);

    let mut adapter_possibilities: HashMap<i64, Vec<i64>> = HashMap::new();

    for adapter in &input_data {
        adapter_possibilities.insert(
            *adapter, 
            (&input_data)
                .iter()
                .map(|a| *a)
                .filter(|dst_adapter| dst_adapter - *adapter >= 1 && dst_adapter - *adapter <= 3)
                .collect()
        );
    }

    let mut number_of_ways: HashMap<i64, i64> = HashMap::new();
    number_of_ways.insert(input_data[input_data.len() - 1], 1);
    for i in (0..input_data.len() - 1).rev() {
        let possible_adapters = adapter_possibilities.get(&input_data[i]).unwrap();
        let possible_adapters_numbers_of_ways: Vec<i64> = possible_adapters
            .iter()
            .map(|adapter| *number_of_ways.get(adapter).unwrap())
            .collect();
        number_of_ways.insert(input_data[i], possible_adapters_numbers_of_ways.iter().fold(0, |a, b| a + b));
    }

    println!("Second challenge result: {}", number_of_ways.get(&0).unwrap());
}
