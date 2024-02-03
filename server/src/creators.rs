// logic for randomly generating round 1
use rand::{Rng, random};
use std::fs::File;
use std::io::Write;

pub fn create_round_1(team_path: &str) {
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

    writer(team_path, output, 1);
}

fn writer(team_path: &str, output: String, round_num: u8) {
    let mut round_file = File::create(format!("{}/round_{round_num}.txt", team_path))
        .expect("Failed to create file");

    round_file.write_all(output.as_bytes())
        .expect("Failed to write into file");
    
    println!("Created Round {round_num} file for {team_path}")
}

pub fn create_round_2(team_path: &str) {
    const MAX_LINES: u32 = 6969;
    const MIN_CHARS: u32 = 6;
    const MAX_CHARS: u32 = 30;

    let mut rng = rand::thread_rng();
    let mut output = String::new();

    for _ in 0..MAX_LINES {
        let characters = rng.gen_range(MIN_CHARS..MAX_CHARS);
        for _ in 0..characters {
            output += &rng.gen_range(1..10).to_string();
        }
        output += "\n";
    }

    writer(team_path, output, 2);
}
