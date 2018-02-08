//! Conversion to and from [`cp437_DOSLatinUS`](http://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT).
//!
//! Use the `{Borrow,}FromPcCp437` traits to convert series of cp437 bytes to Unicode,
//! and the `pc_cp437_to_unicode()` function to decode a single codepoint.
//!
//! Use the `{Into,To}PcCp437` traits to convert Unicode to a series of cp437 bytes,
//! and the `unicode_to_pc_cp437()` function to encode a single codepoint.
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
//!
//! Borrowing from a `&str`:
//!
//! ```
//! # use codepage_437::pc::ToPcCp437;
//! let data = "Some string.";
//!
//! /// in_cp437 will be Cow::Borrowed if data only contains overlapping characters,
//! ///                  Cow::Owned if a conversion needed to have been made,
//! ///               or Err, if data can't be represented as cp437
//! let in_cp437 = data.to_pc_cp437();
//! # assert_eq!(in_cp437, Ok([0x53, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x2E][..].into()));
//!
//! // Also valid (String is AsRef<str>):
//! let data = "Some string.".to_string();
//! let in_cp437 = data.to_pc_cp437();
//! # assert_eq!(in_cp437, Ok([0x53, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x2E][..].into()));
//! ```
//!
//! Moving out of a `String`:
//!
//! ```
//! # use codepage_437::pc::IntoPcCp437;
//! let data = "Some string.".to_string();
//!
//! /// data is moved out of and zero-alloced into in_cp437
//! ///      if it only contains overlapping characters
//! let in_cp437 = data.into_pc_cp437();
//! # assert_eq!(in_cp437, Ok([0x53, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67, 0x2E][..].into()));
//! ```
//!
//! Unrepresentable Unicode:
//!
//! ```
//! # use codepage_437::pc::ToPcCp437;
//! // Ż has no representation in cp437
//! let data = "Jurek żelaznym żurkiem żre żupan.";
//!
//! let result = data.to_pc_cp437();
//! assert!(result.is_err());
//! // result.unwrap_err() is PcCp437Error (or IntoPcCp437Error for into_pc_cp437()),
//! //   with an API modeled after libstd's {From,}Utf8Error
//! # assert_eq!(result.unwrap_err().representable_up_to, 6);
//! ```


mod decode;
mod encode;

pub use self::decode::{BorrowFromPcCp437, FromPcCp437, pc_cp437_to_unicode};
pub use self::encode::{IntoPcCp437Error, PcCp437Error, IntoPcCp437, ToPcCp437, unicode_to_pc_cp437};
