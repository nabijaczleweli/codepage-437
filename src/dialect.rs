use std::hash::{Hasher, Hash};
use std::{cmp, fmt};


/// Specifier for the specific kind of cp437.
///
/// Dialects are instances of this type, aggregating free functions necessary to perform conversions.
#[derive(Copy, Clone)]
pub struct Cp437Dialect {
    cp437_to_unicode: [char; 256],

    overlap_unicode: fn(unicode: char) -> bool,
    overlap_cp437: fn(cp437: u8) -> bool,

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
        self.cp437_to_unicode[cp437 as usize]
    }

    /// Try to encode a single Unicode codepoint as a cp437 one.
    #[inline(always)]
    pub fn encode(&self, unicode: char) -> Option<u8> {
        (self.encode)(unicode)
    }
}

// These traits are implemented manually, because rustc is at a loss for big arrays (like the 256 one).
impl fmt::Debug for Cp437Dialect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Cp437Dialect")
            .field("cp437_to_unicode", &&self.cp437_to_unicode[..])
            .field("overlap_unicode", &self.overlap_unicode)
            .field("overlap_cp437", &self.overlap_cp437)
            .field("encode", &self.encode)
            .finish()
    }
}

impl Hash for Cp437Dialect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cp437_to_unicode[..].hash(state);
        self.overlap_unicode.hash(state);
        self.overlap_cp437.hash(state);
        self.encode.hash(state);
    }
}

impl cmp::Eq for Cp437Dialect {}

impl cmp::PartialEq for Cp437Dialect {
    fn eq(&self, other: &Cp437Dialect) -> bool {
        self.cp437_to_unicode[..] == other.cp437_to_unicode[..] &&  // align
        self.overlap_unicode == other.overlap_unicode &&            // align
        self.overlap_cp437 == other.overlap_cp437 &&                // align
        self.encode == other.encode
    }
}

impl cmp::Ord for Cp437Dialect {
    fn cmp(&self, other: &Cp437Dialect) -> cmp::Ordering {
        self.cp437_to_unicode[..]
            .cmp(&other.cp437_to_unicode[..])
            .then(self.overlap_unicode.cmp(&other.overlap_unicode))
            .then(self.overlap_cp437.cmp(&other.overlap_cp437))
            .then(self.encode.cmp(&other.encode))
    }
}

impl cmp::PartialOrd for Cp437Dialect {
    fn partial_cmp(&self, other: &Cp437Dialect) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}


include!(concat!(env!("OUT_DIR"), "/dialects.rs"));
