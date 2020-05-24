use std::fs;
use std::io;
use std::vec::Vec;
use std::path::Path;

fn main() {
    if let Ok(mut program) = read_program("data/day2.input") {
        // Transform the program first
        program[1] = 12;
        program[2] = 2;
        // Then execute
        exec(&mut program);
        // Get the value at position 0
        println!("{}", program[0]);
    }
}

fn exec(program: &mut Vec<u32>) {
    // Program counter
    let mut pc: usize = 0;
    // Instruction is at program[pc]. Avoid running off the end!
    while pc < program.len() {
        // Halt if we see opcode 99
        if program[pc] == 99 {
            return;
        } else {
            // Read inputs
            let (x1, x2) = (program[program[pc + 1] as usize], program[program[pc + 2] as usize]);
            // Get location of output. Have to do this before to avoid borrowing program mutably twice.
            let result_loc = program[pc + 3] as usize;
            // Perform operation and write output
            if program[pc] == 1 {
                program[result_loc] = x1 + x2;
            } else if program[pc] == 2 {
                program[result_loc] = x1 * x2;
            }
            pc += 4;
        }
    }
}

fn read_program<P>(filename: P) -> io::Result<Vec<u32>>
where P: AsRef<Path>, {
    match fs::read_to_string(filename) {
        Ok(contents) => Ok(contents.as_str().trim()
                                            .split(",")
                                            .map(|s| s.parse::<u32>())
                                            .filter_map(Result::ok)
                                            .collect()),
        Err(e) => Err(e)
    }
}
