use std::io::{self, Read};
use std::cmp;


fn get_occupied_seats(seat_map: &Vec<Vec<char>>) -> i32 {
    let mut occupied_seats = 0;
    
    for y in 0..seat_map.len() {
        for x in 0..seat_map[y].len() {
            if seat_map[y][x] == '#' {
                occupied_seats += 1;
            }
        }
    }
    
    occupied_seats
}

fn first_challenge(input: &String) {
    let mut seat_map: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut temp_seat_map: Vec<Vec<char>> = seat_map
        .iter()
        .map(|chars| chars.clone())
        .collect();

    loop {
        let mut map_changed = false;
        for y in 0..seat_map.len() {
            for x in 0..seat_map[y].len() {
                temp_seat_map[y][x] = seat_map[y][x];

                if seat_map[y][x] == '.' { continue; }

                let mut adjacent_occupied_seats = 0;

                for adjacent_y in cmp::max(0, y as i32 - 1)..cmp::min(y as i32 + 2, seat_map.len() as i32) {
                    for adjacent_x in cmp::max(0, x as i32 - 1)..cmp::min(x as i32 + 2, seat_map[y].len() as i32) {
                        if !(adjacent_y as usize == y && adjacent_x as usize == x) && seat_map[adjacent_y as usize][adjacent_x as usize] == '#' {
                            adjacent_occupied_seats += 1;
                        }
                    }
                }

                if seat_map[y][x] == 'L' && adjacent_occupied_seats == 0 {
                    map_changed = true;
                    temp_seat_map[y][x] = '#';
                } else if seat_map[y][x] == '#' && adjacent_occupied_seats >= 4 {
                    map_changed = true;
                    temp_seat_map[y][x] = 'L';
                }
            }
        }

        if !map_changed { break; }
        seat_map = temp_seat_map
            .iter()
            .map(|chars| chars.clone())
            .collect();        
    }

    println!("First challenge result: {}", get_occupied_seats(&seat_map));
}

fn second_challenge(input: &String) {
    let mut seat_map: Vec<Vec<char>> = input
    .split("\n")
    .map(|line| line.chars().collect())
    .collect();

let mut temp_seat_map: Vec<Vec<char>> = seat_map
    .iter()
    .map(|chars| chars.clone())
    .collect();

    loop {
        let mut map_changed = false;
       
        for y in 0..seat_map.len() {
            for x in 0..seat_map[y].len() {
                temp_seat_map[y][x] = seat_map[y][x];

                if seat_map[y][x] == '.' { continue; }

                let mut adjacent_occupied_seats = 0;

                // TOP LEFT direction
                {
                    let mut loop_x = x;
                    let mut loop_y = y;
                    loop {
                        if loop_x == 0 || loop_y == 0 { break; }
                        loop_x -= 1;
                        loop_y -= 1;
                        if seat_map[loop_y][loop_x] == 'L' { break; }
                        if seat_map[loop_y][loop_x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                // TOP direction
                {
                    let mut loop_y = y;
                    loop {
                        if loop_y == 0 { break; }
                        loop_y -= 1;
                        if seat_map[loop_y][x] == 'L' { break; }
                        if seat_map[loop_y][x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                // TOP RIGHT direction
                {
                    let mut loop_x = x;
                    let mut loop_y = y;
                    loop {
                        if loop_x == seat_map[y].len() - 1 || loop_y == 0 { break; }
                        loop_x += 1;
                        loop_y -= 1;
                        if seat_map[loop_y][loop_x] == 'L' { break; }
                        if seat_map[loop_y][loop_x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                // RIGHT direction
                {
                    let mut loop_x = x;
                    loop {
                        if loop_x == seat_map[y].len() - 1 { break; }
                        loop_x += 1;
                        if seat_map[y][loop_x] == 'L' { break; }
                        if seat_map[y][loop_x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                // RIGHT BOTTOM direction
                {
                    let mut loop_x = x;
                    let mut loop_y = y;
                    loop {
                        if loop_x == seat_map[y].len() - 1 || loop_y == seat_map.len() - 1 { break; }
                        loop_x += 1;
                        loop_y += 1;
                        if seat_map[loop_y][loop_x] == 'L' { break; }
                        if seat_map[loop_y][loop_x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                // BOTTOM direction
                {
                    let mut loop_y = y;
                    loop {
                        if loop_y == seat_map.len() - 1 { break; }
                        loop_y += 1;
                        if seat_map[loop_y][x] == 'L' { break; }
                        if seat_map[loop_y][x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                // LEFT BOTTOM direction
                {
                    let mut loop_x = x;
                    let mut loop_y = y;
                    loop {
                        if loop_x == 0 || loop_y == seat_map.len() - 1 { break; }
                        loop_x -= 1;
                        loop_y += 1;
                        if seat_map[loop_y][loop_x] == 'L' { break; }
                        if seat_map[loop_y][loop_x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                // LEFT direction
                {
                    let mut loop_x = x;
                    loop {
                        if loop_x == 0 { break; }
                        loop_x -= 1;
                        if seat_map[y][loop_x] == 'L' { break; }
                        if seat_map[y][loop_x] == '#' {
                            adjacent_occupied_seats += 1;
                            break;
                        }
                    }
                }

                if seat_map[y][x] == 'L' && adjacent_occupied_seats == 0 {
                    map_changed = true;
                    temp_seat_map[y][x] = '#';
                } else if seat_map[y][x] == '#' && adjacent_occupied_seats >= 5 {
                    map_changed = true;
                    temp_seat_map[y][x] = 'L';
                }
            }
        }

        if !map_changed { break; }
        seat_map = temp_seat_map
            .iter()
            .map(|chars| chars.clone())
            .collect();
    }

    println!("Second challenge result: {}", get_occupied_seats(&seat_map));

}

pub fn day_11() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    first_challenge(&input);
    second_challenge(&input);
}
