use std::io::{self, BufRead};

fn day_1() {
    let mut expense_report: Vec<i32> = Vec::new();

    for line in io::stdin().lock().lines() {
        expense_report.push(line.unwrap().parse::<i32>().unwrap())
    }

    // Challenge 1 specific
    for first_number in &expense_report {
        for second_number in &expense_report {
            if first_number + second_number == 2020 {
                println!("First challenge result: {}", first_number * second_number);
            }
        }
    }

    // Challenge 2 specific
    for first_number in &expense_report {
        for second_number in &expense_report {
            for third_number in &expense_report {
                if first_number + second_number + third_number == 2020 {
                    println!(
                        "Second challenge result: {}",
                        first_number * second_number * third_number
                    );
                }
            }
        }
    }
}

fn day_2() {
    let mut first_challenge_valid_passwords = 0;
    let mut second_challenge_valid_passwords = 0;

    for line in io::stdin().lock().lines() {
        let line_str = line.unwrap();
        let split_line: Vec<&str> = line_str.split_whitespace().collect();
        let policy: Vec<&str> = split_line[0].split('-').collect();
        let policy_min = policy[0].parse::<i32>().unwrap();
        let policy_max = policy[1].parse::<i32>().unwrap();

        let policy_letter = split_line[1].chars().next().unwrap();

        let password = split_line[2];

        let mut letter_qty = 0;

        // Challenge 1 specific
        for letter in password.chars() {
            if letter == policy_letter {
                letter_qty += 1;
            }
        }

        if policy_min <= letter_qty && letter_qty <= policy_max {
            first_challenge_valid_passwords += 1;
        }
        // Challenge 2 specific
        letter_qty = 0;
        let password_letters: Vec<char> = password.chars().collect();
        if password_letters[(policy_min - 1) as usize] == policy_letter {
            letter_qty += 1;
        }

        if password_letters[(policy_max - 1) as usize] == policy_letter {
            letter_qty += 1;
        }
        if letter_qty == 1 {
            second_challenge_valid_passwords += 1;
        }
    }

    println!(
        "First challenge result: {}",
        first_challenge_valid_passwords
    );
    println!(
        "Second challenge result: {}",
        second_challenge_valid_passwords
    );
}

fn main() {
    day_2();
}
