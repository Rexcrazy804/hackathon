use std::fs;
use crate::Colorize;
use crypt::{encoder, decoder};

pub fn get_turns(team_id: u32, round_id: u32) -> u32 {
    let file_path = format!("./Team_{team_id}/round.log");
    let log_lines = fs::read_to_string(file_path)
        .expect("Failed to read log file");

    let round_string = decoder(&log_lines).trim().to_owned();
    display_round_string(&round_string, round_id);

    let rounds = round_string
        .split('_')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    rounds.get(round_id as usize - 1)
        .expect("Invalid round number :/")
        .to_owned()
}

fn display_round_string(round_string: &str, round_id: u32) {
    let ch = round_string.chars().filter(|&ch| ch != '_').nth(round_id as usize - 1)
        .expect("Invalid Round Number");

    let turns_left = ch.to_string().bright_red();
    let round_num = round_id.to_string().bright_green();
    println!("Round {round_num} has {turns_left} turns left");
    println!();
}

pub fn update_turns(team_id: u32, round_id: u32, turns: u32) {
    let file_path = format!("./Team_{team_id}/round.log");
    let log_lines = fs::read_to_string(file_path.clone()).unwrap();

    let round_string = decoder(&log_lines);
    let mut output = String::new();

    for (index, ch) in round_string.chars().filter(|ch| ch != &'_').enumerate() {
        if index + 1 == round_id as usize{
            output += &turns.to_string();
        } else {
            output += &ch.to_string();
        }
        output += "_";
    }

    let output = encoder(output.trim_end_matches('_'));

    fs::write(file_path, output)
        .expect("Failed to write to log file");
}
