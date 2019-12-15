use std::fs;
use std::collections::HashMap;

pub fn main() {
    let input = fs::read_to_string("src/day03/input.txt").unwrap();
    let input_by_line: Vec<&str> = input.split("\n").collect();
   
    let line_positions_1 = parse_line(input_by_line[0]);
    let line_positions_2 = parse_line(input_by_line[1]);

    let mut lowest_manhatten_distance: Option<i32> = None;
    let mut lowest_key: Option<(i32, i32)> = None;
    for (key, value) in &line_positions_1 {
        let same_pos = line_positions_2.get(key);
        if let Some(same_pos) = same_pos {
            // println!("Found the same position in line 1 and 2. pos = {:?}", key);
            let manhatten_distance = key.0.abs() + key.1.abs();
            // println!("Manhatten distance = {}", manhatten_distance);
            if lowest_manhatten_distance.is_none() {
                // println!("Setting new winner.");
                lowest_manhatten_distance = Some(manhatten_distance);
                lowest_key = Some(*key);
            } else if manhatten_distance < lowest_manhatten_distance.unwrap() {
                // println!("Setting new winner.");
                lowest_manhatten_distance = Some(manhatten_distance);
                lowest_key = Some(*key);
            }
        }
    }

    println!("day 03 part 1: manhatten distance = {:?}", lowest_manhatten_distance.unwrap());

}

fn parse_line(line_cmds_as_str: &str) -> HashMap<(i32, i32),bool> {
    // println!("parse_line() called with line_cmds_as_str = {}", line_cmds_as_str);

    let line_cmds = line_cmds_as_str.split(",");
    let mut line_positions = HashMap::new();

    let mut curr_pos = (0, 0);
    for line_cmd in line_cmds {
        // println!("Parsing cmd = {}. curr_pos = {:?}", line_cmd, curr_pos);
        let num_units: u32 = line_cmd[1..].parse().unwrap();
        // println!("Num units = {}", num_units);
        if line_cmd.chars().nth(0).unwrap() == 'U' {
            // println!("Going up.");
            for i in 0..num_units {
                curr_pos = (curr_pos.0, curr_pos.1 + 1);
                line_positions.insert(curr_pos, true);
            }
        } else if line_cmd.chars().nth(0).unwrap() == 'D' {
            // println!("Going down.");
            for i in 0..num_units {
                curr_pos = (curr_pos.0, curr_pos.1 - 1);
                line_positions.insert(curr_pos, true);
            }
        } else if line_cmd.chars().nth(0).unwrap() == 'L' {
            // println!("Going left.");
            for i in 0..num_units {
                curr_pos = (curr_pos.0 - 1, curr_pos.1);
                line_positions.insert(curr_pos, true);
            }
        } else if line_cmd.chars().nth(0).unwrap() == 'R' {
            // println!("Going right.");
            for i in 0..num_units {
                curr_pos = (curr_pos.0 + 1, curr_pos.1);
                line_positions.insert(curr_pos, true);
            }
        } else {
            panic!("Got unrecognized command.");            
        }
    }
    return line_positions;

}