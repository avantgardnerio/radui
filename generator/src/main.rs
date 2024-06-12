use as3_parser::compilation_unit::CompilationUnit;
use as3_parser::ns::{Attribute, Expression, FunctionName, QualifiedIdentifierIdentifier};
use as3_parser::parser::ParserFacade;
use as3_parser::tree::Directive;
use glob::glob;
use itertools::Itertools;
use quick_xml::se::Serializer;
use rad_xsd_parser::models;
use rad_xsd_parser::models::{
    Annotation, Choice, ChoiceOption, ComplexContent, ComplexContentEl, ComplexType, ComplexTypeEl, Element, Extension,
    ExtensionEl, Group, Schema, SchemaElement, Sequence, SequenceEl,
};
use serde::Serialize;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter::once;

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub extends: Option<String>,
    pub setters: HashMap<String, Setter>,
}

#[derive(Debug, Clone)]
pub struct Setter {
    name: String,
    typ: String,
    doc: Option<String>,
}

fn main() {
    let home = dirs::home_dir().unwrap();
    let home = home.join("workspace/flex-sdk/frameworks/projects/**/*.as");
    let home = home.to_str().unwrap();
    println!("home={home:?}");

    // load
    let mut classes = load_classes(home);

    // convert to mem
    let mut export = HashSet::<String>::new();
    let class_names = ["VBox", "HBox", "Label", "DataGrid", "TitleWindow", "WindowedApplication"];
    for class_name in class_names {
        add_class(class_name, &mut classes, &mut export);
    }

    // convert to xml
    let mut schema = Schema {
        schema_elements: vec![],
        target_namespace: "http://www.macromedia.com/2003/mxml".to_string(),
        mx: "http://www.macromedia.com/2003/mxml".to_string(),
        xmlns: "http://www.w3.org/2001/XMLSchema".to_string(),
        element_form_default: "qualified".to_string(),
        attribute_form_default: "unqualified".to_string(),
    };
    let group = Group { name: None, choice: None, reference: Some("mx:Components".to_string()) };
    let seq = Sequence {
        elements: vec![SequenceEl::Group(group)],
        min_occurs: Some("0".to_string()),
        max_occurs: Some("unbounded".to_string()),
    };
    let mut group = vec![];
    for class_name in &export {
        let class = classes.get(class_name).unwrap();
        let mut value: Option<Vec<ComplexTypeEl>> = None;
        let mut complex_content = None;
        let attributes: Vec<models::Attribute> = class
            .setters
            .iter()
            .map(|(name, setter)| models::Attribute {
                name: name.clone(),
                typ: setter.typ.clone(),
                annotation: setter.doc.as_ref().map(|doc| Annotation { documentation: doc.clone() }),
            })
            .collect();
        if let Some(parent) = &class.extends {
            let extensions = once(ExtensionEl::Sequence(seq.clone()))
                .chain(attributes.into_iter().map(|attr| ExtensionEl::Attribute(attr)))
                .collect::<Vec<_>>();
            let extension = Extension { base: format!("mx:I{parent}"), extensions: Some(extensions) };
            let content = Some(ComplexContentEl::Extension(extension));
            complex_content = Some(ComplexContent { content });
        } else {
            let children: Vec<_> = once(ComplexTypeEl::Sequence(seq.clone()))
                .chain(attributes.into_iter().map(|attr| ComplexTypeEl::Attribute(attr)))
                .collect();
            value = Some(children);
        }
        let typ = ComplexType { mixed: false, name: format!("I{}", class_name), complex_content, value };
        let el = Element {
            name: Some(class_name.clone()),
            reference: None,
            typ: Some(format!("mx:{}", typ.name.clone())),
            is_abstract: None,
            substitution_group: None,
        };
        group.push(ChoiceOption::Element(el.clone()));
        schema.schema_elements.push(SchemaElement::ComplexType(typ));
        schema.schema_elements.push(SchemaElement::Element(el));
        println!("{class_name} {:?}", classes.get(class_name).map(|c| &c.setters));
    }
    let typ = ComplexType { mixed: false, name: "ISprite".to_string(), complex_content: None, value: None };
    schema.schema_elements.push(SchemaElement::ComplexType(typ));
    let group = SchemaElement::Group(Group {
        name: Some("Components".to_string()),
        choice: Some(Choice { options: Some(group) }),
        reference: None,
    });
    schema.schema_elements.push(group);

    // save
    let mut buffer = String::new();
    let mut ser = Serializer::new(&mut buffer);
    ser.expand_empty_elements(false);
    ser.indent('\t', 1);
    schema.serialize(ser).unwrap();

    let mut output = File::create("radui/resources/mxml.xsd").unwrap();
    output.write_all(&buffer.as_bytes()).unwrap();
}

fn add_class(name: &str, classes: &mut HashMap<String, Class>, exports: &mut HashSet<String>) {
    let parent = if let Some(class) = classes.get_mut(name) {
        exports.insert(name.to_string());
        if let Some(parent) = &class.extends {
            parent.clone()
        } else {
            return;
        }
    } else {
        return;
    };

    if let Some(parent) = classes.get(&parent).map(|p| p.name.clone()) {
        add_class(parent.as_str(), classes, exports);
    }
}

fn load_classes(home: &str) -> HashMap<String, Class> {
    let white_list = HashMap::from([
        ("int", "int"),
        ("uint", "uint"),
        ("String", "string"),
        ("Number", "double"),
        ("Boolean", "boolean"),
    ]);
    let black_list = [
        "accessibility",
        "rotation",
        "transform",
        "effect",
        "automation",
        "skin",
        "framerate",
        "projection",
        "blend",
        "depth",
        "alpha",
        "flag",
        "focus",
        "enabled",
        "click",
        "clip",
        "policy",
        "creat",
        "drag",
        "drop",
        "cache",
        "deferred",
        "front",
        "state",
        "url",
        "tip",
        "nest",
        "tab",
        "z",
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

                let mut getter_doc = defn.block.directives.iter()
                    .filter_map(|directive| {
                        let Directive::FunctionDefinition(func) = directive.as_ref() else {
                            return None;
                        };
                        let FunctionName::Getter((name, _)) = &func.name else {
                            return None;
                        };
                        let Some(doc) = func.asdoc.clone() else {
                            return None;
                        };
                        let Some((doc, _)) = &doc.main_body else {
                            return None;
                        };
                        Some((name.clone(), doc.clone()))
                    }).collect::<HashMap<String, String>>();
                let mut setters = HashMap::new();
                for directive in defn.block.directives.iter() {
                    let Directive::FunctionDefinition(func) = directive.as_ref() else {
                        continue;
                    };
                    let bad = func
                        .attributes
                        .iter()
                        .filter(|attr| {
                            match attr {
                                Attribute::Private(_) => true,
                                Attribute::Protected(_) => true,
                                Attribute::Internal(_) => true,
                                Attribute::Static(_) => true,
                                // Attribute::Override(_) => true,
                                _ => false,
                            }
                        })
                        .last()
                        .is_some();
                    if bad {
                        continue;
                    }
                    let FunctionName::Setter((name, _)) = &func.name else {
                        continue;
                    };
                    if name.starts_with("$") {
                        continue;
                    }
                    let Ok(param) = func.common.signature.parameters.iter().exactly_one() else {
                        continue;
                    };
                    let typ = param.destructuring.type_annotation.as_ref().unwrap();
                    let Expression::QualifiedIdentifier(id) = &**typ else {
                        continue;
                    };
                    let QualifiedIdentifierIdentifier::Id((typ, _)) = &id.id else {
                        continue;
                    };
                    let Some(typ) = white_list.get(typ.as_str()) else {
                        println!("Skipping {class_name}.{name}:{typ}");
                        continue;
                    };
                    let blacked_out =
                        black_list.iter().filter(|term| name.to_lowercase().contains(*term)).last().is_some();
                    if blacked_out {
                        continue;
                    }
                    let mut doc =
                        func.asdoc.as_ref().map(|doc| doc.main_body.as_ref().map(|body| body.0.clone())).flatten();
                    if doc.is_none() {
                        doc = getter_doc.get(name.as_str()).cloned();
                    }
                    let setter = Setter { name: name.clone(), typ: typ.to_string(), doc };
                    setters.insert(name.clone(), setter);
                }
                let class = Class { name: class_name.clone(), extends, setters };
                classes.insert(class_name, class);
            }
        }
    }
    classes
}
