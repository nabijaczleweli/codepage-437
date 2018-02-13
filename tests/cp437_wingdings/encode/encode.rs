use self::super::super::{VARIANTS_CP437, VARIANTS_UTF8, ALL_UTF8};
use self::super::super::super::ALL_CP437;
use codepage_437::CP437_WINGDINGS;


#[test]
fn normal() {
    for (&b, c) in ALL_CP437.iter().zip(ALL_UTF8.chars()) {
        assert_eq!(CP437_WINGDINGS.encode(c), Some(b));
    }
}

#[test]
fn variants() {
    for (&b, c) in VARIANTS_CP437.iter().zip(VARIANTS_UTF8.chars()) {
        assert_eq!(CP437_WINGDINGS.encode(c), Some(b));
    }
}

#[test]
fn unmapped() {
    for c in "ĄĘĆŹŻŃŁąęćźżńł".chars() {
        assert_eq!(CP437_WINGDINGS.encode(c), None);
    }
}
