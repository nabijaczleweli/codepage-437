extern crate csv;

use std::io::{BufReader, BufRead, Write};
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::char;
use std::env;


#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Mapping {
    cp437: u8,
    unicode: char,
    comment: String,
}

impl Mapping {
    pub fn from_record(record: csv::StringRecord) -> Result<Mapping, String> {
        if record.len() != 3 {
            return Err(format!("Invalid record length ({}, should be 3)", record.len()));
        }

        let (cp437, unicode, comment) = (record.get(0).unwrap(), record.get(1).unwrap(), record.get(2).unwrap());

        if &cp437[..2] != "0x" {
            return Err(format!("Invalid cp437 code prefix (\"{}\", should be \"0x\")", &cp437[..2]));
        }
        let cp437 = &cp437[2..];
        if !cp437.chars().all(|c| (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f')) {
            return Err(format!("cp437 code \"0x{}\" not hex", cp437));
        }
        if cp437.chars().count() > 2 {
            return Err(format!("cp437 code \"0x{}\" too big", cp437));
        }
        let cp437 = u8::from_str_radix(cp437, 16).unwrap();

        if &unicode[..2] != "0x" {
            return Err(format!("Invalid Unicode code prefix (\"{}\", should be \"0x\")", &unicode[..2]));
        }
        let unicode = &unicode[2..];
        if !unicode.chars().all(|c| (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f')) {
            return Err(format!("Unicode code \"0x{}\" not hex", unicode));
        }
        if unicode.chars().count() > 8 {
            return Err(format!("Unicode code \"0x{}\" too big", unicode));
        }
        let unicode = u32::from_str_radix(unicode, 16).unwrap();
        let unicode = if let Some(unicode) = char::from_u32(unicode) {
            unicode
        } else {
            return Err(format!("Unicode code 0x{:X} out of range", unicode));
        };

        Ok(Mapping {
            cp437: cp437,
            unicode: unicode,
            comment: comment.to_string(),
        })
    }

    pub fn from_mappings<P: AsRef<Path>>(p: P) -> Vec<Mapping> {
        let mut ret = Vec::new();
        for record in csv::ReaderBuilder::new().delimiter('\t' as u8).from_path(p).unwrap().into_records().map(Result::unwrap) {
            ret.push(Mapping::from_record(record).unwrap());
        }
        ret
    }
}


fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var nonexistant/non-Unicode");
    let mut specs_rs = File::create(PathBuf::from(format!("{}/dialects.rs", out_dir))).unwrap();

    for dir in fs::read_dir("dialect-specs").unwrap().map(Result::unwrap).filter(|f| f.file_type().unwrap().is_dir()) {
        let dialect_name_func = dir.file_name().to_str().unwrap().to_lowercase();
        let dialect_name_type = dir.file_name().to_str().unwrap().to_uppercase();

        let cp437_overlap_func = format!("{}_cp437_overlaps", dialect_name_func);
        let unicode_overlap_func = format!("{}_unicode_overlaps", dialect_name_func);
        let decode_func = format!("{}_decode", dialect_name_func);
        let encode_func = format!("{}_encode", dialect_name_func);

        let values_tsv = dir.path().join("values.tsv");
        let variants_tsv = dir.path().join("variants.tsv");
        let documentation_md = dir.path().join("documentation.md");
        let overlaps_rs = dir.path().join("overlaps.rs");

        println!("cargo:rerun-if-changed={}", values_tsv.display());
        println!("cargo:rerun-if-changed={}", variants_tsv.display());
        println!("cargo:rerun-if-changed={}", documentation_md.display());
        println!("cargo:rerun-if-changed={}", overlaps_rs.display());

        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "// {} start", dir.path().display()).unwrap();
        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "").unwrap();

        for line in BufReader::new(File::open(&overlaps_rs).unwrap()).lines().map(Result::unwrap) {
            if line.contains("DIALECT_OVERLAP_CP437") || line.contains("DIALECT_OVERLAP_UNICODE") {
                specs_rs.write_all(line.replace("DIALECT_OVERLAP_CP437", &cp437_overlap_func)
                        .replace("DIALECT_OVERLAP_UNICODE", &unicode_overlap_func)
                        .as_bytes())
                    .unwrap();
            } else {
                specs_rs.write_all(line.as_bytes()).unwrap();
            }
            writeln!(specs_rs).unwrap();
        }

        let primary_mappings = Mapping::from_mappings(&values_tsv);
        let variant_mappings = Mapping::from_mappings(&variants_tsv);

        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "fn {}(cp437: u8) -> char {{", decode_func).unwrap();
        writeln!(specs_rs, "\tmatch cp437 {{").unwrap();
        for &Mapping { cp437, unicode, ref comment } in &primary_mappings {
            writeln!(specs_rs, "\t\t0x{:X} => \'\\u{{{:06X}}}\',  // {}", cp437, unicode as u32, comment).unwrap();
        }
        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "\t\tb => b as char,").unwrap();
        writeln!(specs_rs, "\t}}").unwrap();
        writeln!(specs_rs, "}}").unwrap();
        writeln!(specs_rs, "").unwrap();

        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "fn {}(unicode: char) -> Option<u8> {{", encode_func).unwrap();
        writeln!(specs_rs, "\tSome(match unicode {{").unwrap();
        for &mapp in &[&primary_mappings, &variant_mappings] {
            for &Mapping { cp437, unicode, ref comment } in mapp {
                writeln!(specs_rs, "\t\t\'\\u{{{:06X}}}\' => 0x{:X},  // {}", unicode as u32, cp437, comment).unwrap();
            }
            writeln!(specs_rs, "").unwrap();
        }
        writeln!(specs_rs, "\t\tc => if {}(c) {{ c as u8 }} else {{ return None }},", unicode_overlap_func).unwrap();
        writeln!(specs_rs, "\t}})").unwrap();
        writeln!(specs_rs, "}}").unwrap();
        writeln!(specs_rs, "").unwrap();

        for line in BufReader::new(File::open(&documentation_md).unwrap()).lines().map(Result::unwrap) {
            writeln!(specs_rs, "/// {}", line).unwrap();
        }

        writeln!(specs_rs, "pub static {}: Cp437Dialect = Cp437Dialect {{", dialect_name_type).unwrap();
        writeln!(specs_rs, "\toverlap_unicode: {},", unicode_overlap_func).unwrap();
        writeln!(specs_rs, "\toverlap_cp437: {},", cp437_overlap_func).unwrap();
        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "\tdecode: {},", decode_func).unwrap();
        writeln!(specs_rs, "\tencode: {},", encode_func).unwrap();
        writeln!(specs_rs, "}};").unwrap();

        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "// {} end", dir.path().display()).unwrap();
        writeln!(specs_rs, "").unwrap();
        writeln!(specs_rs, "").unwrap();
    }
}
