// sum of Largest sequence(s) of ascending order numbers
/*
* iterate over each line for each line find the sequence that has more than 1 member that is in the
* ansending order. then find the sum of the digits in the said sequence do this for each line
* return the grand sum
*/

use std::cmp::Ordering;
pub(super) fn compute(input_string: &str) -> u32 {
    let mut sum_of_sequence_digits = 0;
    // println!("{:?}", input_string.clone().lines().map(|s| s.to_string()).collect::<Vec<String>>());

    for line in input_string.lines() {
        let mut current_sequence: Vec<u32> = Vec::new();
        let mut highest_length = 1;
        let mut line_sum = 0;

        // the concatination ensures that the last iteration will force a sequence check
        for num in (line.to_owned() + "0").chars().map(|x| x.to_digit(10).unwrap()) {
            // println!("num: {num}, sum: {line_sum}, sequence: {sum_of_sequence_digits}");
            if current_sequence.is_empty() {
                current_sequence.push(num);
                continue
            }
            if &num >= current_sequence.last().unwrap() {
                current_sequence.push(num);
            } else {
                match current_sequence.len().cmp(&highest_length) {
                    Ordering::Less => (),
                    Ordering::Equal => {
                        if highest_length != 1 {
                            line_sum += current_sequence.iter().sum::<u32>();
                        }
                    },
                    Ordering::Greater => {
                        line_sum = current_sequence.iter().sum::<u32>();
                        highest_length = current_sequence.len();
                    },
                }

                current_sequence.clear();
                current_sequence.push(num);
            }
        }
        sum_of_sequence_digits += line_sum;
    }

    sum_of_sequence_digits
}
