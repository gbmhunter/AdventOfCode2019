use std::io::{self};

mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> io::Result<()> {
    day01::main();
    day02::main();
    day03::main();
    day04::main();
    Ok(())

}
