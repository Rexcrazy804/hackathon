use super::*;

#[test]
fn encode_test() {
    assert_eq!(encode_1("9_9_9"), "57 95 57 95 57".to_string());
    assert_eq!(encode_2("57 95 57 95 57"), "65 49 65 49 65".to_string());
}

#[test]
fn decode_test() {
    assert_eq!(decode_1("57 95 57 95 57"), "9_9_9".to_string());
    assert_eq!(decode_2("65 49 65 49 65"), "57 95 57 95 57".to_string());
}

#[test]
fn crypt_master_test() {
    let text_cases = ["9_9_9", "3_1_3", "0_3_3"];
    assert_eq!(text_cases[0].to_string(), decoder(&encoder(text_cases[0])));
    assert_eq!(text_cases[1].to_string(), decoder(&encoder(text_cases[1])));
    assert_eq!(text_cases[2].to_string(), decoder(&encoder(text_cases[2])));
}
