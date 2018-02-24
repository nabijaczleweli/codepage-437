use codepage_437::CP437_WINGDINGS;


#[test]
fn simple() {
    assert_eq!(CP437_WINGDINGS.encode('√'), Some(0xFB));
    assert_eq!(CP437_WINGDINGS.encode('✓'), Some(0xFB));

    assert_eq!(CP437_WINGDINGS.decode(0xFB), '√');

    let mut mapping = CP437_WINGDINGS.clone();
    mapping.remap(0xFB, '✓');

    assert_eq!(mapping.encode('√'), Some(0xFB));
    assert_eq!(mapping.encode('✓'), Some(0xFB));

    assert_eq!(mapping.decode(0xFB), '✓');
}

#[test]
fn hard() {
    assert_eq!(CP437_WINGDINGS.encode('Ź'), None);
    assert_eq!(CP437_WINGDINGS.encode('A'), Some(0x41));

    assert_eq!(CP437_WINGDINGS.decode(0x41), 'A');

    let mut mapping = CP437_WINGDINGS.clone();
    mapping.remap(0x41, 'Ź');

    assert_eq!(mapping.encode('Ź'), Some(0x41));
    assert_eq!(mapping.encode('A'), Some(0x41)); // NB: still holds

    assert_eq!(mapping.decode(0x41), 'Ź');
}

#[test]
fn double() {
    assert_eq!(CP437_WINGDINGS.encode('Ź'), None);
    assert_eq!(CP437_WINGDINGS.encode('A'), Some(0x41));
    assert_eq!(CP437_WINGDINGS.encode('√'), Some(0xFB));
    assert_eq!(CP437_WINGDINGS.encode('✓'), Some(0xFB));

    assert_eq!(CP437_WINGDINGS.decode(0x41), 'A');
    assert_eq!(CP437_WINGDINGS.decode(0xFB), '√');

    let mut mapping = CP437_WINGDINGS.clone();
    mapping.remap(0x41, 'Ź');
    mapping.remap(0xFB, '✓');

    assert_eq!(mapping.encode('Ź'), Some(0x41));
    assert_eq!(mapping.encode('A'), Some(0x41)); // NB: still holds
    assert_eq!(mapping.encode('√'), Some(0xFB));
    assert_eq!(mapping.encode('✓'), Some(0xFB));

    assert_eq!(mapping.decode(0x41), 'Ź');
    assert_eq!(mapping.decode(0xFB), '✓');
}
