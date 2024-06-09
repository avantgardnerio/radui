use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use quick_xml::de::from_str;
use rad_xsd_parser::generate::generate;
use rad_xsd_parser::logical::LogicalSchema;
use rad_xsd_parser::models::Schema;

fn main() {
    let input = "resources/mxml.xsd";
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={input}");

    let dest_path = "src/generated/models.rs";
    // println!("cargo:warning=generating {}", dest_path); // example log statement

    let content = fs::read_to_string(input).expect("Unable to read file");
    let schema: Schema = from_str(&*content).unwrap();
    let logical: LogicalSchema = schema.into();
    let code = generate(logical);

    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(dest_path).unwrap();

    let imports = r#"use serde::Deserialize;

#[derive(Deserialize)]
pub struct Sprite {}

"#;
    file.write_all(imports.as_bytes()).unwrap();
    file.write_all(code.as_bytes()).unwrap();
}
