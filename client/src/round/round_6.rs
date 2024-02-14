// ARMSTRONG??!!
/*
* In this round we ask the participants to generate a list of armstrong numbers from zero to 4
* million, the participants are then to identify how to place the file in their respective
* directory and execute the client to validate the result and enter the number of armstrong numbers
* in the given range
*/

use crate::Colorize;
const ARMSTRONG_NUMBERS: u32 = 25;

pub(super) fn compute(input_string: &str) -> Option<u32> {
    let mut armstrong_count = 0;
    for (index, line) in input_string.lines().enumerate() {
        let number = line.parse::<u32>()
            .unwrap_or_else(|_| {
                println!("Line {index} is {} \nLineContent: {}", "NOT A NUMBER".bright_red(), line.bright_red());
                11
            });
        if !is_armstrong(number) {
            println!("{number} is Not An Armstrong Number");
        }
        armstrong_count += 1;
    }
    if ARMSTRONG_NUMBERS == armstrong_count {
        Some(armstrong_count)
    } else {
        None
    }
}

fn is_armstrong(number: u32) -> bool {
    let digits = number.to_string()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let digits_exponent = digits.len() as u32;

    number == digits.iter()
        .map(|x| x.pow(digits_exponent))
        .sum()
}
