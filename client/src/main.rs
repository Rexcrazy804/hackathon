use std::{cmp, io};
mod round;

fn main() {
    let team_id = input_u32("Enter team number");
    let round_id = input_u32("Enter round number");

    let client_answer = get_answer(team_id, round_id)
        .expect("Client failed to evaluate answer");

    loop {
        let team_answer = input_u32("Enter your answer");
        match client_answer.cmp(&team_answer) {
            cmp::Ordering::Greater | cmp::Ordering::Less => {
                println!("Your answer is incorrect");
                println!("Please try again\n");
            },
            cmp::Ordering::Equal => {
                println!("your answer is CORRECT!!!");
                break;
            },
        }
    }

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
