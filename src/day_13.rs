use std::io::{self, Read};

fn first_challenge(input: &String) {
    let lines: Vec<&str> = input.split('\n').collect();
    let initial_timestamp = lines[0].parse::<i32>().unwrap();

    let buses: Vec<i32> = lines[1]
        .split(',')
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse::<i32>().unwrap())
        .collect();

    let mut earliest_bus_id: i32 = 0;
    let mut earliest_bus_departure = i32::MAX;
    for bus in buses {
        let earliest_departure = (initial_timestamp / bus) * bus + bus;
        if earliest_departure < earliest_bus_departure {
            earliest_bus_departure = earliest_departure;
            earliest_bus_id = bus;
        }
    }

    println!(
        "First challenge result: {}",
        earliest_bus_id * (earliest_bus_departure - initial_timestamp)
    );
}

/**
 * Chinese remainder theorem algorithm extracted from:
 * https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
 *
*/
fn extended_greatest_common_divisor(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_greatest_common_divisor(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
fn modular_inverse(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = extended_greatest_common_divisor(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * modular_inverse(p, modulus)? * p
    }
    Some(sum % prod)
}

fn second_challenge(input: &String) {
    let lines: Vec<&str> = input.split('\n').collect();
    let buses_and_indexes: Vec<(usize, i32)> = lines[1]
        .split(',')
        .enumerate()
        .filter(|(_, bus_id)| *bus_id != "x")
        .map(|(index, bus_id)| (index, bus_id.parse::<i32>().unwrap()))
        .collect();

    let mut max_index = i32::MIN;
    for (index, _) in &buses_and_indexes {
        if *index as i32 > max_index {
            max_index = *index as i32;
        }
    }

    let residues: Vec<i64> = (&buses_and_indexes)
        .iter()
        .map(|(index, _)| max_index as i64 - *index as i64)
        .collect();
    let modulii: Vec<i64> = (&buses_and_indexes)
        .iter()
        .map(|(_, bus_id)| *bus_id as i64)
        .collect();

    let value = chinese_remainder(&residues, &modulii);

    println!(
        "Second challenge result: {}",
        value.unwrap() - max_index as i64
    );
}

pub fn day_13() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    first_challenge(&input);
    second_challenge(&input);
}
