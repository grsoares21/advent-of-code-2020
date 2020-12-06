use std::io::{self, BufRead};

fn front_back_binary(min_row: i32, max_row: i32, dir_array: Vec<char>) -> i32 {
    if min_row == max_row {
        return min_row;
    }
    let (head, tail) = dir_array.split_at(1);

    let difference = max_row - min_row;

    return match head[0] {
        'F' => front_back_binary(min_row, min_row + difference / 2, tail.to_vec()),
        'B' => front_back_binary(min_row + difference / 2 + 1, max_row, tail.to_vec()),
        _ => panic!("Something's not right in your logic mate"),
    };
}

fn right_left_binary(min_col: i32, max_col: i32, dir_array: Vec<char>) -> i32 {
    if min_col == max_col {
        return min_col;
    }
    let (head, tail) = dir_array.split_at(1);

    let difference = max_col - min_col;

    return match head[0] {
        'L' => right_left_binary(min_col, min_col + difference / 2, tail.to_vec()),
        'R' => right_left_binary(min_col + difference / 2 + 1, max_col, tail.to_vec()),
        _ => panic!("Something's not right in your logic mate"),
    };
}

pub fn day_5() {
    let mut tickets: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lock().lines() {
        tickets.push(line.unwrap().chars().collect());
    }

    let mut ticket_ids: Vec<i32> = Vec::new();
    let mut max_ticket_id = 0;
    for ticket in tickets {
        let (row_directions, col_directions) = ticket.split_at(7);
        let row = front_back_binary(0, 127, row_directions.to_vec());
        let col = right_left_binary(0, 7, col_directions.to_vec());

        let ticket_id = row * 8 + col;
        if ticket_id > max_ticket_id {
            max_ticket_id = ticket_id;
        }

        ticket_ids.push(ticket_id);
    }

    println!("First challenge result: {}", max_ticket_id);

    ticket_ids.sort();

    for i in 0..ticket_ids.len() - 1 {
        if ticket_ids[i + 1] - ticket_ids[i] > 1 {
            println!("Second challenge result: {}", ticket_ids[i] + 1);
        }
    }
}
