use std::io::{self, Read};

pub fn day_9() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let input_data: Vec<i64> = input
        .split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut first_challenge_result = 0;

    for i in 0..input_data.len() {
        if i > 24 {
            let mut result_found = false;

            for j in (i-25)..i {
                for k in (j+1)..i {
                    if input_data[j] + input_data[k] == input_data[i] {
                        result_found = true;
                    }
                }
            }

            if !result_found {
                first_challenge_result = input_data[i];
                break;
            }
        }
    }

    println!("First challenge result: {}", first_challenge_result);

    let mut lowest_index = 0;
    let mut highest_index = 1;
    let mut current_sum = input_data[lowest_index] + input_data[highest_index];

    while current_sum != first_challenge_result {
        if current_sum < first_challenge_result {
            highest_index += 1;
            current_sum += input_data[highest_index];
        } else {
            current_sum -= input_data[lowest_index];
            lowest_index += 1;            
        }

        if highest_index > input_data.len() {
            panic!("Searched the whole input and didn't find the sum.")
        }
    }

    let input_slice = &input_data[lowest_index..=highest_index];
    println!("Second challenge result: {}", input_slice.iter().min().unwrap() + input_slice.iter().max().unwrap());
}
