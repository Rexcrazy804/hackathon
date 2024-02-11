use super::*;

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
    // uhh I can't even think of edgecases here :')
    // needa think of more tests later
    assert_eq!(round_3::compute("[(191)(823)(167)]\n[(02)(20)]"), (9 + 4));
    assert_eq!(round_3::compute("[(111)(222)(333)]\n[(111)(222)(333)]\n[(2)(3)(4)(5)(6)]"), (6 + 9 + 9));
}

#[test]
fn round_4_test() {
    // X wins
    assert_eq!(round_4::compute("XXO _XO XXX"), 2);
    // O wins
    assert_eq!(round_4::compute("OXO _X_ OOO"), 3);
    // draw
    assert_eq!(round_4::compute("OXO XXO OOX"), 4);
}

#[test]
#[allow(clippy::identity_op)]
fn round_5_test() {
    assert_eq!(round_5::compute("9 5 4 1 8 3 6 7 0 2"),
        (0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9).to_string()
            .chars().map(|x| x.to_digit(10).unwrap()).product()
    );
}
