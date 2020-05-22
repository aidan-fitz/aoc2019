use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// # Advent of Code Day 1
/// [Problem statement](https://adventofcode.com/2019/day/1)
///
/// Our goal is to calculate the total fuel that our spaceship needs. Each module has a given
/// mass `m`, and the amount of fuel needed to launch that module is given by
/// `f = floor(m / 3) - 2`.
///
/// Boilerplate code from [Rust by Example: `read_lines`](https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html)
fn main() {
    if let Ok(lines) = read_lines("data/day1.input") {
        let mut total_fuel: u32 = 0;
        for line in lines {
            // process each line
            if let Ok(l) = line {
                // Each line is the mass of a module. Since all quantities are non-negative integers,
                // we use the unsigned type `u32`.
                let module_mass: u32 = l.parse().unwrap();
                // Integer division automatically returns the floor of the result.
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
