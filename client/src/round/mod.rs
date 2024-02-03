use std::fs;

mod round_1;
mod round_2;

#[cfg(test)]
mod round_tests;

pub fn parser(team_id: u32, round: u32) -> Option<i32> {
    let file_path = format!("Team_{team_id}/round_{round}.txt");
    let Ok(input_string) = fs::read_to_string(file_path) else { println!("failed to read file"); return None };

    let answer = match {round} {
        1 => Some(round_1::compute(&input_string) as i32),
        2 => Some(round_2::compute(&input_string) as i32),
        _ => None,
    };
    println!("{answer:?}");
    answer
}
