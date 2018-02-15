extern crate codepage_437;

mod cp437_wingdings;
mod cp437_control;
mod dialect;


const ALL_CP437: &[u8] = include_bytes!("../test-data/all.cp437");
