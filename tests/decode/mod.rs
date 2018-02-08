use self::super::{ALL_CP437, ALL_UTF8};
use codepage_437;

mod borrow_from_cp437;
mod from_cp437;


#[test]
fn cp437_to_unicode() {
    let mut full_size = 0;
    for (cnt, (&b, c)) in ALL_CP437.iter().zip(ALL_UTF8.chars()).enumerate() {
        assert_eq!(codepage_437::cp437_to_unicode(b), c);

        if b.is_ascii() {
            assert_eq!(b as char, c);
        }

        assert_eq!(b as usize, cnt); // Verify test data is consecutive
        full_size = cnt;
    }

    // Verify test data covers all 256 bytes
    assert_eq!(full_size, 0xFF);
}
