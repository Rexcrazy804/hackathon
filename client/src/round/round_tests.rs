use super::round_1;
use super::round_2;
use super::round_3;

#[test]
fn round_1_test() {
    assert_eq!(round_1::compute("1asjkldfalk;jljag;"), 11);
    assert_eq!(round_1::compute("1tgow2kajsd9kjlas"), 19);
    assert_eq!(round_1::compute("001010101010101"), 1);
    assert_eq!(round_1::compute("abab"), 0);
    assert_eq!(round_1::compute("aaaa09"), 9);
    assert_eq!(round_1::compute("9AL:Kjg123"), 93);
}

#[test]
fn round_2_test() {
    assert_eq!(round_2::compute("19182316789"), (1+6+7+8+9));
    assert_eq!(round_2::compute("1212"), (1+2+1+2)); 
    // ignore sequences with singular members
    assert_eq!(round_2::compute("987654321"), 0);
    // esnure sequneces at the tail are parsed
    assert_eq!(round_2::compute("99999999999"), (9+9+9+9+9+9+9+9+9+9+9));
}

#[test]
fn round_3_test() {
    assert_eq!(round_3::compute("[(191)(823)(167)(89)]"), (1+6+7+8+9));
}
