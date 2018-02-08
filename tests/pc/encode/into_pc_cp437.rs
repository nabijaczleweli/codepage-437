use self::super::super::{VARIANTS_CP437, VARIANTS_UTF8, ALL_CP437, ALL_UTF8};
use codepage_437::pc::{PcCp437Error, IntoPcCp437};


#[test]
fn good() {
    let everything_utf8 = ALL_UTF8.to_string() + VARIANTS_UTF8;
    let mut everything_cp437 = ALL_CP437.to_vec();
    everything_cp437.extend(VARIANTS_CP437);

    assert_eq!(everything_utf8.into_pc_cp437(), Ok(everything_cp437));
}

#[test]
fn unrepresentable() {
    let err = "Jurek je żurek w żupanie.".to_string().into_pc_cp437().unwrap_err();

    assert_eq!(err.as_str(), "Jurek je żurek w żupanie.");
    assert_eq!(err.pc_cp437_error(), PcCp437Error { representable_up_to: 9 });
    assert_eq!(err.into_string(), "Jurek je żurek w żupanie.");
}
