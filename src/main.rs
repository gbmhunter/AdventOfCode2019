use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut module_masses: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        module_masses.push(line.parse().unwrap());
    }

    let mut fuel_sum = 0;
    for module_mass in module_masses {
        fuel_sum += (module_mass/3) - 2;
    }

    println!("fuel_sum = {}", fuel_sum);
    Ok(())
}
