// Number of Groups
// Random question input generator
// list of questions

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use rand::random;
use rand::Rng;

const TEAM_COUNT: u8 = 2;
const ROUND_COUNT: u8 = 1;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter choice");
        io::stdin().read_line(&mut input)
            .expect("Failed To read input");

        let choice = input.trim().parse::<usize>().unwrap();

        match choice {
            1 => create_dirs(),
            2 => create_round_inputs(),
            _ => break,
        }
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

fn create_round_inputs() {
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
        _ => println!("Invalid round")
    }
}

// logic for randomly generating round 1
fn create_round_1(team_path: &str) {
    const MIN_CHARS: u32 = 6;
    const MAX_CHARS: u32 = 30;
    const MAX_LINES: u32 = 6969;
    const MIN_NUMBER_RATE: f32 = 0.25;
    const MAX_NUMBER_RATE: f32 = 0.75;

    let mut rng = rand::thread_rng();
    let mut output = String::new();

    for _ in 0..MAX_LINES {
        let number_rate = rng.gen_range(MIN_NUMBER_RATE..=MAX_NUMBER_RATE);
        let characters = rng.gen_range(MIN_CHARS..=MAX_CHARS);

        for _ in 0..characters {
            if random::<f32>() <= number_rate { // Number
                output += &rng.gen_range(1..10).to_string();
            } else { // CHaracter
                match rng.gen_range(1..=2) {
                    1 => output += &rng.gen_range('a'..='z').to_string(),
                    2 => output += &rng.gen_range('A'..='Z').to_string(),
                    _ => unreachable!(),
                };
                            }
        }
        output += "\n";
    }

    let mut round_file = File::create(format!("{}/round_1.txt", team_path))
        .expect("Failed to create file");

    round_file.write_all(output.as_bytes())
        .expect("Failed to write into file");
    
    println!("Created Round 1 file for {team_path}")
}

