use super::*;

#[test]
fn encode_test() {
    assert_eq!(encode_1("9_9_9"), "57 95 57 95 57".to_string());
    assert_eq!(encode_2("57 95 57 95 57"), "64 48 64 48 64".to_string());
}

#[test]
fn decode_test() {
    assert_eq!(decode_1("57 95 57 95 57"), "9_9_9".to_string());
    assert_eq!(decode_2("64 48 64 48 64"), "57 95 57 95 57".to_string());
}

#[test]
fn crypt_master_test() {
    assert_eq!("9_9_9".to_string(), decoder(&encoder("9_9_9")));
}
