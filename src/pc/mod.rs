//! Conversion to and from [`cp437_DOSLatinUS`](http://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT).
//!
//! Use the `{Borrow,}FromPcCp437` traits to convert series of cp437 bytes to Unicode,
//!     the `pc_cp437_to_unicode()` function to decode a single codepoint,
//!     and `is_pc_cp437_or_ascii()` to check if a cp437 byte is also valid Unicode.
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


mod decode;

pub use self::decode::{BorrowFromPcCp437, FromPcCp437, is_pc_cp437_or_ascii, pc_cp437_to_unicode};
