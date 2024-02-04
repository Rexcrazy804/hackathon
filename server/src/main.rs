// Number of Groups
// Random question input generator
// list of questions

use std::fs;
use std::io;

const TEAM_COUNT: u8 = 2;
const ROUND_COUNT: u8 = 3;

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
        let team_path = format!("./Team_{}", i);
        match fs::create_dir(team_path) {
            Ok(_) => (),
            Err(_) => println!("Failed to create dir") 
        }
    }
}

fn create_team_inputs() {
    for i in 1..=TEAM_COUNT {
        let team_path = format!("./Team_{}", i);
        for round in 1..=ROUND_COUNT {
            create_input(&team_path, round);
        }
    }
}

fn create_input(team_path: &str, round: u8) {
    match round {
        1 => create_round_1(team_path),
        2 => create_round_2(team_path),
        3 => create_round_3(team_path),
        _ => println!("Invalid round")
    }
}


