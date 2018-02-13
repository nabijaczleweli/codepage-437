use std::borrow::Cow;

mod decode;
mod encode;


const ALL_CP437: &[u8] = include_bytes!("../../test-data/cp437_control/all.cp437");
const ALL_UTF8: &str = include_str!("../../test-data/cp437_control/all.utf8");

const VARIANTS_CP437: &[u8] = include_bytes!("../../test-data/cp437_control/variants.cp437");
const VARIANTS_UTF8: &str = include_str!("../../test-data/cp437_control/variants.utf8");


fn is_borrowed<T: ToOwned + ?Sized>(who: &Cow<T>) -> bool {
    match who {
        &Cow::Borrowed(_) => true,
        &Cow::Owned(_) => false,
    }
}
