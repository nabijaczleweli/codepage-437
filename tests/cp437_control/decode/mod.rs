use self::super::{ALL_CP437, ALL_UTF8};
use codepage_437::CP437_CONTROL;

mod borrow_from_cp437;
mod from_cp437;


#[test]
fn decode() {
    let mut full_size = 0;
    for (cnt, (&b, c)) in ALL_CP437.iter().zip(ALL_UTF8.chars()).enumerate() {
        assert_eq!(CP437_CONTROL.decode(b), c);

        if b.is_ascii() {
            assert_eq!(b as char, c);
        }

        assert_eq!(b as usize, cnt); // Verify test data is consecutive
        full_size = cnt;
    }

    // Verify test data covers all 256 bytes
    assert_eq!(full_size, 0xFF);
}
