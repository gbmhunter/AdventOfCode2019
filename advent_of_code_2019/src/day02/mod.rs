use std::fs;

pub fn main() {
    let contents = fs::read_to_string("src/day02/input.txt").unwrap();

    let split = contents.split(",");
    let mut memory: Vec<u32> = Vec::new();
    for num_as_string in split {
        memory.push(num_as_string.parse().unwrap());
    }


    let mut memory_copy = memory.clone();
    // Manual replacement as per instructions
    memory_copy[1] = 12;
    memory_copy[2] = 2;
    memory_copy = run_computer(memory_copy);

    println!("day 02 part 1: Value at memory 0 = {}", memory_copy[0]);

    let mut done = false;
    for i in 0..100 {
        // println!("i = {}", i);
        for j in 0..100 {
            memory_copy = memory.clone();
            memory_copy[1] = i;
            memory_copy[2] = j;
            memory_copy = run_computer(memory_copy);
            if memory_copy[0] == 19690720 {
                println!("day 02 part 2: 100*noun + verb = {}", 100*i + j);
                done = true;
            }
            if done {
                break;
            }
        }
        if done {
            break;
        }
    }
}

pub fn run_computer(mut memory: Vec<u32>) -> Vec<u32> {
    let mut curr_pos = 0;
    loop {
        let op_code = memory[curr_pos];
        if op_code == 1 {
            let operand_1 = memory[memory[curr_pos + 1] as usize];
            let operand_2 = memory[memory[curr_pos + 2] as usize];
            let output_idx = memory[curr_pos + 3] as usize;

            let output_value = operand_1 + operand_2;
            memory[output_idx] = output_value;
        } else if op_code == 2 {
            let operand_1 = memory[memory[curr_pos + 1] as usize];
            let operand_2 = memory[memory[curr_pos + 2] as usize];
            let output_idx = memory[curr_pos + 3] as usize;

            let output_value = operand_1 * operand_2;
            memory[output_idx] = output_value;
        } else if op_code == 99 {
            break;
        } else {
            panic!("Unrecognized op code.");
        }
        curr_pos += 4;
    }
    return memory;
}