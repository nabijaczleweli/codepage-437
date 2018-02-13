use self::super::Cp437Dialect;
use std::borrow::Cow;


/// Errors which can occur when attempting to interpret a string as a sequence of cp437 codepoints.
///
/// As such, the `into_cp437` family of functions and functions make use of this error, for example.
#[derive(Debug, Copy, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Cp437Error {
    /// Returns the index in the given string up to which valid cp437 was verified.
    ///
    /// It is the maximum index such that `input[..index].to_cp_437()` would return `Ok(_)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::{CP437_CONTROL, ToCp437};
    /// // some unrepresentable characters, in a &str
    /// let word = "Eżektor";
    ///
    /// // ToCp437::to_cp437() returns a Cp437Error
    /// let error = word.to_cp437(&CP437_CONTROL).unwrap_err();
    ///
    /// // the second character is unrepresentable here
    /// assert_eq!(error.representable_up_to, 1);
    /// ```
    pub representable_up_to: usize,
}

/// A possible error value when converting a `String` into a cp437 byte vector.
///
/// This type is the error type for the [`into_cp437()`](trait.IntoCp437.html#tymethod.into_cp437)
/// function on [`IntoCp437`](trait.IntoCp437.html). It is designed in such a way to carefully avoid reallocations:
/// the [`into_string()`](#method.into_string) function will give back the String that was used
/// in the conversion attempt.
///
/// The [`Cp437Error`](struct.Cp437Error.html) type represents an error that may
/// occur when converting a `&str` to a sequence of `u8`s. In this sense, it's
/// an analogue to `IntoCp437Error`, and you can get one from a `IntoCp437Error`
/// through the [`cp437_error()`](#method.cp437_error) function.
///
/// # Examples
///
/// ```
/// # use codepage_437::{CP437_CONTROL, IntoCp437};
/// // some unrepresentable chracters, in a String
/// let word = "Eżektor".to_string();
///
/// let value = word.into_cp437(&CP437_CONTROL);
///
/// assert!(value.is_err());
/// assert_eq!(value.unwrap_err().into_string(), "Eżektor".to_string());
/// ```
#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntoCp437Error {
    string: String,
    error: Cp437Error,
}

impl IntoCp437Error {
    /// Returns a `&str` that was attempted to convert to cp437.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::{CP437_CONTROL, IntoCp437};
    /// // some unrepresentable chracters, in a String
    /// let word = "Eżektor".to_string();
    ///
    /// let value = word.into_cp437(&CP437_CONTROL);
    ///
    /// assert_eq!(value.unwrap_err().as_str(), "Eżektor");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.string
    }

    /// Returns the `String` that was attempted to convert to cp437.
    ///
    /// This function is carefully constructed to avoid allocation. It will
    /// consume the error, moving out the string, so that a copy of the string
    /// does not need to be made.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::{CP437_CONTROL, IntoCp437};
    /// // some unrepresentable chracters, in a String
    /// let word = "Eżektor".to_string();
    ///
    /// let value = word.into_cp437(&CP437_CONTROL);
    ///
    /// assert_eq!(value.unwrap_err().into_string(), "Eżektor".to_string());
    /// ```
    pub fn into_string(self) -> String {
        self.string
    }

    /// Fetch a `Cp437Error` to get more details about the conversion failure.
    ///
    /// The [`Cp437Error`](struct.Cp437Error.html) type represents an error that may
    /// occur when converting a `&str` to a sequence of `u8`s. In this sense, it's
    /// an analogue to `IntoCp437Error`. See its documentation for more details
    /// on using it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codepage_437::{CP437_CONTROL, IntoCp437};
    /// // some unrepresentable chracters, in a String
    /// let word = "Eżektor".to_string();
    ///
    /// let error = word.into_cp437(&CP437_CONTROL).unwrap_err().cp437_error();
    ///
    /// // the first character is unrepresentable here
    /// assert_eq!(error.representable_up_to, 1);
    /// ```
    pub fn cp437_error(&self) -> Cp437Error {
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
/// # use codepage_437::{CP437_CONTROL, IntoCp437};
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
/// assert_eq!(unicode.into_cp437(&CP437_CONTROL), Ok(cp437));  // unicode is moved out of
/// ```
///
/// Unrepresentable:
///
/// ```
/// # use codepage_437::{CP437_CONTROL, IntoCp437};
/// // Ż cannot be represented in cp437
/// let unicode = "Jurek je żurek w żupanie.".to_string();
///
/// let error = unicode.into_cp437(&CP437_CONTROL).unwrap_err();  // unicode is moved out of
/// assert_eq!(error.as_str(), "Jurek je żurek w żupanie.");
/// assert_eq!(error.cp437_error().representable_up_to, 9);
///
/// let unicode = error.into_string();                   // unicode now the same as original
/// # assert_eq!(unicode, "Jurek je żurek w żupanie.");
/// ```
pub trait IntoCp437<T> {
    /// Do the conversion.
    fn into_cp437(self, dialect: &Cp437Dialect) -> Result<T, IntoCp437Error>;
}

impl IntoCp437<Vec<u8>> for String {
    fn into_cp437(self, dialect: &Cp437Dialect) -> Result<Vec<u8>, IntoCp437Error> {
        if self.chars().all(|c| dialect.overlap_unicode(c)) {
            Ok(self.into_bytes())
        } else {
            to_cp437_impl_meat(&self, dialect).map_err(|e| {
                IntoCp437Error {
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
/// # use codepage_437::{CP437_CONTROL, ToCp437};
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
/// assert_eq!(unicode.to_cp437(&CP437_CONTROL), Ok(cp437[..].into()));
/// ```
///
/// Unrepresentable:
///
/// ```
/// # use codepage_437::{CP437_CONTROL, ToCp437};
/// // Ż cannot be represented in cp437
/// let unicode = "Jurek je żurek w żupanie.";
///
/// let error = unicode.to_cp437(&CP437_CONTROL).unwrap_err();
/// assert_eq!(error.representable_up_to, 9);
/// ```
pub trait ToCp437<'s, T> {
    /// Do the conversion.
    fn to_cp437(&'s self, dialect: &Cp437Dialect) -> Result<T, Cp437Error>;
}

impl<'s> ToCp437<'s, Cow<'s, [u8]>> for str {
    fn to_cp437(&'s self, dialect: &Cp437Dialect) -> Result<Cow<'s, [u8]>, Cp437Error> {
        to_cp437_cow_impl(&self, dialect)
    }
}

impl<'s, S: AsRef<str>> ToCp437<'s, Cow<'s, [u8]>> for S {
    fn to_cp437(&'s self, dialect: &Cp437Dialect) -> Result<Cow<'s, [u8]>, Cp437Error> {
        to_cp437_cow_impl(self.as_ref(), dialect)
    }
}


fn to_cp437_cow_impl<'c>(whom: &'c str, dialect: &Cp437Dialect) -> Result<Cow<'c, [u8]>, Cp437Error> {
    if whom.chars().all(|c| dialect.overlap_unicode(c)) {
        Ok(Cow::Borrowed(whom.as_bytes()))
    } else {
        to_cp437_impl_meat(whom, dialect).map(Cow::Owned)
    }
}

fn to_cp437_impl_meat(whom: &str, dialect: &Cp437Dialect) -> Result<Vec<u8>, Cp437Error> {
    let mut result = Vec::with_capacity(whom.chars().count());

    for c in whom.chars() {
        if let Some(b) = dialect.encode(c) {
            result.push(b);
        } else {
            return Err(Cp437Error { representable_up_to: result.len() });
        }
    }

    Ok(result)
}
