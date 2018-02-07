use codepage_437::pc::is_pc_cp437_or_ascii;


#[test]
fn yes() {
    for c in 0..0x80 {
        assert!(is_pc_cp437_or_ascii(c));
    }
}

#[test]
fn no() {
    for c in 0x80..0xFF {
        assert!(!is_pc_cp437_or_ascii(c));
    }
    assert!(!is_pc_cp437_or_ascii(0xFF));
}
