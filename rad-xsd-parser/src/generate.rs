use crate::logical::LogicalSchema;
use crate::models::{AttribGroupEl, ExtensionEl};

pub fn generate(schema: LogicalSchema) -> String {
    schema
        .elements
        .iter()
        .map(|(_name, el)| {
            if Some(true) == el.is_abstract {
                return "".to_string();
            }
            let Some(typ) = &el.typ else {
                return "".to_string();
            };
            let typ = schema.types.get(typ).unwrap();
            let mut attrs = vec![];
            let Some(content) = &typ.complex_content else {
                return "".to_string();
            };
            let Some(extension) = &content.extension else {
                return "".to_string();
            };
            let Some(extension) = &extension.extensions else {
                return "".to_string();
            };
            for ext in extension {
                let ExtensionEl::AttributeGroup(grp) = ext else {
                    continue;
                };
                let Some(attributes) = &grp.attributes else {
                    continue;
                };
                for attr in attributes {
                    let AttribGroupEl::Attribute(attr) = attr else {
                        continue;
                    };
                    let str = format!("pub {}: {},", attr.name, attr.typ);
                    attrs.push(str);
                }
            }
            let attrs = attrs.join("\n");
            let mut res = format!("pub struct {} {{\n", el.name.as_ref().unwrap());
            res.push_str(attrs.as_str());
            res.push_str("}}\n");
            res
        })
        .collect::<Vec<_>>()
        .join("\n")
}
