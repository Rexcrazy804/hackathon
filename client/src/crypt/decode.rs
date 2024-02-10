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
            .map(|ch| {
                let num = ch.to_digit(10).unwrap() + 1;
                num.to_string()
            })
            .collect::<String>();
        output += &str_rev;
        output += " ";

    }
    output.trim().to_string()
}
