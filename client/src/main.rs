use core::panic;
use std::{cmp, io};
use colored::Colorize;

mod round;
mod turn_handler;

fn main() {
    let team_id = input_u32("Enter team number");
    let round_id = input_u32("Enter round number");

    let client_answer = match get_answer(team_id, round_id) {
        Some(x) => x,
        None => {
            println!("The provided input file for round_6 is incomplete/invalid");
            println!("Program needs to exit, hit enter to continue");
            let _ = io::stdin().read_line(&mut String::new());
            panic!()
        }
    };


    let mut turns = turn_handler::get_turns(team_id, round_id);

    println!("type 0 to exit");
    while turns > 0 {
        let team_answer = input_u32("Enter your answer");
        if team_answer == 0 { break }

        turns -= 1;
        match client_answer.cmp(&team_answer) {
            cmp::Ordering::Greater | cmp::Ordering::Less => {
                println!("Your answer is {}\n", "incorrect".bright_red());
            },
            cmp::Ordering::Equal => {
                println!("your answer is {}", "CORRECT!!!".bright_green());
                break;
            },
        }
    }

    if turns == 0 {
        println!("You have run out of Turns!");
    }

    turn_handler::update_turns(team_id, round_id, turns);

    println!("Hit Enter to continue...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn input_u32(msg: &str) -> u32 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().unwrap_or_else(|_| {
        println!("I'd suggest entering a valid number, try again");
        input_u32(msg)
    })
}

fn get_answer(team_id: u32, round_id: u32) -> Option<u32> {
    round::parser(team_id, round_id)
}
