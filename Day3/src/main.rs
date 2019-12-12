/*
 * Derek Prince
 * Advent of Code Day 3
 * Manhattan Rats Nest
*/

use std::env;
use std::fs;
use std::io::{Read, Error};

fn read_csv<R: Read>(io : R, mut csv_vec : Vec<u32>) -> Result<Vec<u32>, Error> {
    let mut csv_rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io);

    for record in csv_rdr.records() {

        let result = record?;
        println!("{:?}", result);
        for i in 0..result.len() {
            csv_vec.push(result[i].parse::<u32>().unwrap());
        }

    }
    Ok(csv_vec)
}

fn main() -> Result<(), Error> {
    let args : Vec<String> = env::args().collect();
    let input = &args[1];

    let mut inputs : Vec<u32> = Vec::new();
    inputs = read_csv(fs::File::open(input)?, inputs)?;

    Ok(())
}
