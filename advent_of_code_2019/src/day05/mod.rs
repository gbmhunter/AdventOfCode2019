use std::fs;

pub fn main() {

    let contents = fs::read_to_string("src/day05/input.txt").unwrap();

    let split = contents.split(",");
    let mut memory: Vec<i32> = Vec::new();
    for num_as_string in split {
        memory.push(num_as_string.parse().unwrap());
    }

    let mut memory_copy = memory.clone();
    let ret_vals = run_computer(memory_copy);
    println!("day 05 part 1: diagnostic code = {}", ret_vals.1[ret_vals.1.len() - 1]);
}

fn run_computer(mut memory: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // println!("run_computer() called.");
    let mut curr_pos: usize = 0;
    let mut outputs: Vec<i32> = Vec::new();
    while true {
        let op_code_and_parameter_mode = memory[curr_pos];
        // println!("op_code_and_parameter_mode = {}", op_code_and_parameter_mode);
        let mut op_code_and_parameters_mode_vec = number_to_vec(op_code_and_parameter_mode as i32);
        op_code_and_parameters_mode_vec.reverse();

        let op_code: i32;
        if op_code_and_parameters_mode_vec.len() == 1 {
            op_code = op_code_and_parameters_mode_vec[0];
        } else {
            op_code = op_code_and_parameters_mode_vec[1]*10 + op_code_and_parameters_mode_vec[0];
        }
        // println!("Got op_code = {}", op_code);

        let output_value: i32;
        if op_code == 1 {
            // println!("Got addition opcode.");
            let output_idx = memory[curr_pos + 3] as usize;
            // println!("output_idx = {}", output_idx);
            let operands = parse_2_params(&memory, curr_pos);
            output_value = operands.0 + operands.1;
            memory[output_idx] = output_value;
            curr_pos += 4;
        } else if op_code == 2 {
            let output_idx = memory[curr_pos + 3] as usize;
            let operands = parse_2_params(&memory, curr_pos);
            output_value = operands.0 * operands.1;
            memory[output_idx] = output_value;
            curr_pos += 4;
        } else if op_code == 3 {
            // Take input
            // println!("Got input opcode.");
            let input = 1;
            let output_idx = memory[curr_pos + 1] as usize;
            memory[output_idx] = input;
            curr_pos += 2;
        } else if op_code == 4 {
            let output = memory[memory[curr_pos + 1] as usize];
            // println!("Printing output: {}", output);
            outputs.push(output);
            curr_pos += 2;
        } else if op_code == 99 {
            // println!("Halting.");
            break;
        } else {
            panic!("Unrecognized op code.");
        }
    }
    return (memory, outputs);
}

fn number_to_vec(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn parse_2_params(memory: &Vec<i32>, curr_pos: usize) -> (i32, i32) {
    let op_code_and_parameter_mode = memory[curr_pos];
    let mut op_code_and_parameters_mode_vec = number_to_vec(op_code_and_parameter_mode as i32);
    op_code_and_parameters_mode_vec.reverse();

    let mut param_1_mode = 0;
    if op_code_and_parameters_mode_vec.len() >= 3 {
        param_1_mode = op_code_and_parameters_mode_vec[2];
    }

    let mut param_2_mode = 0;
    if op_code_and_parameters_mode_vec.len() >= 4 {
        param_2_mode = op_code_and_parameters_mode_vec[3];
    }

    let mut param_3_mode = 0;
    if op_code_and_parameters_mode_vec.len() >= 5 {
        param_3_mode = op_code_and_parameters_mode_vec[4];
    }

    let operand_1: i32;
    if param_1_mode == 0 {
        operand_1 = memory[memory[curr_pos + 1] as usize];
    } else if param_1_mode == 1 {
        operand_1 = memory[curr_pos + 1];
    } else {
        panic!("param_1_mode not supported.");
    }

    let operand_2: i32;
    if param_2_mode == 0 {
        operand_2 = memory[memory[curr_pos + 2] as usize];
    } else if param_2_mode == 1 {
        operand_2 = memory[curr_pos + 2];
    } else {
        panic!("param_2_mode not supported. param_2_mode = {}", param_2_mode);
    }

    if param_3_mode != 0 {
        panic!("param_3_mode not supported.")
    }

    (operand_1, operand_2)

}