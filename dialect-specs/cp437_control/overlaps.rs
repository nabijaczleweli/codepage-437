#[inline(always)]
fn DIALECT_OVERLAP_CP437(b: u8) -> bool {
    b.is_ascii()
}

#[inline(always)]
fn DIALECT_OVERLAP_UNICODE(c: char) -> bool {
    c.is_ascii()
}
