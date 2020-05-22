use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Advent of Code Day 1
fn main() {
    if let Ok(lines) = read_lines("data/day1.input") {
        let mut total_fuel: u32 = 0;
        for line in lines {
            // process each line
            if let Ok(l) = line {
                let module_mass: u32 = l.parse().unwrap();
                let module_fuel = (module_mass / 3) - 2;
                total_fuel += module_fuel;
            }
        }
        println!("{}", total_fuel);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
