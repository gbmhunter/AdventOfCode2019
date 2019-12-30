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

    let mut child_to_parent_map: HashMap<String, String> = HashMap::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        // println!("{}. {}", index + 1, &line);
        let parts: Vec<&str> = line.split(")").collect();
        let central_planet_name = parts[0].to_string();
        let orbiting_planet_name = parts[1].to_string();

        child_to_parent_map.insert(orbiting_planet_name, central_planet_name);      

    }

    let mut count = 0;
    for mut child in child_to_parent_map.keys() {
        // println!("child = {}", child);
        while let Some(parent) = child_to_parent_map.get(child) {
            // println!("parent = {}", parent);
            child = parent;
            count += 1;
        }
    }
    println!("day 06 part 1: count = {}", count);

    let mut santa_parents = find_parents(&child_to_parent_map, String::from("SAN"));
    let mut you_parents = find_parents(&child_to_parent_map, String::from("YOU"));

    santa_parents.reverse();
    you_parents.reverse();

    let mut i = 0;
    loop {
        if santa_parents[i] != you_parents[i] {
            break;
        }
        i += 1;
    }
    println!("shared_parent_count = {}", i);
}

pub fn find_parents(child_to_parent_map: &HashMap<String, String>, mut name: String) -> Vec<String> {
    let mut parents: Vec<String> = Vec::new();
    while let Some(parent) = child_to_parent_map.get(&name) {
        // println!("parent = {}", parent);
        parents.push(parent.clone());
        name = parent.clone();
    }
    return parents;
}