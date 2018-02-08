use std::borrow::Cow;


/// Errors which can occur when attempting to interpret a string as a sequence of cp437 codepoints.
///
/// As such, the `into_pc_cp437` family of functions and methods make use of this error, for example.
#[derive(Debug, Copy, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct PcCp437Error {
    /// Returns the index in the given string up to which valid cp437 was verified.
    ///
    /// It is the maximum index such that `input[..index].to_cp_437()` would return `Ok(_)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::pc::ToPcCp437;
    /// // some unrepresentable characters, in a &str
    /// let word = "Eżektor";
    ///
    /// // ToPcCp437::to_pc_cp437() returns a PcCp437Error
    /// let error = word.to_pc_cp437().unwrap_err();
    ///
    /// // the second character is unrepresentable here
    /// assert_eq!(error.representable_up_to, 1);
    /// ```
    pub representable_up_to: usize,
}

/// A possible error value when converting a `String` into a cp437 byte vector.
///
/// This type is the error type for the [`into_pc_cp437()`](TODO) function on [`IntoPcCp437`](TODO). It
/// is designed in such a way to carefully avoid reallocations: the
/// [`into_string`](TODO) method will give back the String that was used in the
/// conversion attempt.
///
/// The [`PcCp437Error`](TODO) type represents an error that may
/// occur when converting a `&str` to a sequence of `u8`s. In this sense, it's
/// an analogue to `IntoPcCp437Error`, and you can get one from a `IntoPcCp437Error`
/// through the [`pc_cp437_error()`](TODO) function.
///
/// # Examples
///
/// ```
/// # use codepage_437::pc::IntoPcCp437;
/// // some unrepresentable chracters, in a String
/// let word = "Eżektor".to_string();
///
/// let value = word.into_pc_cp437();
///
/// assert!(value.is_err());
/// assert_eq!(value.unwrap_err().into_string(), "Eżektor".to_string());
/// ```
#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntoPcCp437Error {
    string: String,
    error: PcCp437Error,
}

impl IntoPcCp437Error {
    /// Returns a `&str` that was attempted to convert to cp437.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::pc::IntoPcCp437;
    /// // some unrepresentable chracters, in a String
    /// let word = "Eżektor".to_string();
    ///
    /// let value = word.into_pc_cp437();
    ///
    /// assert_eq!(value.unwrap_err().as_str(), "Eżektor");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.string
    }

    /// Returns the `String` that was attempted to convert to cp437.
    ///
    /// This method is carefully constructed to avoid allocation. It will
    /// consume the error, moving out the string, so that a copy of the string
    /// does not need to be made.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::pc::IntoPcCp437;
    /// // some unrepresentable chracters, in a String
    /// let word = "Eżektor".to_string();
    ///
    /// let value = word.into_pc_cp437();
    ///
    /// assert_eq!(value.unwrap_err().into_string(), "Eżektor".to_string());
    /// ```
    pub fn into_string(self) -> String {
        self.string
    }

    /// Fetch a `PcCp437Error` to get more details about the conversion failure.
    ///
    /// The [`PcCp437Error`](TODO) type represents an error that may
    /// occur when converting a `&str` to a sequence of `u8`s. In this sense, it's
    /// an analogue to `IntoPcCp437Error`. See its documentation for more details
    /// on using it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::pc::IntoPcCp437;
    /// // some unrepresentable chracters, in a String
    /// let word = "Eżektor".to_string();
    ///
    /// let error = word.into_pc_cp437().unwrap_err().pc_cp437_error();
    ///
    /// // the first character is unrepresentable here
    /// assert_eq!(error.representable_up_to, 1);
    /// ```
    pub fn pc_cp437_error(&self) -> PcCp437Error {
        self.error
    }
}


/// Move Unicode data to a container of cp437 data.
///
/// # Examples
///
/// Good:
///
/// ```
/// # use codepage_437::pc::IntoPcCp437;
/// let cp437 = vec![0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x6E, 0x65, 0x77, 0x73, 0x20, 0x72, 0x65,
///                  0x70, 0x6F, 0x72, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
///                  0x65, 0x20, 0x9E, 0xAB, 0x20, 0x6D, 0x69, 0x6C, 0x6C, 0x69, 0x6F, 0x6E, 0x20,
///                  0x41, 0x69, 0x72, 0x20, 0x4D, 0x65, 0x6C, 0x61, 0x6E, 0x65, 0x73, 0x69, 0x91,
///                  0x20, 0x61, 0x69, 0x72, 0x63, 0x72, 0x61, 0x66, 0x74, 0x20, 0x68, 0x61, 0x73,
///                  0x20, 0x63, 0x72, 0x61, 0x73, 0x68, 0x65, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73,
///                  0x20, 0x6D, 0x6F, 0x72, 0x6E, 0x69, 0x6E, 0x67, 0x20, 0x61, 0x72, 0x6F, 0x75,
///                  0x6E, 0x64, 0x20, 0x39, 0x3A, 0x30, 0x30, 0x61, 0x6D, 0x2E];
/// let unicode =
///     "Local news reports that the ₧½ million Air Melanesiæ aircraft has crashed this morning around 9:00am.".to_string();
///
/// assert_eq!(unicode.into_pc_cp437(), Ok(cp437));  // unicode is moved out of
/// ```
///
/// Unrepresentable:
///
/// ```
/// # use codepage_437::pc::IntoPcCp437;
/// // Ż cannot be represented in cp437
/// let unicode = "Jurek je żurek w żupanie.".to_string();
///
/// let error = unicode.into_pc_cp437().unwrap_err();           // unicode is moved out of
/// assert_eq!(error.as_str(), "Jurek je żurek w żupanie.");
/// assert_eq!(error.pc_cp437_error().representable_up_to, 9);
///
/// let unicode = error.into_string();                          // unicode now the same as original
/// # assert_eq!(unicode, "Jurek je żurek w żupanie.");
/// ```
pub trait IntoPcCp437<T> {
    /// Do the conversion.
    fn into_pc_cp437(self) -> Result<T, IntoPcCp437Error>;
}

impl IntoPcCp437<Vec<u8>> for String {
    fn into_pc_cp437(self) -> Result<Vec<u8>, IntoPcCp437Error> {
        if self.is_ascii() {
            Ok(self.into_bytes())
        } else {
            to_pc_cp437_impl_meat(&self).map_err(|e| {
                IntoPcCp437Error {
                    string: self,
                    error: e,
                }
            })
        }
    }
}


/// Borrow (if possible) Unicode data as cp437 data.
///
/// # Examples
///
/// Good:
///
/// ```
/// # use codepage_437::pc::ToPcCp437;
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
/// assert_eq!(unicode.to_pc_cp437(), Ok(cp437[..].into()));
/// ```
///
/// Unrepresentable:
///
/// ```
/// # use codepage_437::pc::ToPcCp437;
/// // Ż cannot be represented in cp437
/// let unicode = "Jurek je żurek w żupanie.";
///
/// let error = unicode.to_pc_cp437().unwrap_err();
/// assert_eq!(error.representable_up_to, 9);
/// ```
pub trait ToPcCp437<'s, T> {
    /// Do the conversion.
    fn to_pc_cp437(&'s self) -> Result<T, PcCp437Error>;
}

impl<'s> ToPcCp437<'s, Cow<'s, [u8]>> for str {
    fn to_pc_cp437(&'s self) -> Result<Cow<'s, [u8]>, PcCp437Error> {
        to_pc_cp437_cow_impl(&self)
    }
}

impl<'s, S: AsRef<str>> ToPcCp437<'s, Cow<'s, [u8]>> for S {
    fn to_pc_cp437(&'s self) -> Result<Cow<'s, [u8]>, PcCp437Error> {
        to_pc_cp437_cow_impl(self.as_ref())
    }
}


fn to_pc_cp437_cow_impl(whom: &str) -> Result<Cow<[u8]>, PcCp437Error> {
    if whom.is_ascii() {
        Ok(Cow::Borrowed(whom.as_bytes()))
    } else {
        to_pc_cp437_impl_meat(whom).map(Cow::Owned)
    }
}

fn to_pc_cp437_impl_meat(whom: &str) -> Result<Vec<u8>, PcCp437Error> {
    let mut result = Vec::with_capacity(whom.chars().count());

    for c in whom.chars() {
        if let Some(b) = unicode_to_pc_cp437(c) {
            result.push(b);
        } else {
            return Err(PcCp437Error { representable_up_to: result.len() });
        }
    }

    Ok(result)
}


/// Hopefully convert a Unicode codepoint to a cp437 one.
///
/// Based on the [cp437](http://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT)
/// table provided by the Unicode Consortium and the [variant](https://en.wikipedia.org/wiki/Code_page_437#Notes)
/// table on Wikipedia.
///
/// # Examples
///
/// ```
/// # use codepage_437::pc::unicode_to_pc_cp437;
/// assert_eq!(unicode_to_pc_cp437('A'), Some(0x41));
/// assert_eq!(unicode_to_pc_cp437('æ'), Some(0x91));  // LATIN SMALL LIGATURE AE
///
/// assert_eq!(unicode_to_pc_cp437('ź'), None);        // LATIN SMALL LETTER Z WITH ACUTE
/// ```
pub fn unicode_to_pc_cp437(unicode: char) -> Option<u8> {
    Some(match unicode {
        '\u{00C7}' => 0x80, // LATIN CAPITAL LETTER C WITH CEDILLA
        '\u{00FC}' => 0x81, // LATIN SMALL LETTER U WITH DIAERESIS
        '\u{00E9}' => 0x82, // LATIN SMALL LETTER E WITH ACUTE
        '\u{00E2}' => 0x83, // LATIN SMALL LETTER A WITH CIRCUMFLEX
        '\u{00E4}' => 0x84, // LATIN SMALL LETTER A WITH DIAERESIS
        '\u{00E0}' => 0x85, // LATIN SMALL LETTER A WITH GRAVE
        '\u{00E5}' => 0x86, // LATIN SMALL LETTER A WITH RING ABOVE
        '\u{00E7}' => 0x87, // LATIN SMALL LETTER C WITH CEDILLA
        '\u{00EA}' => 0x88, // LATIN SMALL LETTER E WITH CIRCUMFLEX
        '\u{00EB}' => 0x89, // LATIN SMALL LETTER E WITH DIAERESIS
        '\u{00E8}' => 0x8A, // LATIN SMALL LETTER E WITH GRAVE
        '\u{00EF}' => 0x8B, // LATIN SMALL LETTER I WITH DIAERESIS
        '\u{00EE}' => 0x8C, // LATIN SMALL LETTER I WITH CIRCUMFLEX
        '\u{00EC}' => 0x8D, // LATIN SMALL LETTER I WITH GRAVE
        '\u{00C4}' => 0x8E, // LATIN CAPITAL LETTER A WITH DIAERESIS
        '\u{00C5}' => 0x8F, // LATIN CAPITAL LETTER A WITH RING ABOVE
        '\u{00C9}' => 0x90, // LATIN CAPITAL LETTER E WITH ACUTE
        '\u{00E6}' => 0x91, // LATIN SMALL LIGATURE AE
        '\u{00C6}' => 0x92, // LATIN CAPITAL LIGATURE AE
        '\u{00F4}' => 0x93, // LATIN SMALL LETTER O WITH CIRCUMFLEX
        '\u{00F6}' => 0x94, // LATIN SMALL LETTER O WITH DIAERESIS
        '\u{00F2}' => 0x95, // LATIN SMALL LETTER O WITH GRAVE
        '\u{00FB}' => 0x96, // LATIN SMALL LETTER U WITH CIRCUMFLEX
        '\u{00F9}' => 0x97, // LATIN SMALL LETTER U WITH GRAVE
        '\u{00FF}' => 0x98, // LATIN SMALL LETTER Y WITH DIAERESIS
        '\u{00D6}' => 0x99, // LATIN CAPITAL LETTER O WITH DIAERESIS
        '\u{00DC}' => 0x9A, // LATIN CAPITAL LETTER U WITH DIAERESIS
        '\u{00A2}' => 0x9B, // CENT SIGN
        '\u{00A3}' => 0x9C, // POUND SIGN
        '\u{00A5}' => 0x9D, // YEN SIGN
        '\u{20A7}' => 0x9E, // PESETA SIGN
        '\u{0192}' => 0x9F, // LATIN SMALL LETTER F WITH HOOK
        '\u{00E1}' => 0xA0, // LATIN SMALL LETTER A WITH ACUTE
        '\u{00ED}' => 0xA1, // LATIN SMALL LETTER I WITH ACUTE
        '\u{00F3}' => 0xA2, // LATIN SMALL LETTER O WITH ACUTE
        '\u{00FA}' => 0xA3, // LATIN SMALL LETTER U WITH ACUTE
        '\u{00F1}' => 0xA4, // LATIN SMALL LETTER N WITH TILDE
        '\u{00D1}' => 0xA5, // LATIN CAPITAL LETTER N WITH TILDE
        '\u{00AA}' => 0xA6, // FEMININE ORDINAL INDICATOR
        '\u{00BA}' => 0xA7, // MASCULINE ORDINAL INDICATOR
        '\u{00BF}' => 0xA8, // INVERTED QUESTION MARK
        '\u{2310}' => 0xA9, // REVERSED NOT SIGN
        '\u{00AC}' => 0xAA, // NOT SIGN
        '\u{00BD}' => 0xAB, // VULGAR FRACTION ONE HALF
        '\u{00BC}' => 0xAC, // VULGAR FRACTION ONE QUARTER
        '\u{00A1}' => 0xAD, // INVERTED EXCLAMATION MARK
        '\u{00AB}' => 0xAE, // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
        '\u{00BB}' => 0xAF, // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
        '\u{2591}' => 0xB0, // LIGHT SHADE
        '\u{2592}' => 0xB1, // MEDIUM SHADE
        '\u{2593}' => 0xB2, // DARK SHADE
        '\u{2502}' => 0xB3, // BOX DRAWINGS LIGHT VERTICAL
        '\u{2524}' => 0xB4, // BOX DRAWINGS LIGHT VERTICAL AND LEFT
        '\u{2561}' => 0xB5, // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
        '\u{2562}' => 0xB6, // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
        '\u{2556}' => 0xB7, // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
        '\u{2555}' => 0xB8, // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
        '\u{2563}' => 0xB9, // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
        '\u{2551}' => 0xBA, // BOX DRAWINGS DOUBLE VERTICAL
        '\u{2557}' => 0xBB, // BOX DRAWINGS DOUBLE DOWN AND LEFT
        '\u{255D}' => 0xBC, // BOX DRAWINGS DOUBLE UP AND LEFT
        '\u{255C}' => 0xBD, // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
        '\u{255B}' => 0xBE, // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
        '\u{2510}' => 0xBF, // BOX DRAWINGS LIGHT DOWN AND LEFT
        '\u{2514}' => 0xC0, // BOX DRAWINGS LIGHT UP AND RIGHT
        '\u{2534}' => 0xC1, // BOX DRAWINGS LIGHT UP AND HORIZONTAL
        '\u{252C}' => 0xC2, // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
        '\u{251C}' => 0xC3, // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
        '\u{2500}' => 0xC4, // BOX DRAWINGS LIGHT HORIZONTAL
        '\u{253C}' => 0xC5, // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
        '\u{255E}' => 0xC6, // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
        '\u{255F}' => 0xC7, // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
        '\u{255A}' => 0xC8, // BOX DRAWINGS DOUBLE UP AND RIGHT
        '\u{2554}' => 0xC9, // BOX DRAWINGS DOUBLE DOWN AND RIGHT
        '\u{2569}' => 0xCA, // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
        '\u{2566}' => 0xCB, // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
        '\u{2560}' => 0xCC, // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
        '\u{2550}' => 0xCD, // BOX DRAWINGS DOUBLE HORIZONTAL
        '\u{256C}' => 0xCE, // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
        '\u{2567}' => 0xCF, // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
        '\u{2568}' => 0xD0, // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
        '\u{2564}' => 0xD1, // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
        '\u{2565}' => 0xD2, // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
        '\u{2559}' => 0xD3, // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
        '\u{2558}' => 0xD4, // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
        '\u{2552}' => 0xD5, // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
        '\u{2553}' => 0xD6, // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
        '\u{256B}' => 0xD7, // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
        '\u{256A}' => 0xD8, // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
        '\u{2518}' => 0xD9, // BOX DRAWINGS LIGHT UP AND LEFT
        '\u{250C}' => 0xDA, // BOX DRAWINGS LIGHT DOWN AND RIGHT
        '\u{2588}' => 0xDB, // FULL BLOCK
        '\u{2584}' => 0xDC, // LOWER HALF BLOCK
        '\u{258C}' => 0xDD, // LEFT HALF BLOCK
        '\u{2590}' => 0xDE, // RIGHT HALF BLOCK
        '\u{2580}' => 0xDF, // UPPER HALF BLOCK
        '\u{03B1}' => 0xE0, // GREEK SMALL LETTER ALPHA
        '\u{00DF}' => 0xE1, // LATIN SMALL LETTER SHARP S
        '\u{0393}' => 0xE2, // GREEK CAPITAL LETTER GAMMA
        '\u{03C0}' => 0xE3, // GREEK SMALL LETTER PI
        '\u{03A3}' => 0xE4, // GREEK CAPITAL LETTER SIGMA
        '\u{03C3}' => 0xE5, // GREEK SMALL LETTER SIGMA
        '\u{00B5}' => 0xE6, // MICRO SIGN
        '\u{03C4}' => 0xE7, // GREEK SMALL LETTER TAU
        '\u{03A6}' => 0xE8, // GREEK CAPITAL LETTER PHI
        '\u{0398}' => 0xE9, // GREEK CAPITAL LETTER THETA
        '\u{03A9}' => 0xEA, // GREEK CAPITAL LETTER OMEGA
        '\u{03B4}' => 0xEB, // GREEK SMALL LETTER DELTA
        '\u{221E}' => 0xEC, // INFINITY
        '\u{03C6}' => 0xED, // GREEK SMALL LETTER PHI
        '\u{03B5}' => 0xEE, // GREEK SMALL LETTER EPSILON
        '\u{2229}' => 0xEF, // INTERSECTION
        '\u{2261}' => 0xF0, // IDENTICAL TO
        '\u{00B1}' => 0xF1, // PLUS-MINUS SIGN
        '\u{2265}' => 0xF2, // GREATER-THAN OR EQUAL TO
        '\u{2264}' => 0xF3, // LESS-THAN OR EQUAL TO
        '\u{2320}' => 0xF4, // TOP HALF INTEGRAL
        '\u{2321}' => 0xF5, // BOTTOM HALF INTEGRAL
        '\u{00F7}' => 0xF6, // DIVISION SIGN
        '\u{2248}' => 0xF7, // ALMOST EQUAL TO
        '\u{00B0}' => 0xF8, // DEGREE SIGN
        '\u{2219}' => 0xF9, // BULLET OPERATOR
        '\u{00B7}' => 0xFA, // MIDDLE DOT
        '\u{221A}' => 0xFB, // SQUARE ROOT
        '\u{207F}' => 0xFC, // SUPERSCRIPT LATIN SMALL LETTER N
        '\u{00B2}' => 0xFD, // SUPERSCRIPT TWO
        '\u{25A0}' => 0xFE, // BLACK SQUARE
        '\u{00A0}' => 0xFF, // NO-BREAK SPACE

        '\u{0394}' => 0x7F,  // Greek capital delta
        '\u{03B2}' => 0xE1,  // Greek small beta
        '\u{03A0}' |         // Greek capital pi
        '\u{220F}' => 0xE3,  // n-ary product sign
        '\u{2211}' => 0xE4,  // n-ary summation sign
        '\u{03BC}' => 0xE6,  // Mu Small
        '\u{00F0}' |         // small eth
        '\u{2202}' => 0xEB,  // partial derivative sign
        '\u{03D5}'  |        // Phi Small (Closed Form)
        '\u{1D719}' |        // Italicized Phi Small (Closed Form)
        '\u{2205}'  |        // empty set sign
        '\u{2300}'  |        // diameter sign
        '\u{00D8}'  |        // Capital Latin letter O with stroke
        '\u{00F8}'  => 0xED, // Lowercase Latin letter O with stroke
        '\u{2208}' |         // element-of sign
        '\u{20AC}' => 0xEE,  // euro sign
        '\u{2713}' => 0xFB,  // check mark

        c => if c.is_ascii() { c as u8 } else { return None; },
    })
}
