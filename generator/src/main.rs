use std::{env, fs};
use as3_parser::compilation_unit::CompilationUnit;
use as3_parser::parser::ParserFacade;
use glob::glob;

fn main() {
    let home = dirs::home_dir().unwrap();
    let home = home.join("workspace/flex-sdk/frameworks/projects/framework/src/mx/core/*.as");
    let home = home.to_str().unwrap();
    println!("home={home:?}");

    for e in glob(home).expect("Failed to read glob pattern") {
        let source_path = e.unwrap();
        println!("{}", source_path.display());
        let source_content = fs::read_to_string(source_path.clone()).unwrap();
        let source_path = source_path.to_str().unwrap().to_string();
        let compilation_unit = CompilationUnit::new(Some(source_path), source_content);
        let program = ParserFacade(&compilation_unit, Default::default()).parse_program();
    }
}
