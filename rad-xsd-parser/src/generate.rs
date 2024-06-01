use crate::logical::LogicalSchema;
use crate::models::{AttribGroupEl, ExtensionEl};

pub fn generate(schema: LogicalSchema) -> String {
    schema.elements.iter().map(|(_name, el)| {
        if Some(true) == el.is_abstract {
            return "".to_string();
        }
        let mut attrs = vec![];
        if let Some(typ) = &el.typ {
            let typ = schema.types.get(typ).unwrap();
            for ext in &typ.complexContent.extensions {
                match ext {
                    ExtensionEl::AttributeGroup(grp) => {
                        if let Some(attributes) = &grp.attributes {
                            for attr in attributes {
                                match attr {
                                    AttribGroupEl::AttributeGroup(_) => {}
                                    AttribGroupEl::Attribute(attr) => {
                                        let str = format!("pub {}: {},", attr.name, attr.typ);
                                        attrs.push(str);
                                    }
                                }
                            }
                        }
                    }
                    ExtensionEl::Group(_) => {}
                }
            }
        }
        let attrs = attrs.join("\n");
        let mut res = format!("pub struct {} {{\n", el.name.as_ref().unwrap());
        res.push_str(attrs.as_str());
        res.push_str("}}\n");
        res
    }).collect::<Vec<_>>().join("\n")
}