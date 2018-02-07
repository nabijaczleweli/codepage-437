//! Conversion to and from [`cp437_DOSLatinUS`](http://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT).
//!
//! Use the `{Borrow,}FromPcCp437` traits to convert series of cp437 bytes to Unicode,
//!     the `pc_cp437_to_unicode()` function
//!
//! # Examples
//!
//! Borrowing from a buffer:
//!
//! ```
//! # use codepage_437::pc::BorrowFromPcCp437;
//! # use std::borrow::Cow;
//! # /*
//! let data = &[/* buffer acquired somewhere */];
//! # */
//! # let data = &[0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x6E, 0x65, 0x77, 0x73];
//!
//! /// in_unicode will be Cow::Borrowed if data only contains overlapping characters,
//! ///                 or Cow::Owned if a conversion needed to have been made.
//! let in_unicode = Cow::borrow_from_pc_cp437(data);
//! # assert_eq!(in_unicode, "Local news");
//!
//! // Also valid:
//! let in_unicode = String::borrow_from_pc_cp437(data);
//! # assert_eq!(in_unicode, "Local news");
//! ```
//!
//! Moving out of a buffer:
//!
//! ```
//! # use codepage_437::pc::FromPcCp437;
//! # /*
//! let data = vec![/* buffer moved in from somewhere */];
//! # */
//! # let data = vec![0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x6E, 0x65, 0x77, 0x73];
//!
//! /// data is moved out of and zero-alloced into in_unicode
//! ///      if it only contains overlapping characters
//! let in_unicode = String::from_pc_cp437(data);
//! # assert_eq!(in_unicode, "Local news");
//! ```


use std::iter::FromIterator;
use std::borrow::Cow;
use std::str;


/// Move data encoded in cp437 to a Unicode container of the specified type.
///
/// # Examples
///
/// ```
/// # use codepage_437::pc::FromPcCp437;
/// let cp437 = vec![0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x6E, 0x65, 0x77, 0x73, 0x20, 0x72, 0x65,
///                  0x70, 0x6F, 0x72, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
///                  0x65, 0x20, 0x9E, 0xAB, 0x20, 0x6D, 0x69, 0x6C, 0x6C, 0x69, 0x6F, 0x6E, 0x20,
///                  0x41, 0x69, 0x72, 0x20, 0x4D, 0x65, 0x6C, 0x61, 0x6E, 0x65, 0x73, 0x69, 0x91,
///                  0x20, 0x61, 0x69, 0x72, 0x63, 0x72, 0x61, 0x66, 0x74, 0x20, 0x68, 0x61, 0x73,
///                  0x20, 0x63, 0x72, 0x61, 0x73, 0x68, 0x65, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73,
///                  0x20, 0x6D, 0x6F, 0x72, 0x6E, 0x69, 0x6E, 0x67, 0x20, 0x61, 0x72, 0x6F, 0x75,
///                  0x6E, 0x64, 0x20, 0x39, 0x3A, 0x30, 0x30, 0x61, 0x6D, 0x2E];
/// let unicode = "Local news reports that the ₧½ million Air Melanesiæ aircraft has crashed this morning around 9:00am.";
///
/// assert_eq!(String::from_pc_cp437(cp437), unicode);  // cp437 is moved out of
/// ```
pub trait FromPcCp437<T: Sized> {
    fn from_pc_cp437(cp437: T) -> Self;
}

/// Try to borrow data encoded in cp437 as a Unicode container of the specified type.
///
/// If that cannot be done, clone it.
///
/// # Examples
///
/// ```
/// # use codepage_437::pc::BorrowFromPcCp437;
/// # use std::borrow::Cow;
/// let cp437 = [0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x6E, 0x65, 0x77, 0x73, 0x20, 0x72, 0x65,
///              0x70, 0x6F, 0x72, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
///              0x65, 0x20, 0x9E, 0xAB, 0x20, 0x6D, 0x69, 0x6C, 0x6C, 0x69, 0x6F, 0x6E, 0x20,
///              0x41, 0x69, 0x72, 0x20, 0x4D, 0x65, 0x6C, 0x61, 0x6E, 0x65, 0x73, 0x69, 0x91,
///              0x20, 0x61, 0x69, 0x72, 0x63, 0x72, 0x61, 0x66, 0x74, 0x20, 0x68, 0x61, 0x73,
///              0x20, 0x63, 0x72, 0x61, 0x73, 0x68, 0x65, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73,
///              0x20, 0x6D, 0x6F, 0x72, 0x6E, 0x69, 0x6E, 0x67, 0x20, 0x61, 0x72, 0x6F, 0x75,
///              0x6E, 0x64, 0x20, 0x39, 0x3A, 0x30, 0x30, 0x61, 0x6D, 0x2E];
/// let unicode = "Local news reports that the ₧½ million Air Melanesiæ aircraft has crashed this morning around 9:00am.";
///
/// assert_eq!(Cow::borrow_from_pc_cp437(&cp437[..]), String::borrow_from_pc_cp437(&cp437[..]));
/// assert_eq!(Cow::borrow_from_pc_cp437(&cp437[..]), unicode);
/// ```
pub trait BorrowFromPcCp437<'c, T: ?Sized> {
    fn borrow_from_pc_cp437(cp437: &'c T) -> Self;
}

macro_rules! from_pc_cp437_impl {
    ($to:expr, $($t:ty)*) => ($(
        impl FromPcCp437<$t> for String {
            fn from_pc_cp437(cp437: $t) -> Self {
                if cp437.iter().all(|&c| is_pc_cp437_or_ascii(c)) {
                    String::from_utf8(cp437.to_vec()).unwrap()
                } else {
                    String::from_iter(cp437.into_iter().map($to))
                }
            }
        }
    )*)
}

macro_rules! borrow_from_pc_cp437_impl {
    ($($t:ty)*) => ($(
        impl<'c> BorrowFromPcCp437<'c, $t> for Cow<'c, str> {
            fn borrow_from_pc_cp437(cp437: &'c $t) -> Self {
                if cp437.iter().all(|&c| is_pc_cp437_or_ascii(c)) {
                    Cow::Borrowed(str::from_utf8(&cp437[..]).unwrap())
                } else {
                    Cow::Owned(String::from_iter(cp437.iter().map(|&c| pc_cp437_to_unicode(c))))
                }
            }
        }

        impl<'c> BorrowFromPcCp437<'c, $t> for String {
            fn borrow_from_pc_cp437(cp437: &'c $t) -> Self {
                if cp437.iter().all(|&c| is_pc_cp437_or_ascii(c)) {
                    str::from_utf8(&cp437[..]).unwrap().to_string()
                } else {
                    String::from_iter(cp437.iter().map(|&c| pc_cp437_to_unicode(c)))
                }
            }
        }
    )*)
}

from_pc_cp437_impl!(pc_cp437_to_unicode, Vec<u8>);
from_pc_cp437_impl!(|&c| pc_cp437_to_unicode(c), [u8;  0] [u8;  1] [u8;  2] [u8;  3] [u8;  4] [u8;  5] [u8;  6] [u8;  7] [u8;  8] [u8;  9]
                                           [u8; 10] [u8; 11] [u8; 12] [u8; 13] [u8; 14] [u8; 15] [u8; 16] [u8; 17] [u8; 18] [u8; 19]
                                           [u8; 20] [u8; 21] [u8; 22] [u8; 23] [u8; 24] [u8; 25] [u8; 26] [u8; 27] [u8; 28] [u8; 29]
                                           [u8; 30] [u8; 31] [u8; 32]);

borrow_from_pc_cp437_impl!([u8] Vec<u8>);
borrow_from_pc_cp437_impl!([u8;  0] [u8;  1] [u8;  2] [u8;  3] [u8;  4] [u8;  5] [u8;  6] [u8;  7] [u8;  8] [u8;  9]
                        [u8; 10] [u8; 11] [u8; 12] [u8; 13] [u8; 14] [u8; 15] [u8; 16] [u8; 17] [u8; 18] [u8; 19]
                        [u8; 20] [u8; 21] [u8; 22] [u8; 23] [u8; 24] [u8; 25] [u8; 26] [u8; 27] [u8; 28] [u8; 29]
                        [u8; 30] [u8; 31] [u8; 32]);


/// Check, whether the specified cp437 has the same representation in ASCII.
///
/// Based on the [cp437](http://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT)
///          and [ASCII](https://www.unicode.org/Public/MAPPINGS/VENDORS/MISC/US-ASCII-QUOTES.TXT)
/// tables provided by the Unicode Consortium.
///
/// # Examples
///
/// ```
/// # use codepage_437::pc::is_pc_cp437_or_ascii;
/// assert!(is_pc_cp437_or_ascii(0x41));   // "A" in both
/// assert!(!is_pc_cp437_or_ascii(0x91));  // "æ" in cp437, "‘" in Unicode
/// ```
pub fn is_pc_cp437_or_ascii(cp437: u8) -> bool {
    cp437 <= 0x7F
}

/// Convert a cp437 codepoint to a Unicode one.
///
/// Based on the [cp437](http://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT)
/// table provided by the Unicode Consortium.
///
/// # Examples
///
/// ```
/// # use codepage_437::pc::pc_cp437_to_unicode;
/// assert_eq!(pc_cp437_to_unicode(0x41), 'A');
/// assert_eq!(pc_cp437_to_unicode(0x91), 'æ');  // LATIN SMALL LIGATURE AE
/// ```
pub fn pc_cp437_to_unicode(cp437: u8) -> char {
    if is_pc_cp437_or_ascii(cp437) {
        cp437 as char
    } else {
        match cp437 {
            0x80 => '\u{00C7}', // LATIN CAPITAL LETTER C WITH CEDILLA
            0x81 => '\u{00FC}', // LATIN SMALL LETTER U WITH DIAERESIS
            0x82 => '\u{00E9}', // LATIN SMALL LETTER E WITH ACUTE
            0x83 => '\u{00E2}', // LATIN SMALL LETTER A WITH CIRCUMFLEX
            0x84 => '\u{00E4}', // LATIN SMALL LETTER A WITH DIAERESIS
            0x85 => '\u{00E0}', // LATIN SMALL LETTER A WITH GRAVE
            0x86 => '\u{00E5}', // LATIN SMALL LETTER A WITH RING ABOVE
            0x87 => '\u{00E7}', // LATIN SMALL LETTER C WITH CEDILLA
            0x88 => '\u{00EA}', // LATIN SMALL LETTER E WITH CIRCUMFLEX
            0x89 => '\u{00EB}', // LATIN SMALL LETTER E WITH DIAERESIS
            0x8A => '\u{00E8}', // LATIN SMALL LETTER E WITH GRAVE
            0x8B => '\u{00EF}', // LATIN SMALL LETTER I WITH DIAERESIS
            0x8C => '\u{00EE}', // LATIN SMALL LETTER I WITH CIRCUMFLEX
            0x8D => '\u{00EC}', // LATIN SMALL LETTER I WITH GRAVE
            0x8E => '\u{00C4}', // LATIN CAPITAL LETTER A WITH DIAERESIS
            0x8F => '\u{00C5}', // LATIN CAPITAL LETTER A WITH RING ABOVE
            0x90 => '\u{00C9}', // LATIN CAPITAL LETTER E WITH ACUTE
            0x91 => '\u{00E6}', // LATIN SMALL LIGATURE AE
            0x92 => '\u{00C6}', // LATIN CAPITAL LIGATURE AE
            0x93 => '\u{00F4}', // LATIN SMALL LETTER O WITH CIRCUMFLEX
            0x94 => '\u{00F6}', // LATIN SMALL LETTER O WITH DIAERESIS
            0x95 => '\u{00F2}', // LATIN SMALL LETTER O WITH GRAVE
            0x96 => '\u{00FB}', // LATIN SMALL LETTER U WITH CIRCUMFLEX
            0x97 => '\u{00F9}', // LATIN SMALL LETTER U WITH GRAVE
            0x98 => '\u{00FF}', // LATIN SMALL LETTER Y WITH DIAERESIS
            0x99 => '\u{00D6}', // LATIN CAPITAL LETTER O WITH DIAERESIS
            0x9A => '\u{00DC}', // LATIN CAPITAL LETTER U WITH DIAERESIS
            0x9B => '\u{00A2}', // CENT SIGN
            0x9C => '\u{00A3}', // POUND SIGN
            0x9D => '\u{00A5}', // YEN SIGN
            0x9E => '\u{20A7}', // PESETA SIGN
            0x9F => '\u{0192}', // LATIN SMALL LETTER F WITH HOOK
            0xA0 => '\u{00E1}', // LATIN SMALL LETTER A WITH ACUTE
            0xA1 => '\u{00ED}', // LATIN SMALL LETTER I WITH ACUTE
            0xA2 => '\u{00F3}', // LATIN SMALL LETTER O WITH ACUTE
            0xA3 => '\u{00FA}', // LATIN SMALL LETTER U WITH ACUTE
            0xA4 => '\u{00F1}', // LATIN SMALL LETTER N WITH TILDE
            0xA5 => '\u{00D1}', // LATIN CAPITAL LETTER N WITH TILDE
            0xA6 => '\u{00AA}', // FEMININE ORDINAL INDICATOR
            0xA7 => '\u{00BA}', // MASCULINE ORDINAL INDICATOR
            0xA8 => '\u{00BF}', // INVERTED QUESTION MARK
            0xA9 => '\u{2310}', // REVERSED NOT SIGN
            0xAA => '\u{00AC}', // NOT SIGN
            0xAB => '\u{00BD}', // VULGAR FRACTION ONE HALF
            0xAC => '\u{00BC}', // VULGAR FRACTION ONE QUARTER
            0xAD => '\u{00A1}', // INVERTED EXCLAMATION MARK
            0xAE => '\u{00AB}', // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
            0xAF => '\u{00BB}', // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
            0xB0 => '\u{2591}', // LIGHT SHADE
            0xB1 => '\u{2592}', // MEDIUM SHADE
            0xB2 => '\u{2593}', // DARK SHADE
            0xB3 => '\u{2502}', // BOX DRAWINGS LIGHT VERTICAL
            0xB4 => '\u{2524}', // BOX DRAWINGS LIGHT VERTICAL AND LEFT
            0xB5 => '\u{2561}', // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
            0xB6 => '\u{2562}', // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
            0xB7 => '\u{2556}', // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
            0xB8 => '\u{2555}', // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
            0xB9 => '\u{2563}', // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
            0xBA => '\u{2551}', // BOX DRAWINGS DOUBLE VERTICAL
            0xBB => '\u{2557}', // BOX DRAWINGS DOUBLE DOWN AND LEFT
            0xBC => '\u{255D}', // BOX DRAWINGS DOUBLE UP AND LEFT
            0xBD => '\u{255C}', // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
            0xBE => '\u{255B}', // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
            0xBF => '\u{2510}', // BOX DRAWINGS LIGHT DOWN AND LEFT
            0xC0 => '\u{2514}', // BOX DRAWINGS LIGHT UP AND RIGHT
            0xC1 => '\u{2534}', // BOX DRAWINGS LIGHT UP AND HORIZONTAL
            0xC2 => '\u{252C}', // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
            0xC3 => '\u{251C}', // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
            0xC4 => '\u{2500}', // BOX DRAWINGS LIGHT HORIZONTAL
            0xC5 => '\u{253C}', // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
            0xC6 => '\u{255E}', // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
            0xC7 => '\u{255F}', // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
            0xC8 => '\u{255A}', // BOX DRAWINGS DOUBLE UP AND RIGHT
            0xC9 => '\u{2554}', // BOX DRAWINGS DOUBLE DOWN AND RIGHT
            0xCA => '\u{2569}', // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
            0xCB => '\u{2566}', // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
            0xCC => '\u{2560}', // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
            0xCD => '\u{2550}', // BOX DRAWINGS DOUBLE HORIZONTAL
            0xCE => '\u{256C}', // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
            0xCF => '\u{2567}', // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
            0xD0 => '\u{2568}', // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
            0xD1 => '\u{2564}', // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
            0xD2 => '\u{2565}', // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
            0xD3 => '\u{2559}', // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
            0xD4 => '\u{2558}', // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
            0xD5 => '\u{2552}', // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
            0xD6 => '\u{2553}', // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
            0xD7 => '\u{256B}', // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
            0xD8 => '\u{256A}', // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
            0xD9 => '\u{2518}', // BOX DRAWINGS LIGHT UP AND LEFT
            0xDA => '\u{250C}', // BOX DRAWINGS LIGHT DOWN AND RIGHT
            0xDB => '\u{2588}', // FULL BLOCK
            0xDC => '\u{2584}', // LOWER HALF BLOCK
            0xDD => '\u{258C}', // LEFT HALF BLOCK
            0xDE => '\u{2590}', // RIGHT HALF BLOCK
            0xDF => '\u{2580}', // UPPER HALF BLOCK
            0xE0 => '\u{03B1}', // GREEK SMALL LETTER ALPHA
            0xE1 => '\u{00DF}', // LATIN SMALL LETTER SHARP S
            0xE2 => '\u{0393}', // GREEK CAPITAL LETTER GAMMA
            0xE3 => '\u{03C0}', // GREEK SMALL LETTER PI
            0xE4 => '\u{03A3}', // GREEK CAPITAL LETTER SIGMA
            0xE5 => '\u{03C3}', // GREEK SMALL LETTER SIGMA
            0xE6 => '\u{00B5}', // MICRO SIGN
            0xE7 => '\u{03C4}', // GREEK SMALL LETTER TAU
            0xE8 => '\u{03A6}', // GREEK CAPITAL LETTER PHI
            0xE9 => '\u{0398}', // GREEK CAPITAL LETTER THETA
            0xEA => '\u{03A9}', // GREEK CAPITAL LETTER OMEGA
            0xEB => '\u{03B4}', // GREEK SMALL LETTER DELTA
            0xEC => '\u{221E}', // INFINITY
            0xED => '\u{03C6}', // GREEK SMALL LETTER PHI
            0xEE => '\u{03B5}', // GREEK SMALL LETTER EPSILON
            0xEF => '\u{2229}', // INTERSECTION
            0xF0 => '\u{2261}', // IDENTICAL TO
            0xF1 => '\u{00B1}', // PLUS-MINUS SIGN
            0xF2 => '\u{2265}', // GREATER-THAN OR EQUAL TO
            0xF3 => '\u{2264}', // LESS-THAN OR EQUAL TO
            0xF4 => '\u{2320}', // TOP HALF INTEGRAL
            0xF5 => '\u{2321}', // BOTTOM HALF INTEGRAL
            0xF6 => '\u{00F7}', // DIVISION SIGN
            0xF7 => '\u{2248}', // ALMOST EQUAL TO
            0xF8 => '\u{00B0}', // DEGREE SIGN
            0xF9 => '\u{2219}', // BULLET OPERATOR
            0xFA => '\u{00B7}', // MIDDLE DOT
            0xFB => '\u{221A}', // SQUARE ROOT
            0xFC => '\u{207F}', // SUPERSCRIPT LATIN SMALL LETTER N
            0xFD => '\u{00B2}', // SUPERSCRIPT TWO
            0xFE => '\u{25A0}', // BLACK SQUARE
            0xFF => '\u{00A0}', // NO-BREAK SPACE

            _ => unreachable!(),
        }
    }
}
