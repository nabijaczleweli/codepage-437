use codepage_437::{CP437_WINGDINGS, BorrowFromCp437};
use self::super::super::super::is_borrowed;
use std::borrow::Cow;


#[test]
fn borrowed_for_ascii_subset() {
    let mut data = vec![];
    for b in (0x00..0x01).chain(0x20..0x7F) {
        data.push(b);

        assert!(is_borrowed(&Cow::borrow_from_cp437(&data, &CP437_WINGDINGS)));
        assert!(is_borrowed(&Cow::borrow_from_cp437(&[b], &CP437_WINGDINGS)));
    }
}

#[test]
fn owned_for_ascii_superset() {
    let mut data = vec![];
    for b in (0x01..0x20).chain(0x80..0x100) {
        let b = b as u8;
        data.push(b);

        assert!(!is_borrowed(&Cow::borrow_from_cp437(&data, &CP437_WINGDINGS)));
        assert!(!is_borrowed(&Cow::borrow_from_cp437(&[b], &CP437_WINGDINGS)));
    }
}

#[test]
fn owned_for_both() {
    let mut superset_idx = 0;
    let mut data = vec![0x80];

    while data.len() <= 0x7F {
        data.pop();
        let dlen = data.len();
        data.push(dlen as u8);
        data.push((0x80 + (superset_idx % (0xFF - 0x80))) as u8);
        superset_idx += 1;

        assert!(!is_borrowed(&Cow::borrow_from_cp437(&data, &CP437_WINGDINGS)));
        assert!(!is_borrowed(&Cow::borrow_from_cp437(&[data[dlen], data[dlen + 1]], &CP437_WINGDINGS)));
    }
}
