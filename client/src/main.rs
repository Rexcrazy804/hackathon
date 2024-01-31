use std::io;

mod round_1;

#[cfg(test)]
mod round_tests;

fn main() {
    let team_id = input_u32("Enter team number");
    let round = input_u32("Enter round number");

    get_answer(team_id, round);
}


fn input_u32(msg: &str) -> u32 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().unwrap_or_else(|_| {
        println!("Failed to read team number, try again");
        input_u32(msg)
    })
}

fn get_answer(team_id: u32, round: u32) {
    match round {
        1 => round_1::parser(team_id),
        _ => (),
    }
}
