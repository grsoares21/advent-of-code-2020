use std::io::{self, BufRead};

pub fn day_3() {
  let mut slope_1_1_trees: i64 = 0;
  let mut slope_3_1_trees: i64 = 0;
  let mut slope_5_1_trees: i64 = 0;
  let mut slope_7_1_trees: i64 = 0;
  let mut slope_1_2_trees: i64 = 0;
  let map_cells: Vec<Vec<char>> = io::stdin()
    .lock()
    .lines()
    .map(|result| result.unwrap().chars().collect())
    .collect();

  for i in 0..map_cells.len() {
    let line_len = map_cells[i].len();

    if map_cells[i][i % line_len] == '#' {
      slope_1_1_trees += 1;
    }

    if map_cells[i][(i * 3) % line_len] == '#' {
      slope_3_1_trees += 1;
    }

    if map_cells[i][(i * 5) % line_len] == '#' {
      slope_5_1_trees += 1;
    }

    if map_cells[i][(i * 7) % line_len] == '#' {
      slope_7_1_trees += 1;
    }


    if i < map_cells.len() / 2 && map_cells[i*2][i % line_len] == '#' {
      slope_1_2_trees += 1;
    }
  }

  println!("First challenge result: {}", slope_3_1_trees);
  println!("Second challenge result: {}", slope_1_1_trees * slope_3_1_trees * slope_5_1_trees * slope_7_1_trees * slope_1_2_trees);
}
