use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

pub fn day_4() {
    // Parsing passport inputs into HashMaps of Field -> Value
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    let passport_inputs: Vec<&str> = input.split("\n\n").collect();
    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    for input in passport_inputs {
        let mut passport: HashMap<String, String> = HashMap::new();
        for line in input.split('\n') {
            for field in line.split_whitespace() {
                let kvp: Vec<&str> = field.split(':').collect();
                passport.insert(kvp[0].to_string(), kvp[1].to_string());
            }
        }

        passports.push(passport);
    }

    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_eye_colors: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().collect();
    let mut valid_passports = 0;

    // Challenge 1 specific
    for passport in &passports {
        let mut is_valid = true;
        for required in required_fields.iter() {
            if !passport.contains_key(&required.to_string()) {
                is_valid = false;
            }
        }

        if is_valid {
            valid_passports += 1;
        }
    }

    println!("First challenge result: {}", valid_passports);

    // Challenge 2 specific
    valid_passports = 0;

    let hgt_rgx = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
    let hcl_rgx = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_rgx = Regex::new(r"^[0-9]{9}$").unwrap();

    for passport in &passports {
        let mut is_valid = true;
        for required in required_fields.iter() {
            if !passport.contains_key(&required.to_string()) {
                is_valid = false;
                continue;
            }
            let value = passport.get(&required.to_string()).unwrap();
            match *required {
                "byr" => {
                    if value.len() != 4 {
                        is_valid = false;
                    } else {
                        let value_num = value.parse::<i32>().unwrap();
                        if !(value_num >= 1920 && value_num <= 2002) {
                            is_valid = false;
                        }
                    }
                }
                "iyr" => {
                    if value.len() != 4 {
                        is_valid = false;
                    } else {
                        let value_num = value.parse::<i32>().unwrap();
                        if !(value_num >= 2010 && value_num <= 2020) {
                            is_valid = false;
                        }
                    }
                }
                "eyr" => {
                    if value.len() != 4 {
                        is_valid = false;
                    } else {
                        let value_num = value.parse::<i32>().unwrap();
                        if !(value_num >= 2020 && value_num <= 2030) {
                            is_valid = false;
                        }
                    }
                }
                "hgt" => {
                    if !hgt_rgx.is_match(value) {
                        is_valid = false;
                    } else {
                        let captured = hgt_rgx.captures(value).unwrap();
                        let height = (
                            captured.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap(),
                            captured.get(2).map_or("", |m| m.as_str()),
                        );

                        match height {
                            (qty, "in") => { is_valid &= qty >= 59 && qty <= 76 }
                            (qty, "cm") => { is_valid &= qty >= 150 && qty <= 193 }
                            _ => is_valid = false,
                        }
                    }
                }
                "hcl" => {
                    if !hcl_rgx.is_match(value) {
                        is_valid = false;
                    }
                }
                "ecl" => {
                    if !valid_eye_colors.contains(value.as_str()) {
                        is_valid = false;
                    }                    
                }
                "pid" => {
                    if !pid_rgx.is_match(value) {
                        is_valid = false;
                    }
                }
                _ => {}
            }
        }

        if is_valid {
            valid_passports += 1;
        }
    }

    println!("Second challenge result: {}", valid_passports);
}
