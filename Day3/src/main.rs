/*
 * Derek Prince
 * Advent of Code Day 3
 * Manhattan Rats Nest
*/

use std::env;
use std::fs;
use std::io::{Read, Error};
use crate::manhattan_grid::{Manratty, Instruction};

mod manhattan_grid;

fn read_csv<R: Read>(io : R) -> Result<Manratty, Error> {
    let mut csv_rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io);

    let mut mr : Manratty = Manratty::default();

    let mut row : usize = 0;
    let mut i1 : Vec<Instruction> = vec![];
    let mut i2 : Vec<Instruction> = vec![];

    for result in csv_rdr.records() {
        let record = result?;
        // Capture rows in separate instruction vectors
        // println!("Record len: {}", record.len());
        for i in 0..record.len() {
            if row == 0 {
                let wl = Instruction::new(record[i].parse().unwrap());
                // println!("Instruction | Direction: {}, Distance: {}", wl.direction, wl.distance);
                i1.push(wl);

            }
            else {
                let wl = Instruction::new(record[i].parse().unwrap());
                // println!("Instruction | Direction: {}, Distance: {}", wl.direction, wl.distance);
                i2.push(wl);
            }
        }
        row = row + 1;
    }
    mr.store_instructions(i1, i2);
    // okay mister
    Ok(mr)
}

fn main() -> Result<(), Error> {
    let args : Vec<String> = env::args().collect();
    let input = &args[1];

    let mut mr = read_csv(fs::File::open(input)?)?;
    mr.plot_wires();
    let s = mr.find_intersect();
    println!("Shortest distance: {}", s[0]+s[1]);


    Ok(())
}
