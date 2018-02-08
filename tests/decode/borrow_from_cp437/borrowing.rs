use self::super::super::super::is_borrowed;
use codepage_437::BorrowFromCp437;
use std::borrow::Cow;


#[test]
fn borrowed_for_ascii_subset() {
    let mut data = vec![];
    while data.len() <= 0x7F {
        let dlen = data.len();
        data.push(dlen as u8);

        assert!(is_borrowed(&Cow::borrow_from_cp437(&data)));
        assert!(is_borrowed(&Cow::borrow_from_cp437(&[*data.last().unwrap()])));
    }
}

#[test]
fn owned_for_ascii_superset() {
    let mut data = vec![];
    while data.len() <= (0xFF - 0x80) {
        let dlen = data.len();
        data.push((dlen + 0x80) as u8);

        assert!(!is_borrowed(&Cow::borrow_from_cp437(&data)));
        assert!(!is_borrowed(&Cow::borrow_from_cp437(&[*data.last().unwrap()])));
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

        assert!(!is_borrowed(&Cow::borrow_from_cp437(&data)));
        assert!(!is_borrowed(&Cow::borrow_from_cp437(&[data[dlen], data[dlen + 1]])));
    }
}
