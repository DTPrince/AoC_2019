/* Derek Prince
 * 12/1/19
 * Advent of code Day 1
 * Calculating fuel mass required to launch 100 packages
 */

// I have not used Rust since v0.8 or something so I will likely need liberal use of forums to solve
// basic problems.
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

// Source from https://users.rust-lang.org/t/reading-integers-from-a-file-into-vector/17517/2
fn read<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn fuel_required(pkg_mass: &u32) -> u32 {
    let fuel_req = (pkg_mass / 3) - 2;
    fuel_req
}

fn main() -> Result<(), Error> {
    // Gather input arguments (input file)
    let args: Vec<String> = env::args().collect();
    let input = &args[2];

//    let path = env::current_dir().unwrap();
//    println!("PWD: {}", path.display());
//    println!("Input: {}", input);

    println!("Reading contents from {}", input);
    let packages = read(fs::File::open(input)?)?;

    let mut req_weight : u32 = 0;
    for package in &packages {
        req_weight += fuel_required(package);
    }

    println!("Required fuel mass: {}", req_weight);
    Ok(())
}