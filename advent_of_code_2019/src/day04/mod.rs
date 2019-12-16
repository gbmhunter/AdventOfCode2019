pub fn main() {

    let input_min = 234208;
    // let input_min = 444455;
    let input_max = 765869;
    // let input_max = 444456;

    let mut curr_num = input_min;
    let mut nums_that_meet_rules_part_1 = 0;
    let mut nums_that_meet_rules_part_2 = 0;
    while curr_num <= input_max {
        // if curr_num % 1000 == 0 {
        //     println!("{}", curr_num);
        // }
        let digits = number_to_vec(curr_num);
        let mut last_digit: Option<i32> = None;
        let mut two_or_more_digits_seq = false;
        let mut two_digits_seq = false;
        let mut never_decreasing = true;
        let mut num_digits_sequential = 1;
        for digit in digits {
            // println!("Looking at digit = {}", digit);
            if last_digit.is_none() {
                // println!("Looking at first digit.");
                last_digit = Some(digit);
                continue;
            }

            if digit < last_digit.unwrap() {
                never_decreasing = false;
            }

            if digit == last_digit.unwrap() {
                num_digits_sequential += 1;
                two_or_more_digits_seq = true;
            } else {
                if num_digits_sequential == 2 {
                    // println!("Found two sequential digits in number {}", curr_num);
                    two_digits_seq = true;
                }
                num_digits_sequential = 1;
            }
            last_digit = Some(digit);
        }

        if num_digits_sequential == 2 {
            two_digits_seq = true;
        }

        if (never_decreasing == true) && (two_or_more_digits_seq == true) {
            nums_that_meet_rules_part_1 += 1;
        }
        if (never_decreasing == true) && (two_digits_seq == true) {
            // println!("Number meets part 2 rules = {}", curr_num);
            nums_that_meet_rules_part_2 += 1;
        }
        // println!("Num = {}, never_decreasing = {}, two_digits_seq = {}, num_digits_sequential = {}",
        // curr_num, never_decreasing, two_digits_seq, num_digits_sequential);
        curr_num += 1;
    }

    println!("day 04 part 1: number of nums that meet rules = {}", nums_that_meet_rules_part_1);
    println!("day 04 part 2: number of nums that meet rules = {}", nums_that_meet_rules_part_2);
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