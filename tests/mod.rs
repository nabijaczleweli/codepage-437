extern crate codepage_437;

use std::borrow::Cow;

mod decode;
mod encode;


const ALL_CP437: &[u8] = include_bytes!("../test-data/pc/all.cp437");
const ALL_UTF8: &str = include_str!("../test-data/pc/all.utf8");

const VARIANTS_CP437: &[u8] = include_bytes!("../test-data/pc/variants.cp437");
const VARIANTS_UTF8: &str = include_str!("../test-data/pc/variants.utf8");


fn is_borrowed<T: ToOwned + ?Sized>(who: &Cow<T>) -> bool {
    match who {
        &Cow::Borrowed(_) => true,
        &Cow::Owned(_) => false,
    }
}
