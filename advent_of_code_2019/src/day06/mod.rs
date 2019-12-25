use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Planet<'a> {
    name: String,
    orbits: Option<&'a mut Self>,
    orbiting_children: Vec<&'a mut Self>,
}


pub fn main() {
    let filename = "src/day06/input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut name_to_planet_map: HashMap<String, Planet> = HashMap::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, &line);
        let parts: Vec<&str> = line.split(")").collect();
        let central_planet_name = parts[0].to_string();
        let orbiting_planet_name = parts[1].to_string();

        // Check if central planet already exists
        if !name_to_planet_map.contains_key(&central_planet_name) {
            println!("Planet '{}' does not exist, creating...", &central_planet_name);
            let mut planet = Planet {
                name: central_planet_name.clone(),
                orbits: None,
                orbiting_children: Vec::new(),
            };
            name_to_planet_map.insert(planet.name.clone(), planet);
        }
        // Check if orbiting already exists
        if !name_to_planet_map.contains_key(&orbiting_planet_name) {
            println!("Planet '{}' does not exist, creating...", &orbiting_planet_name);
            let mut planet = Planet {
                name: orbiting_planet_name.clone(),
                orbits: None,
                orbiting_children: Vec::new(),
            };
            name_to_planet_map.insert(planet.name.clone(), planet);
        }

        // Add link from parent planet
        let mut central_planet = name_to_planet_map.get_mut(&central_planet_name).unwrap();
        let mut orbiting_planet = name_to_planet_map.get_mut(&orbiting_planet_name).unwrap();
        // central_planet.orbiting_children.push(orbiting_planet);

    }
}