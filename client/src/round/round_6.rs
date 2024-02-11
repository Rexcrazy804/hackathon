const ARMSTRONG_NUMBERS: u32 = 0;

pub(super) fn compute(input_string: &str) -> u32 {
    let mut armstrong_count = 0;
    for line in input_string.lines() {
        let number = line.parse::<u32>().unwrap();
        if is_armstrong(number) {
            armstrong_count += 1;
        } else {
            println!("{number} is Not An Armstrong Number");
        }
    }
    if ARMSTRONG_NUMBERS == armstrong_count {
        1
    } else {
        696_969_669
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
