use std::{fs};
use std::collections::{HashMap, HashSet};
use as3_parser::compilation_unit::CompilationUnit;
use as3_parser::ns::{Expression, FunctionName, QualifiedIdentifierIdentifier};
use as3_parser::parser::ParserFacade;
use as3_parser::tree::Directive;
use glob::glob;

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub extends: Option<String>,
    pub setters: Vec<String>,
}

fn main() {
    let home = dirs::home_dir().unwrap();
    let home = home.join("workspace/flex-sdk/frameworks/projects/**/*.as");
    let home = home.to_str().unwrap();
    println!("home={home:?}");

    let classes = load_classes(home);

    let mut export = HashSet::<String>::new();
    let class_names = ["VBox", "HBox", "DataGrid"];
    for mut class_name in class_names {
        while let Some(class) = classes.get(class_name) {
            export.insert(class_name.to_string());
            println!("{} extends {:?}", class_name, class.extends);
            println!("{:?}", class.setters);
            if let Some(parent) = &class.extends {
                class_name = &parent.as_str();
            } else {
                break;
            }
        }
    }
}

fn load_classes(home: &str) -> HashMap::<String, Class>{
    let black_list = ["accessibility", "rotation"];
    let mut classes = HashMap::<String, Class>::new();
    for e in glob(home).expect("Failed to read glob pattern") {
        let source_path = e.unwrap();
        let source_content = fs::read_to_string(source_path.clone()).unwrap();
        let source_path = source_path.to_str().unwrap().to_string();
        let compilation_unit = CompilationUnit::new(Some(source_path), source_content);
        let program = ParserFacade(&compilation_unit, Default::default()).parse_program();

        for package in program.packages.iter() {
            for directive in package.block.directives.iter() {
                let Directive::ClassDefinition(defn) = directive.as_ref() else {
                    continue;
                };
                let class_name = defn.name.0.to_string();
                let mut extends = None;
                if let Some(ext) = &defn.extends_clause {
                    if let Expression::QualifiedIdentifier(id) = ext.as_ref() {
                        if let QualifiedIdentifierIdentifier::Id((name, _)) = &id.id {
                            extends = Some(name.clone());
                        }
                    }
                }

                let mut setters = vec![];
                for directive in defn.block.directives.iter() {
                    let Directive::FunctionDefinition(func) = directive.as_ref() else {
                        continue;
                    };
                    match &func.name {
                        FunctionName::Identifier(_) => {}
                        FunctionName::Getter(_) => {}
                        FunctionName::Setter((name, _)) => {
                            if name.starts_with("$") {
                                continue;
                            }
                            let blacked_out = black_list
                                .iter()
                                .filter(|term| name.contains(*term))
                                .last()
                                .is_some();
                            if blacked_out {
                                continue;
                            }
                            setters.push(name.clone());
                        },
                        FunctionName::Constructor(_) => {}
                    }
                }
                let class = Class {
                    name: class_name.clone(),
                    extends,
                    setters,
                };
                classes.insert(class_name, class);
            }
        }
    }
    classes
}
