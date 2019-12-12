/*
 * Derek Prince
 * Advent of Code Day 2
 */

use std::env;
use std::fs;
use std::io::{Read, Error};

// R must implement Read
fn read_fcsv<R: Read>(io : R, mut csv_vec : Vec<u32>) -> Result<Vec<u32>, Error> {
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

fn run_intcodes(mut intcodes : Vec<u32>) -> Result<u32, Error> {
    const ADDITION : u32 = 1;
    const MULTIPLICATION : u32 = 2;
    const EXIT : u32 = 99;

    let mut operation = 0;
    let mut input1 = 0;
    let mut input2 = 0;
    let mut output = 0;

    // Iterate over the whole thing...
    let mut i = 0;
    while i < (intcodes.len() - 1) {
        operation = intcodes[i];
        input1 = intcodes[intcodes[i + 1] as usize];
        input2 = intcodes[intcodes[i + 2] as usize];
        output = intcodes[i + 3] as usize;

        match operation {
            ADDITION => {
//                println!("Adding: {}: {} + {}: {}, storing in {}", i+1, input1, i+2, input2, output);
                intcodes[output as usize] = input1 + input2
            },
            MULTIPLICATION => {
//                println!("Multiply: {}: {} * {}: {}, storing in {}", i+1, input1, i+2, input2, output);
                intcodes[output as usize] = input1 * input2
            },
            EXIT => {
                println!("Exiting on {}.", i);
                return Ok(intcodes[0])
            },
            _ => { println!("I dont have any idea how you got here. Operation: {}", operation); }
        }
        i += 4;
    }

    Ok(intcodes[0])
}

// Part tuwu
fn find_verb_noun(intcodes : Vec<u32>) -> Result<Vec<u32>, Error> {
    const KEYVAL : u32 = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut vec_copy : Vec<u32> = intcodes.to_vec();
            vec_copy[1] = noun;
            vec_copy[2] = verb;
            let result = run_intcodes(vec_copy)?;
            if result == KEYVAL {
                // if result is key value fn is searching for, store noun/verb pair and exit
                let nv_pair : Vec<u32> = vec![noun, verb];
                return Ok(nv_pair);
            }
        }
    }

    let nv_pair : Vec<u32> = vec![0];
    Ok(nv_pair)
}

fn main() -> Result<(), Error> {
    let args : Vec<String> = env::args().collect();
    let input = &args[1];

    let mut opcodes : Vec<u32> = Vec::new();
    opcodes = read_fcsv(fs::File::open(input)?, opcodes)?;

    let result = run_intcodes(opcodes.to_vec())?;

    println!("Value at position 0: {}", result);

    let nv : Vec<u32> = find_verb_noun(opcodes)?;
    println!("Noun/Verb pair found: {{{}, {}}}, code: {}", nv[0], nv[1], 100 * nv[0] + nv[1]);

    Ok(())
}
