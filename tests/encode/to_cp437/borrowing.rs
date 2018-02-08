use self::super::super::super::{VARIANTS_UTF8, ALL_UTF8, is_borrowed};
use codepage_437::ToCp437;


#[test]
fn borrowed_for_ascii() {
    let mut data = String::new();
    for c in ALL_UTF8.chars().take(0x80) {
        data.push(c);

        assert!(is_borrowed(&data.to_cp437().unwrap()));
        assert!(is_borrowed(&[c].into_iter().collect::<String>().to_cp437().unwrap()));
    }
}

#[test]
fn owned_for_beyond_ascii() {
    let mut data = String::new();
    for c in ALL_UTF8.chars().skip(0x80).chain(VARIANTS_UTF8.chars()) {
        data.push(c);

        assert!(!is_borrowed(&data.to_cp437().unwrap()));
        assert!(!is_borrowed(&[c].into_iter().collect::<String>().to_cp437().unwrap()));
    }
}

#[test]
fn owned_for_both() {
    let mut beyond_iter = ALL_UTF8.chars().skip(0x80).chain(VARIANTS_UTF8.chars()).cycle();

    let mut data = String::new();
    data.push(beyond_iter.next().unwrap());

    for c in ALL_UTF8.chars().take(0x80) {
        let new_beyond = beyond_iter.next().unwrap();

        data.pop();
        data.push(c);
        data.push(new_beyond);

        assert!(!is_borrowed(&data.to_cp437().unwrap()));
        assert!(!is_borrowed(&[c, new_beyond].into_iter().collect::<String>().to_cp437().unwrap()));
    }
}
