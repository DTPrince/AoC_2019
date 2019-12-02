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

fn fuel_required(fuel_mass: &u32) -> u32 {
    // Here we check on 6 because it will result in 0 fuel needed.
    // Also Rust shits a brick over attempting to subtract with overflow otherwise. Which is good.
    if fuel_mass < &6 {
        return 0
    }
    let mut mass : u32 = (fuel_mass / 3) - 2;
//    println!("Fuel Mass: {}, Mass for mass: {}", fuel_mass, mass);
    // Accumulate on returns
    mass += fuel_required(&mass);
    mass
}

fn main() -> Result<(), Error> {
    // Gather input arguments (input file)
    let args: Vec<String> = env::args().collect();
    let input = &args[2];

    println!("Reading contents from {}", input);
    let packages = read(fs::File::open(input)?)?;

    // The file read function inspired me to try a super simple closure
    // This was the old, non-recursive fuel calculation function.
    // Once it went recursive a closure was no longer the right way (I believe anyway)
    let f_req = |pkg_mass: &u32| ((pkg_mass / 3) - 2) as u32;

    // accumulate results
    let mut package_fuel_req : u32 = 0;
    for package in &packages {
        package_fuel_req += fuel_required(package);
    }
    println!("Required fuel for packages and extra fuel: {}", package_fuel_req);

    Ok(())
}