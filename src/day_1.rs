use std::io::{self, BufRead};

pub fn day_1() {
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
