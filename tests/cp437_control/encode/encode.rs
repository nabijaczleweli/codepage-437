use self::super::super::{VARIANTS_CP437, VARIANTS_UTF8, ALL_CP437, ALL_UTF8};
use codepage_437::CP437_CONTROL;


#[test]
fn normal() {
    for (&b, c) in ALL_CP437.iter().zip(ALL_UTF8.chars()) {
        assert_eq!(CP437_CONTROL.encode(c), Some(b));
    }
}

#[test]
fn variants() {
    for (&b, c) in VARIANTS_CP437.iter().zip(VARIANTS_UTF8.chars()) {
        assert_eq!(CP437_CONTROL.encode(c), Some(b));
    }
}

#[test]
fn unmapped() {
    for c in "ĄĘĆŹŻŃŁąęćźżńł".chars() {
        assert_eq!(CP437_CONTROL.encode(c), None);
    }
}
