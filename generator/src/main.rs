use std::{env, fs};
use as3_parser::compilation_unit::CompilationUnit;
use as3_parser::ns::FunctionName;
use as3_parser::parser::ParserFacade;
use as3_parser::tree::Directive;
use glob::glob;

fn main() {
    let home = dirs::home_dir().unwrap();
    let home = home.join("workspace/flex-sdk/frameworks/projects/mx/src/mx/**/*.as");
    let home = home.to_str().unwrap();
    println!("home={home:?}");

    for e in glob(home).expect("Failed to read glob pattern") {
        let source_path = e.unwrap();
        println!("{}", source_path.display());
        let source_content = fs::read_to_string(source_path.clone()).unwrap();
        let source_path = source_path.to_str().unwrap().to_string();
        let compilation_unit = CompilationUnit::new(Some(source_path), source_content);
        let program = ParserFacade(&compilation_unit, Default::default()).parse_program();

        for package in program.packages.iter() {
            for directive in package.block.directives.iter() {
                match directive.as_ref() {
                    Directive::ClassDefinition(defn) => {
                        for directive in defn.block.directives.iter() {
                            match directive.as_ref() {
                                Directive::FunctionDefinition(func) => {
                                    match &func.name {
                                        FunctionName::Identifier(_) => {}
                                        FunctionName::Getter(_) => {}
                                        FunctionName::Setter((name, loc)) => {
                                            println!("set {name}");
                                        }
                                        FunctionName::Constructor(_) => {}
                                    }
                                }
                                _ => {}
                            }
                            // if print {
                            //     println!("{directive:#?}\n");
                            // }
                        }
                    },

                    _ => {},
                }
            }
        }

    }
}
