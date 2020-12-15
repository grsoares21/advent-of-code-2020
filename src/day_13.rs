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

/*fn second_challenge(input: &String) {
    println!("Second challenge result: {}", 0);
}*/

pub fn day_13() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    first_challenge(&input);
    //second_challenge(&input);
}
