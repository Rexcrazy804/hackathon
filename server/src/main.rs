// Number of Groups
// Random question input generator
// list of questions

use std::fs;
use std::fs::File;
use std::io::{self, Write};
use crypt::encoder;

const TEAM_COUNT: u8 = 2;
const ROUND_COUNT: u8 = 5;

mod creators;
use creators::*;

fn main() {
    let mut input = String::new();
    println!("Enter choice");
    io::stdin().read_line(&mut input)
        .expect("Failed To read input");

    let choice = input.trim().parse::<usize>().unwrap();

    match choice {
        1 => create_dirs(),
        2 => create_team_inputs(),
        _ => (),
    }
}

fn create_dirs() {
    for i in 1..=TEAM_COUNT {
        let team_path = format!("./Team_{i}");
        match fs::create_dir(team_path) {
            Ok(_) => (),
            Err(_) => println!("Failed to create dir") 
        }
    }
}

fn create_team_inputs() {
    for i in 1..=TEAM_COUNT {
        let team_path = format!("./Team_{i}");
        for round in 1..=ROUND_COUNT {
            create_input(&team_path, round);
        }
        reset_round_tries(&team_path);
    }
}

fn create_input(team_path: &str, round: u8) {
    match round {
        1 => create_round_1(team_path),
        2 => create_round_2(team_path),
        3 => create_round_3(team_path),
        4 => create_round_4(team_path),
        5 => create_round_5(team_path),
        _ => println!("Invalid round")
    }
}

fn reset_round_tries(team_path: &str) {
    println!("Creating log for {team_path}");
    let mut file = File::create(format!("{team_path}/round.log"))
        .expect("Failed to create log File");

    let mut initial_string = String::new();
    for _ in 1..=ROUND_COUNT {
        initial_string += "9_";
    }

    let encoded_string = encoder(initial_string.trim_end_matches('_'));
    file.write_all(encoded_string.as_bytes())
        .expect("Failed to write into log file");
}
