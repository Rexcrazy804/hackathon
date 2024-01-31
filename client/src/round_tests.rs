
#[test]
fn round_1_test() {
    assert_eq!(super::round_1::compute("1asjkldfalk;jljag;"), 11);
    assert_eq!(super::round_1::compute("1tgow2kajsd9kjlas"), 19);
    assert_eq!(super::round_1::compute("001010101010101"), 1);
    assert_eq!(super::round_1::compute("abab"), 0);
    assert_eq!(super::round_1::compute("aaaa09"), 9);
    assert_eq!(super::round_1::compute("9AL:Kjg123"), 93);
}
