use std::{fs};
use std::collections::{HashMap, HashSet};
use as3_parser::compilation_unit::CompilationUnit;
use as3_parser::ns::{Attribute, Expression, FunctionName, QualifiedIdentifierIdentifier};
use as3_parser::parser::ParserFacade;
use as3_parser::tree::Directive;
use glob::glob;
use rad_xsd_parser::models::Schema;

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub extends: Option<String>,
    pub setters: HashSet<String>,
}

fn main() {
    let home = dirs::home_dir().unwrap();
    let home = home.join("workspace/flex-sdk/frameworks/projects/**/*.as");
    let home = home.to_str().unwrap();
    println!("home={home:?}");

    let mut classes = load_classes(home);

    let mut export = HashSet::<String>::new();
    let class_names = ["VBox", "HBox", "Label", "DataGrid"];
    for mut class_name in class_names {
        let _ = add_class(class_name, &mut classes, &mut export);
    }

    let mut schema = Schema { schema_elements: vec![] };
    for class_name in &export {
        println!("{class_name} {:?}", classes.get(class_name).map(|c| &c.setters));
    }
}

fn add_class(
    name: &str,
    classes: &mut HashMap<String, Class>,
    exports: &mut HashSet<String>
) -> HashSet<String> {
    let parent = if let Some(class) = classes.get_mut(name) {
        exports.insert(name.to_string());
        if let Some(parent) = &class.extends {
            parent.clone()
        } else {
            return class.setters.clone(); // no parent: my props
        }
    } else {
        return HashSet::new(); // not found: no props
    };

    let mut parent_props = HashSet::new();
    if let Some(parent) = classes.get(&parent).map(|p| p.name.clone()) {
        parent_props = add_class(parent.as_str(), classes, exports);
    }
    
    let mut class = classes.get_mut(name).unwrap();
    class.setters.retain(|x| !parent_props.contains(x));
    parent_props.extend(class.setters.clone());
    parent_props
}

fn load_classes(home: &str) -> HashMap<String, Class> {
    let black_list = ["accessibility", "rotation", "creat", "focus", "auto", "data", "drag",
        "style", "cache", "drop", "valid", "effect", "matrix", "transform", "render", "count",
        "editor", "tween", "enabled", "manager", "transition", "project", "repeater", "layout",
        "select", "columns", "initialized", "processed", "flex", "tool", "depth", "nest",
        "factory", "layer", "flag", "button", "icon", "descriptor", "document", "state",
        "indices", "error", "center", "owner", "baseline", "scale", "visible", "alpha", "blend",
        "filters", "measured", "percent", "explicit", "min", "max", "z",
    ];
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

                let mut setters = HashSet::new();
                for directive in defn.block.directives.iter() {
                    let Directive::FunctionDefinition(func) = directive.as_ref() else {
                        continue;
                    };
                    let bad = func.attributes.iter().filter(|attr| {
                        match attr {
                            Attribute::Private(_) => true,
                            Attribute::Protected(_) => true,
                            Attribute::Internal(_) => true,
                            Attribute::Static(_) => true,
                            // Attribute::Override(_) => true,
                            _ => false,
                        }
                    }).last().is_some();
                    if bad {
                        continue;
                    }
                    match &func.name {
                        FunctionName::Identifier(_) => {}
                        FunctionName::Getter(_) => {}
                        FunctionName::Setter((name, _)) => {
                            if name.starts_with("$") {
                                continue;
                            }
                            let blacked_out = black_list
                                .iter()
                                .filter(|term| name.to_lowercase().contains(*term))
                                .last()
                                .is_some();
                            if blacked_out {
                                continue;
                            }
                            setters.insert(name.clone());
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
