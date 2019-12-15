use std::io::{self};

mod day01;
mod day02;
mod day03;

fn main() -> io::Result<()> {
    day01::main();
    day02::main();
    day03::main();
    Ok(())

}
