use self::super::super::super::{VARIANTS_CP437, VARIANTS_UTF8, ALL_CP437, ALL_UTF8};
use codepage_437::pc::{PcCp437Error, ToPcCp437};


#[test]
fn good() {
    let everything_utf8 = ALL_UTF8.to_string() + VARIANTS_UTF8;
    let mut everything_cp437 = ALL_CP437.to_vec();
    everything_cp437.extend(VARIANTS_CP437);

    assert_eq!(everything_utf8.to_pc_cp437(), Ok(everything_cp437[..].into()));
    assert_eq!(everything_utf8[..].to_pc_cp437(), Ok(everything_cp437[..].into()));
}

#[test]
fn unrepresentable() {
    assert_eq!("Jurek je żurek w żupanie.".to_pc_cp437(), Err(PcCp437Error { representable_up_to: 9 }));
}
