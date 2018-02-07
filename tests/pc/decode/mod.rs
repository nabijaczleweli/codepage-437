use codepage_437::pc;
use std::borrow::Cow;

mod borrow_from_pc_cp437;
mod from_pc_cp437;


const ALL_CP437: &[u8] = include_bytes!("../../../test-data/pc/all.cp437");
const ALL_UTF8: &str = include_str!("../../../test-data/pc/all.utf8");


fn is_borrowed<T: ToOwned + ?Sized>(who: &Cow<T>) -> bool {
    match who {
        &Cow::Borrowed(_) => true,
        &Cow::Owned(_) => false,
    }
}


#[test]
fn pc_cp437_to_unicode() {
    let mut full_size = 0;
    for (cnt, (&b, c)) in ALL_CP437.iter().zip(ALL_UTF8.chars()).enumerate() {
        assert_eq!(pc::pc_cp437_to_unicode(b), c);

        if b.is_ascii() {
            assert_eq!(b as char, c);
        }

        assert_eq!(b as usize, cnt); // Verify test data is consecutive
        full_size = cnt;
    }

    // Verify test data covers all 256 bytes
    assert_eq!(full_size, 0xFF);
}
