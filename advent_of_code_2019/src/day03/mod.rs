use std::fs;
use std::collections::HashMap;

pub fn main() {
    let input = fs::read_to_string("src/day03/input.txt").unwrap();
    let input_by_line: Vec<&str> = input.split("\n").collect();
   
    let line_positions_1 = parse_line(input_by_line[0]);
    let line_positions_2 = parse_line(input_by_line[1]);

    let mut lowest_manhatten_distance: Option<i32> = None;
    let mut lowest_key: Option<(i32, i32)> = None;
    let mut lowest_combined_steps: Option<i32> = None;
    for (key, value) in &line_positions_1 {
        let same_pos = line_positions_2.get(key);
        if let Some(same_pos) = same_pos {
            // println!("Found the same position in line 1 and 2. pos = {:?}", key);
            let manhatten_distance = key.0.abs() + key.1.abs();
            let combined_steps = value + same_pos;

            // println!("Manhatten distance = {}, combined_steps = {}", manhatten_distance, combined_steps);
            if lowest_manhatten_distance.is_none() {
                // println!("Setting new winner.");
                lowest_manhatten_distance = Some(manhatten_distance);
                lowest_key = Some(*key);
            } else if manhatten_distance < lowest_manhatten_distance.unwrap() {
                // println!("Setting new winner.");
                lowest_manhatten_distance = Some(manhatten_distance);
                lowest_key = Some(*key);
            }

            if lowest_combined_steps.is_none() {
                lowest_combined_steps = Some(combined_steps);
            } else if combined_steps < lowest_combined_steps.unwrap() {
                lowest_combined_steps = Some(combined_steps);
            }
        }
    }

    println!("day 03 part 1: manhatten distance = {:?}", lowest_manhatten_distance.unwrap());
    println!("day 03 part 2: lowest combined steps = {:?}", lowest_combined_steps.unwrap());

}

fn parse_line(line_cmds_as_str: &str) -> HashMap<(i32, i32), i32> {
    // println!("parse_line() called with line_cmds_as_str = {}", line_cmds_as_str);

    let line_cmds = line_cmds_as_str.split(",");
    let mut line_positions: HashMap<(i32, i32), i32> = HashMap::new();

    let mut curr_pos = (0, 0);
    let mut num_steps = 0;
    for line_cmd in line_cmds {
        // println!("Parsing cmd = {}. curr_pos = {:?}", line_cmd, curr_pos);
        let cmd = line_cmd.chars().nth(0).unwrap();
        let num_units: u32 = line_cmd[1..].parse().unwrap();
        for i in 0..num_units {
            num_steps += 1;

            // Update current position based on command direction
            if cmd == 'U' {
            // println!("Going up.");
                curr_pos = (curr_pos.0, curr_pos.1 + 1);
            } else if cmd == 'D' {
                curr_pos = (curr_pos.0, curr_pos.1 - 1);
            } else if line_cmd.chars().nth(0).unwrap() == 'L' {
                curr_pos = (curr_pos.0 - 1, curr_pos.1);
            } else if line_cmd.chars().nth(0).unwrap() == 'R' {
                curr_pos = (curr_pos.0 + 1, curr_pos.1);
            } else {
                panic!("Got unrecognized command.");            
            }

            if line_positions.contains_key(&curr_pos) {
                // Map already contains entry at this position
                // let value = line_positions.get(&curr_pos).unwrap();
                // println!("Not updating this position on grid, number steps will be higher.");
            } else {
                // Map position doesn't have entry yet
                line_positions.insert(curr_pos, num_steps);
            }
        }
    }
    return line_positions;

}
