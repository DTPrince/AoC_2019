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

    let row : usize = 0;
    let mut i1 : Vec<Instruction> = vec![Instruction::default()];
    let mut i2 : Vec<Instruction> = vec![Instruction::default()];

    for result in csv_rdr.records() {
        let record = result?;
        // Capture rows in separate instruction vectors
        for i in 0..record.len() {
            if row == 0 {
                i1.push(Instruction::new(record[i].parse().unwrap()));
            }
            else {
                i2.push(Instruction::new(record[i].parse().unwrap()));
            }
        }
    }

    mr.store_instructions(&i1, &i2);
    // okay mister
    Ok(mr)
}

fn main() -> Result<(), Error> {
    let args : Vec<String> = env::args().collect();
    let input = &args[1];

    let mr = read_csv(fs::File::open(input)?)?;

    Ok(())
}
