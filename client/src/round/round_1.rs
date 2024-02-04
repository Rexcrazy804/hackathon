pub(super) fn compute(input_string: &str) -> u32 {
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

