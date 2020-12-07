use std::collections::HashSet;
use std::io::{self, Read};

pub fn day_6() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut total_first_challenge = 0;
    let mut total_second_challenge = 0;

    for group in &groups {
        let mut group_answers = HashSet::new();

        let people: Vec<&str> = group.split("\n").collect();
        for person in &people {
            for answer in person.chars() {
                group_answers.insert(answer);
            }
        }

        total_first_challenge += group_answers.len() as i32;

        for group_answer in group_answers {
            let mut all_people_contain = true;
            for person in &people {
                if !person.contains(group_answer) {
                    all_people_contain = false;
                }
            }

            if all_people_contain {
                total_second_challenge += 1;
            }
        }
    }

    println!("First challenge result: {}", total_first_challenge);
    println!("Second challenge result: {}", total_second_challenge);
}
