/*
 * Derek Prince
 * Advent of Code Day 3
 * Manhattan Rats Nest
*/

use std::env;
use std::fs;
use std::io::{Read, Error};
use crate::manhattan_grid::{Manratty, Instruction, WireLine};

mod manhattan_grid;

fn read_csv<R: Read>(io : R) -> Result<Manratty, Error> {
    let mut csv_rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io);

    let mut mr : Manratty = Manratty::default();

    let row : usize = 0;
    let mut i1 : Vec<WireLine> = vec![WireLine::default()];
    let mut i2 : Vec<WireLine> = vec![WireLine::default()];

    for result in csv_rdr.records() {
        let record = result?;
        // Capture rows in separate instruction vectors
        for i in 0..record.len() {
            if row == 0 {
                let wl = WireLine::new(Instruction::new(record[i].parse().unwrap()));
                mr.store_raw_instruction_w1(wl);
//                i1.push(wl);
            }
            else {
                let wl = WireLine::new(Instruction::new(record[i].parse().unwrap()));
                mr.store_raw_instruction_w2(wl);

//                i2.push(WireLine::new(Instruction::new(record[i].parse().unwrap())));
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
