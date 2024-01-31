use std::fs;

pub fn parser(team_id: u32) {
    let file_path = format!("Team_{team_id}/round_1.txt");
    let Ok(input_string) = fs::read_to_string(file_path) else { println!("failed to read file"); return };

    let answer: u32 = compute(&input_string);
    println!("{answer}")
}

fn compute(input_string: &str) -> u32 {
    let mut answer = 0;
    for line in input_string.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<u32>>();

        if digits.is_empty() {
            continue
        }

        answer += digits.first().unwrap()*10 + digits.last().unwrap();
    }
    answer
}


