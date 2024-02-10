pub fn encode_1(input: &str) -> String {
    let mut output = String::new();
    for ch in input.chars() {
        let ascii = ch as u32;
        output += &ascii.to_string();
        output += " ";
    }
    output.trim().to_string()
}

pub fn encode_2(input: &str) -> String {
    let mut output = String::new();
    for str in input.split(' ') {
        let str_rev = str.chars()
            .rev()
            .map(|ch| {
                let num = ch.to_digit(10).unwrap() - 1;
                num.to_string()
            })
            .collect::<String>();
        output += &str_rev;
        output += " ";

    }
    output.trim().to_string()
}
