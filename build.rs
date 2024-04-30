use std::{env, fs};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::parse;
use xsd_parser::parser::types::{RsEntity, RsFile, TupleStruct};

fn main() {
    let input = "resources/radui.xsd";
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={input}");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("models.rs");
    println!("cargo:warning=generating {}", dest_path.display());

    let gen = GeneratorBuilder::default().build();
    let mut rs_file = RsFile::default();
    assert!(gen.generate_rs_file(&rs_file).is_empty());

    rs_file.types.push(RsEntity::TupleStruct(TupleStruct {
        name: "name".to_string(),
        comment: Some("comment".into()),
        type_name: "type".to_string(),
        ..Default::default()
    }));
    let xsd = fs::read(input).expect("Error reading XSD");
    let xsd = String::from_utf8_lossy(xsd.as_slice()).to_string();
    let rs_file = parse(xsd.as_str()).unwrap();
    let gen = GeneratorBuilder::default().build();
    let code = gen.generate_rs_file(&rs_file);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(dest_path)
        .unwrap();
    file.write_all(code.as_bytes()).unwrap();
}
