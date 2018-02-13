//! Conversion to and from codepage 437.
//!
//! Use the `{Borrow,}FromCp437` traits to convert series of cp437 bytes to Unicode,
//! and the `cp437_to_unicode()` function to decode a single codepoint.
//!
//! Use the `{Into,To}Cp437` traits to convert Unicode to a series of cp437 bytes,
//! and the `unicode_to_cp437()` function to encode a single codepoint.
//!
//! # Examples
//!
//! Borrowing from a buffer:
//!
//! ```
//! # use codepage_437::{CP437_CONTROL, BorrowFromCp437};
//! # use std::borrow::Cow;
//! # /*
//! let data = &[/* buffer acquired somewhere */];
//! # */
//! # let data = &[0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x6E, 0x65, 0x77, 0x73];
//!
//! /// in_unicode will be Cow::Borrowed if data only contains overlapping characters,
//! ///                 or Cow::Owned if a conversion needed to have been made.
//! let in_unicode = Cow::borrow_from_cp437(data, &CP437_CONTROL);
//! # assert_eq!(in_unicode, "Local news");
//!
//! // Also valid:
//! let in_unicode = String::borrow_from_cp437(data, &CP437_CONTROL);
//! # assert_eq!(in_unicode, "Local news");
//! ```
//!
//! Moving out of a buffer:
//!
//! ```
//! # use codepage_437::{CP437_CONTROL, FromCp437};
//! # /*
//! let data = vec![/* buffer moved in from somewhere */];
//! # */
//! # let data = vec![0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x6E, 0x65, 0x77, 0x73];
//!
//! /// data is moved out of and zero-alloced into in_unicode
//! ///      if it only contains overlapping characters
//! let in_unicode = String::from_cp437(data, &CP437_CONTROL);
//! # assert_eq!(in_unicode, "Local news");
//! ```
//!
//! Borrowing from a `&str`:
//!
//! ```
//! # use codepage_437::{CP437_CONTROL, ToCp437};
//! let data = "Some string.";
//!
//! /// in_cp437 will be Cow::Borrowed if data only contains overlapping characters,
//! ///                  Cow::Owned if a conversion needed to have been made,
//! ///               or Err, if data can't be represented as cp437
//! let in_cp437 = data.to_cp437(&CP437_CONTROL);
//! # assert_eq!(in_cp437, Ok([0x53, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x2E][..].into()));
//!
//! // Also valid (String is AsRef<str>):
//! let data = "Some string.".to_string();
//! let in_cp437 = data.to_cp437(&CP437_CONTROL);
//! # assert_eq!(in_cp437, Ok([0x53, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x2E][..].into()));
//! ```
//!
//! Moving out of a `String`:
//!
//! ```
//! # use codepage_437::{CP437_CONTROL, IntoCp437};
//! let data = "Some string.".to_string();
//!
//! /// data is moved out of and zero-alloced into in_cp437
//! ///      if it only contains overlapping characters
//! let in_cp437 = data.into_cp437(&CP437_CONTROL);
//! # assert_eq!(in_cp437, Ok([0x53, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x2E][..].into()));
//! ```
//!
//! Unrepresentable Unicode:
//!
//! ```
//! # use codepage_437::{CP437_CONTROL, ToCp437};
//! // Ż has no representation in cp437
//! let data = "Jurek żelaznym żurkiem żre żupan.";
//!
//! let result = data.to_cp437(&CP437_CONTROL);
//! assert!(result.is_err());
//! // result.unwrap_err() is Cp437Error (or IntoCp437Error for into_cp437()),
//! //   with an API modeled after libstd's {From,}Utf8Error
//! # assert_eq!(result.unwrap_err().representable_up_to, 6);
//! ```


mod decode;
mod encode;
mod dialect;

pub use self::dialect::*;
pub use self::decode::{BorrowFromCp437, FromCp437};
pub use self::encode::{IntoCp437Error, Cp437Error, IntoCp437, ToCp437};
