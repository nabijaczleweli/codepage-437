#[inline(always)]
fn DIALECT_OVERLAP_CP437(b: u8) -> bool {
    b == 0 || (b > 0x1F && b < 0x7F)
}

#[inline(always)]
fn DIALECT_OVERLAP_UNICODE(c: char) -> bool {
    c == '\u{00}' || (c > '\u{1F}' && c < '\u{7F}')
}
