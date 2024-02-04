use std::cmp::Ordering;
pub(super) fn compute(input_string: &str) -> u32 {
    let mut sum_of_sequence_digits = 0;

    for line in input_string.lines() {
        let mut current_sequence: Vec<u32> = Vec::new();
        let mut highest_length = 1;
        let mut line_sum = 0;

        for num in line.chars().map(|x| x.to_digit(10).unwrap()) {
            // num is alwasy greater than zero in the event that the sequence is empty we will
            // force the if condition to be true by imposing num > 0
            if &num > current_sequence.last().unwrap_or(&0) {
                current_sequence.push(num);
            } else {
                match current_sequence.len().cmp(&highest_length) {
                    Ordering::Less => (),
                    Ordering::Equal => {
                        line_sum += current_sequence.iter().sum::<u32>();
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
