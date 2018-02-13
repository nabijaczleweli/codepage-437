use codepage_437::{CP437_WINGDINGS, BorrowFromCp437};
use self::super::super::super::super::ALL_CP437;
use self::super::super::super::ALL_UTF8;
use std::borrow::Cow;


macro_rules! array_test {
	($test_name:ident, $sz:expr) => {
		#[test]
		fn $test_name() {
			let mut buf = [0u8; $sz];
			buf.copy_from_slice(&ALL_CP437[..$sz]);

			let expected: String = ALL_UTF8.chars().take($sz).collect();
			assert_eq!(Cow::borrow_from_cp437(&buf, &CP437_WINGDINGS), expected);
			assert_eq!(String::borrow_from_cp437(&buf, &CP437_WINGDINGS), expected);
		}
	}
}


#[test]
fn slice() {
    assert_eq!(Cow::borrow_from_cp437(ALL_CP437, &CP437_WINGDINGS), ALL_UTF8);
    assert_eq!(String::borrow_from_cp437(ALL_CP437, &CP437_WINGDINGS), ALL_UTF8);
}

#[test]
fn vec() {
    assert_eq!(Cow::borrow_from_cp437(&ALL_CP437.to_vec(), &CP437_WINGDINGS), ALL_UTF8);
    assert_eq!(String::borrow_from_cp437(&ALL_CP437.to_vec(), &CP437_WINGDINGS), ALL_UTF8);
}


array_test!(array_0, 0);
array_test!(array_1, 1);
array_test!(array_2, 2);
array_test!(array_3, 3);
array_test!(array_4, 4);
array_test!(array_5, 5);
array_test!(array_6, 6);
array_test!(array_7, 7);
array_test!(array_8, 8);
array_test!(array_9, 9);
array_test!(array_10, 10);
array_test!(array_11, 11);
array_test!(array_12, 12);
array_test!(array_13, 13);
array_test!(array_14, 14);
array_test!(array_15, 15);
array_test!(array_16, 16);
array_test!(array_17, 17);
array_test!(array_18, 18);
array_test!(array_19, 19);
array_test!(array_20, 20);
array_test!(array_21, 21);
array_test!(array_22, 22);
array_test!(array_23, 23);
array_test!(array_24, 24);
array_test!(array_25, 25);
array_test!(array_26, 26);
array_test!(array_27, 27);
array_test!(array_28, 28);
array_test!(array_29, 29);
array_test!(array_30, 30);
array_test!(array_31, 31);
array_test!(array_32, 32);
