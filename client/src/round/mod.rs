use core::panic;
use std::fs;

mod round_1;
mod round_2;
mod round_3;
mod round_4;
mod round_5;
mod round_6;

#[cfg(test)]
mod round_tests;

pub fn parser(team_id: u32, round: u32) -> Option<u32> {
    let file_path = format!("Team_{team_id}/round_{round}.txt");
    let input_string = fs::read_to_string(file_path).unwrap_or_else(|error| {
        println!("failed to read file! Wrong Team/Round number perhaps?");
        println!("Team: {team_id} Round: {round}");
        println!("Press Enter to continue, the client will have to close");
        let _ = std::io::stdin().read_line(&mut String::new());

        panic!("{error}");
    });

    let answer = match {round} {
        1 => round_1::compute(&input_string).into(),
        2 => round_2::compute(&input_string).into(),
        3 => round_3::compute(&input_string).into(),
        4 => round_4::compute(&input_string).into(),
        5 => round_5::compute(&input_string).into(),
        6 => round_6::compute(&input_string),
        _ => None,
    };

    let env_vars: Vec<String> = std::env::args().collect();

    if let Some(arg) = env_vars.last() {
        if arg == "RexielScarletistheanswer" {
            println!("Answer: {}", answer.unwrap_or(0));
        }
    }

    answer
}
