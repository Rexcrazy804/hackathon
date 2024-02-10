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
        let num = str.parse::<u32>().unwrap() - 1;
        let num_str_rev = num.to_string().chars()
            .rev()
            .collect::<String>();
        output += &num_str_rev;
        output += " ";

    }
    output.trim().to_string()
}
