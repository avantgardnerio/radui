use crate::logical::LogicalSchema;
use crate::models::{AttribGroupEl, ComplexContentEl, ExtensionEl};

pub fn generate(schema: LogicalSchema) -> String {
    schema
        .elements
        .iter()
        .filter_map(|(_name, el)| {
            if Some(true) == el.is_abstract {
                return None;
            }
            let Some(typ) = &el.typ else {
                return None;
            };
            let typ = typ.strip_prefix("mx:").unwrap().to_string();
            let typ = schema.types.get(&typ).expect(format!("Can't find complexType: {typ}").as_str());
            let mut attrs = vec![];
            let Some(content) = &typ.complex_content else {
                return None;
            };
            let Some(content) = &content.content else {
                return None;
            };
            let ComplexContentEl::Extension(extension) = content else {
                return None;
            };
            let Some(extensions) = &extension.extensions else {
                return None;
            };
            for ext in extensions {
                let ExtensionEl::AttributeGroup(grp) = ext else {
                    continue;
                };
                let Some(reference) = &grp.reference else {
                    continue;
                };
                let grp = reference.strip_prefix("mx:").unwrap().to_string();
                let grp =
                    schema.attribute_groups.get(&grp).expect(format!("Can't find attribute_group: {grp}").as_str());
                let Some(attributes) = &grp.attributes else {
                    continue;
                };
                for attr in attributes {
                    let AttribGroupEl::Attribute(attr) = attr else {
                        continue;
                    };
                    let typ = match attr.typ.as_str() {
                        "string" => "String",
                        _ => panic!("Unknown type: {}", attr.typ),
                    };
                    let str = format!("pub {}: {typ},", attr.name);
                    attrs.push(str);
                }
            }
            let attrs = attrs.join("\n\t");
            let mut res = format!("pub struct {} {{\n\t", el.name.as_ref().unwrap());
            res.push_str(attrs.as_str());
            res.push_str("\n}\n");
            Some(res)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
