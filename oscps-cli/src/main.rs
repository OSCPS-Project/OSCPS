//! # main
//! This file is the main entry point for OSCPS-CLI
//! At the moment, primarily for testing purposes.

use oscps_lib::blocks;
use std::io;

fn main() {
        
    let block_id = "1";
    let mass_balance = blocks::Mixer::new(block_id);

    let mut input = String::new();
    let mut count = 1;

    loop {
        println!("Enter your prompt ('q' to quit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();

        match command {
            "q" => {
                println!("Exiting...");
                break;
            },
            "n" => {
                count += 1;
                let mass_balance = blocks::Mixer::new(count.to_string());
                    println!("Created new mixer, block id: {}", mass_balance.block_id);
            },
            _ => println!("Unknown command: {command}")
        };

        let test_string = mass_balance.block_id.clone();
        println!("ID: {test_string}")
    }
}
