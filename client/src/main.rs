use std::io;
mod round;

fn main() {
    let team_id = input_u32("Enter team number");
    let round_id = input_u32("Enter round number");

    match get_answer(team_id, round_id) {
        Some(answer) => println!("answer is :{}", answer),
        None => println!("Failed to retrieve answer"),
    }
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

fn get_answer(team_id: u32, round_id: u32) -> Option<i32> {
    round::parser(team_id, round_id)
}
