use rand::seq::SliceRandom;
// logic for randomly generating round 1
use rand::{self, Rng, random};
use std::fs::File;
use std::io::Write;

const MIN_CHARS: u32 = 10;
const MAX_CHARS: u32 = 40;
const MIN_LINES: u32 = 4200;
const MAX_LINES: u32 = 6969;

pub fn create_round_1(team_path: &str) {
    const MIN_NUMBER_RATE: f32 = 0.25;
    const MAX_NUMBER_RATE: f32 = 0.75;

    let mut rng = rand::thread_rng();
    let mut output = String::new();

    let lines = rng.gen_range(MIN_LINES..=MAX_LINES);
    for _ in 0..lines {
        let number_rate = rng.gen_range(MIN_NUMBER_RATE..=MAX_NUMBER_RATE);
        let characters = rng.gen_range(MIN_CHARS..=MAX_CHARS);

        for _ in 0..characters {
            if random::<f32>() <= number_rate { // Number
                output += &rng.gen_range(0..10).to_string();
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

    let mut rng = rand::thread_rng();
    let mut output = String::new();

    let lines = rng.gen_range(MIN_LINES..=MAX_LINES);
    for _ in 0..lines {
        let characters = rng.gen_range(MIN_CHARS..MAX_CHARS);
        for _ in 0..characters {
            output += &rng.gen_range(1..10).to_string();
        }
        output += "\n";
    }

    writer(team_path, output, 2);
}

pub fn create_round_3(team_path: &str) {
    // number of rows / columns of the generated matrix
    const MIN_INDEX: u32 = 2;
    const MAX_INDEX: u32 = 5;
    
    let mut rng = rand::thread_rng();
    let mut output = String::new();

    let lines = rng.gen_range(MIN_LINES..=MAX_LINES);
    for _ in 0..lines {
        let rows = rng.gen_range(MIN_INDEX..=MAX_INDEX);
        let columns = rng.gen_range(MIN_INDEX..=MAX_INDEX);

        output += "[";
        for _ in 0..rows {
            output += "(";
            for _ in 0..columns {
                output += &rng.gen_range(0..10).to_string();
            }
            output += ")";
        }
        output +="]\n";
    }
    writer(team_path, output, 3);
}

pub fn create_round_4(team_path: &str) {
    const FACTOR: u32 = 5;
    const MIN_GAMES: u32 = MIN_LINES/FACTOR;
    const MAX_GAMES: u32 = MAX_LINES/FACTOR;

    let mut rng = rand::thread_rng();
    let mut output = String::new();

    let games = rng.gen_range(MIN_GAMES..=MAX_GAMES);

    for _ in 0..games {
        let mut table = vec!['_'; 9];
        let mut positions = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut stroke = random::<bool>();

        let mut get_stroke = || {
            stroke = !stroke;
            if stroke {
                'X'
            } else {
                'O'
            }
        };

        while !positions.is_empty() && !won(&table) {
            let index = rng.gen_range(0..positions.len());
            table[positions[index]] = get_stroke();
            positions.remove(index);
        }

        output += &table.chunks(3).map(|row| row.iter().collect::<String>()).fold(String::new(), |acc, x| acc + " " + &x);
        output += "\n";
    }

    writer(team_path, output, 4);
}

fn won(table: &[char]) -> bool {
    // rowks
    if table.chunks(3).filter(|row| {
        let (x, y, z) = (row[0], row[1], row[2]);
        x != '_' && x == y && x == z
    }).count() > 0 {
        return true;
    }

    //columns
    if (0..3).filter(|&index|
        table[index] != '_' && table[index] == table[index+3] && table[index] == table[index+6]
    ).count() > 0 {
        return true;
    }

    // Diagonal left
    if table[0] != '_' && table[0] == table[4] && table[0] == table[8] {
        return true;
    }

    // Diagonal Right
    if table[2] != '_' && table[2] == table[4] && table[2] == table[6] {
        return true;
    }

    false
}

pub fn create_round_5(team_path: &str) {
    const FACTOR: u32 = 5;
    const MIN_NODE_COUNT: usize = 7;
    const MAX_NODE_COUNT: usize = 15;
    const RANGE_UPPER_LIMIT: u32 = 100;

    let mut rng = rand::thread_rng();
    let mut output = String::new();

    let lines = rng.gen_range((MIN_LINES/FACTOR)..=(MAX_LINES/FACTOR));
    for _ in 0..lines {
        let node_count = rng.gen_range(MIN_NODE_COUNT..=MAX_NODE_COUNT);
        let nums = (0..RANGE_UPPER_LIMIT).collect::<Vec<u32>>();

        let nums: Vec<u32> = nums
            .choose_multiple(&mut rand::thread_rng(), node_count)
            .map(|x| x.to_owned())
            .collect();

        let line = nums.iter().map(u32::to_string).fold(String::new(), |acc, num| acc + &num + " ");
        let line = line.trim();

        output += line;
        output += "\n";
    }

    writer(team_path, output, 5);
}
