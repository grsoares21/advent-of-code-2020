use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn color_to_qty(color: String, graph: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let dst_colors = graph.get(&color).unwrap();
    if dst_colors.len() == 0 {
        return 0;
    }

    let mut total = 0;

    for dst_color in dst_colors {
        let qty_for_color = color_to_qty(dst_color.0.clone(), graph);
        total += dst_color.1 + dst_color.1 * qty_for_color;
    }
    return total;
}

pub fn day_7() {
    let color_qty_rgx = Regex::new(r"^([0-9]) (.*)$").unwrap();

    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line_str = line.unwrap();
        let src_dst_string: Vec<String> =
            line_str.split(" contain ").map(|x| x.to_string()).collect();
        let src = src_dst_string[0].clone().replace(" bags", "");
        let dst_string = src_dst_string[1].clone();

        let dst_vec: Vec<String> = dst_string
            .replace("s.", ".")
            .replace("s,", ",")
            .replace(".", "")
            .replace(", ", ",")
            .split(",")
            .map(|x| x.to_string())
            .collect();
        let trimmed_dst_vec: Vec<String> = dst_vec.iter().map(|x| x.replace(" bag", "")).collect();

        let color_qty_vec: Vec<(String, i32)> = match trimmed_dst_vec[0].as_str() {
            "no other" => Vec::new(),
            _ => trimmed_dst_vec
                .iter()
                .map(|dst| {
                    let captured = color_qty_rgx.captures(dst).unwrap();
                    let color_qty = (
                        captured.get(2).map_or("", |m| m.as_str()).to_string(),
                        captured
                            .get(1)
                            .map_or("", |m| m.as_str())
                            .parse::<i32>()
                            .unwrap(),
                    );
                    color_qty
                })
                .collect(),
        };

        graph.insert(src, color_qty_vec);
    }

    let mut total = 0;

    for color in &graph {
        let mut queue: Vec<&String> = Vec::new();

        for new_color in color.1.iter() {
            queue.push(&new_color.0)
        }

        while queue.len() > 0 {
            let visiting_color = queue.pop().unwrap();
            if visiting_color == "shiny gold" {
                total += 1;
                break;
            }

            let new_colors = &graph.get(visiting_color).unwrap();

            for color in new_colors.iter() {
                queue.push(&color.0)
            }
        }
    }

    println!("First challenge result: {}", total);
    println!(
        "Second challenge result: {}",
        color_to_qty("shiny gold".to_string(), &graph)
    );
}
