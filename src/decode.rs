use self::super::Cp437Dialect;
use std::iter::FromIterator;
use std::borrow::Cow;
use std::str;


/// Move data encoded in cp437 to a Unicode container of the specified type.
///
/// # Examples
///
/// ```
/// # use codepage_437::{CP437_CONTROL, FromCp437};
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
/// assert_eq!(String::from_cp437(cp437, &CP437_CONTROL), unicode);  // cp437 is moved out of
/// ```
pub trait FromCp437<T: Sized> {
    /// Do the conversion.
    fn from_cp437(cp437: T, dialect: &Cp437Dialect) -> Self;
}

macro_rules! from_cp437_slice_impl {
    ($($l:expr)*) => ($(
        impl FromCp437<[u8; $l]> for String {
            fn from_cp437(cp437: [u8; $l], dialect: &Cp437Dialect) -> Self {
                from_cp437_slice_impl(&cp437, dialect)
            }
        }
    )*)
}

impl FromCp437<Vec<u8>> for String {
    fn from_cp437(cp437: Vec<u8>, dialect: &Cp437Dialect) -> Self {
        if cp437.iter().all(|&b| dialect.overlap_cp437(b)) {
            String::from_utf8(cp437).unwrap()
        } else {
            String::from_iter(cp437.into_iter().map(|b| dialect.decode(b)))
        }
    }
}

impl FromCp437<[u8; 0]> for String {
    fn from_cp437(_: [u8; 0], _: &Cp437Dialect) -> Self {
        String::new()
    }
}

from_cp437_slice_impl!(    1  2  3  4  5  6  7  8  9
                          10 11 12 13 14 15 16 17 18 19
                          20 21 22 23 24 25 26 27 28 29
                          30 31 32);

fn from_cp437_slice_impl(cp437: &[u8], dialect: &Cp437Dialect) -> String {
    if cp437.iter().all(|&b| dialect.overlap_cp437(b)) {
        String::from_utf8(cp437.to_vec()).unwrap()
    } else {
        String::from_iter(cp437.iter().map(|&b| dialect.decode(b)))
    }
}


/// Try to borrow data encoded in cp437 as a Unicode container of the specified type.
///
/// If that cannot be done, clone it.
///
/// # Examples
///
/// ```
/// # use codepage_437::{CP437_CONTROL, BorrowFromCp437};
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
/// assert_eq!(Cow::borrow_from_cp437(&cp437[..], &CP437_CONTROL),
///            String::borrow_from_cp437(&cp437[..], &CP437_CONTROL));
/// assert_eq!(Cow::borrow_from_cp437(&cp437[..], &CP437_CONTROL), unicode);
/// ```
pub trait BorrowFromCp437<'c, T: ?Sized> {
    /// Do the conversion.
    fn borrow_from_cp437(cp437: &'c T, dialect: &Cp437Dialect) -> Self;
}

impl<'c, T: AsRef<[u8]> + ?Sized> BorrowFromCp437<'c, T> for Cow<'c, str> {
    fn borrow_from_cp437(cp437: &'c T, dialect: &Cp437Dialect) -> Self {
        borrow_from_cp437_cow_slice_impl(cp437.as_ref(), dialect)
    }
}

impl<'c, T: AsRef<[u8]> + ?Sized> BorrowFromCp437<'c, T> for String {
    fn borrow_from_cp437(cp437: &'c T, dialect: &Cp437Dialect) -> Self {
        borrow_from_cp437_string_slice_impl(cp437.as_ref(), dialect)
    }
}

fn borrow_from_cp437_cow_slice_impl<'c>(cp437: &'c [u8], dialect: &Cp437Dialect) -> Cow<'c, str> {
    if cp437.iter().all(|&b| dialect.overlap_cp437(b)) {
        Cow::Borrowed(str::from_utf8(&cp437[..]).unwrap())
    } else {
        Cow::Owned(String::from_iter(cp437.iter().map(|&b| dialect.decode(b))))
    }
}

fn borrow_from_cp437_string_slice_impl(cp437: &[u8], dialect: &Cp437Dialect) -> String {
    if cp437.iter().all(|&b| dialect.overlap_cp437(b)) {
        String::from_utf8(cp437.to_vec()).unwrap()
    } else {
        String::from_iter(cp437.iter().map(|&b| dialect.decode(b)))
    }
}
