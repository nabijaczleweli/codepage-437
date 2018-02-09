/// Specifier for the specific kind of cp437.
///
/// Dialects are instances of this type, aggregating free functions necessary to perform conversions.
#[derive(Debug, Copy, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Cp437Dialect {
    overlap_unicode: fn(unicode: char) -> bool,
    overlap_cp437: fn(cp437: u8) -> bool,

    decode: fn(cp437: u8) -> char,
    encode: fn(unicode: char) -> Option<u8>,
}

impl Cp437Dialect {
    /// Check, whether the specified Unicode codepoint overlaps with a cp437 one.
    #[inline(always)]
    pub fn overlap_unicode(&self, unicode: char) -> bool {
        (self.overlap_unicode)(unicode)
    }

    /// Check, whether the specified cp437 codepoint overlaps with a Unicode one.
    #[inline(always)]
    pub fn overlap_cp437(&self, cp437: u8) -> bool {
        (self.overlap_cp437)(cp437)
    }

    /// Decode a single cp437 codepoint as a Unicode one.
    #[inline(always)]
    pub fn decode(&self, cp437: u8) -> char {
        (self.decode)(cp437)
    }

    /// Try to encode a single Unicode codepoint as a cp437 one.
    #[inline(always)]
    pub fn encode(&self, unicode: char) -> Option<u8> {
        (self.encode)(unicode)
    }
}


include!(concat!(env!("OUT_DIR"), "/dialects.rs"));
