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
    let mut fuel_sum_with_fuel: i32 = 0;
    for module_mass in module_masses {
        let fuel_for_module = (module_mass/3) - 2;
        fuel_sum += fuel_for_module;
        let extra_fuel = calc_extra_fuel(fuel_for_module as i32);
        fuel_sum_with_fuel += (fuel_for_module as i32) + extra_fuel;
    }

    println!("part 1: fuel_sum = {}", fuel_sum);

    println!("part 2: fuel_sum_with_fuel = {}", fuel_sum_with_fuel);


    Ok(())
}

fn calc_extra_fuel(fuel_weight: i32) -> i32 {
    // println!("calc_extra_fuel() called with fuel_weight={}", fuel_weight);
    let mut extra_fuel: i32 = (fuel_weight)/3 - 2;
    if extra_fuel > 0 {
        extra_fuel += calc_extra_fuel(extra_fuel);
    } else {
        extra_fuel = 0
    }
    // println!("total extra fuel={}", extra_fuel);
    extra_fuel
}
