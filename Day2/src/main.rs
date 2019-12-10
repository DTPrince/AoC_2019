/*
 * Derek Prince
 * Advent of Code Day 2
 */

use std::error::Error;
use std::io;

// R must implement Read
fn read_fcsv<R: Read>(io : R) -> Result<mut Vec<u32>, Error> {
    let mut csv_rdr = csv::Reader::from_reader(io);
    let mut csv_vec = Vec<u32>;
    for result in csv_rdr.records() {
        let item = result?;

    }
    Ok(csv_vec)

}

fn main() {
    println!("Hello, world!");
}
