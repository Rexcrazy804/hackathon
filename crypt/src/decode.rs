pub fn decode_1(input: &str) -> String {
    let mut output = String::new();
    for num in input.split(' ') {
        let ch = char::from_u32(num.parse().unwrap()).unwrap();
        output += &ch.to_string();
    }
    output
}

pub fn decode_2(input: &str) -> String {
    let mut output = String::new();
    for str in input.split(' ') {
        let str_rev = str.chars()
            .rev()
            .collect::<String>();
        let num = str_rev.parse::<u32>().unwrap() + 1;

        output += &num.to_string();
        output += " ";

    }
    output.trim().to_string()
}
