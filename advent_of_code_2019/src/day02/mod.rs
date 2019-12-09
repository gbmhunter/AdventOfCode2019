use std::fs;

pub fn main() {
    let contents = fs::read_to_string("src/day02/input.txt").unwrap();

    let split = contents.split(",");
    let mut op_codes: Vec<u32> = Vec::new();
    for num_as_string in split {
        op_codes.push(num_as_string.parse().unwrap());
    }

    // Manual replacement as per instructions
    op_codes[1] = 12;
    op_codes[2] = 2;

    let mut curr_pos = 0;
    while true {
        let op_code = op_codes[curr_pos];
        if op_code == 1 {
            let operand_1 = op_codes[op_codes[curr_pos + 1] as usize];
            let operand_2 = op_codes[op_codes[curr_pos + 2] as usize];
            let output_idx = op_codes[curr_pos + 3] as usize;

            let output_value = operand_1 + operand_2;
            op_codes[output_idx] = output_value;
        } else if op_code == 2 {
            let operand_1 = op_codes[op_codes[curr_pos + 1] as usize];
            let operand_2 = op_codes[op_codes[curr_pos + 2] as usize];
            let output_idx = op_codes[curr_pos + 3] as usize;

            let output_value = operand_1 * operand_2;
            op_codes[output_idx] = output_value;
        } else if op_code == 99 {
            break;
        } else {
            println!("ERROR!")
        }
        curr_pos += 4;
    }


    println!("day 02 part 1: Value at op code 0 = {}", op_codes[0]);
}