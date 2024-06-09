use crate::logical::LogicalSchema;
use crate::models::{ComplexContentEl, ExtensionEl};
use convert_case::{Case, Casing};

pub fn generate(schema: LogicalSchema) -> String {
    let groups = schema
        .groups
        .iter()
        .filter_map(|(name, _group)| {
            let mut str = format!("#[derive(Deserialize)]\npub enum {name} {{\n");
            for (_name, el) in &schema.elements {
                let Some(typ) = &el.typ else {
                    continue;
                };
                if typ == "string" {
                    continue;
                }
                let mut typ = typ.strip_prefix("mx:I").unwrap();
                if typ == "Box" {
                    typ = "MxBox";
                }
                str.push_str(format!("\t{}({}),\n", typ, typ).as_str());
            }
            str.push_str("}\n");
            Some(str)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let elements = schema
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
            let ComplexContentEl::Extension(mut extension) = content.clone() else {
                return None;
            };
            let Some(extensions) = &extension.extensions else {
                return None;
            };
            for ext in extensions {
                let ExtensionEl::Attribute(attr) = ext else {
                    continue;
                };
                let typ = match attr.typ.as_str() {
                    "string" => "Option<String>",
                    _ => panic!("Unknown type: {}", attr.typ),
                };
                let str = format!("#[serde(skip_serializing_if = \"Option::is_none\")]\n\t#[serde(rename = \"@{}\")]\n\tpub {}: {typ},", attr.name, attr.name.to_case(Case::Snake));
                attrs.push(str);
            }

            while let Some(typ) = schema.types.get(extension.base.strip_prefix("mx:").unwrap()) {
                let mut name = typ.name.strip_prefix("I").unwrap();
                if name == "Box" {
                    name = "MxBox";
                }
                let str = format!("#[serde(flatten)]\n\tpub {}: {},", name.to_case(Case::Snake), name);
                attrs.push(str);

                let Some(content) = &typ.complex_content else {
                    break;
                };
                let Some(content) = &content.content else {
                    break;
                };
                extension = match content {
                    ComplexContentEl::Extension(ext) => ext.clone(),
                    _ => break,
                };
            }
            attrs.push(format!("#[serde(rename = \"$value\")]\n\tpub children: Option<Vec<Components>>,"));

            let attrs = attrs.join("\n\t");
            let mut name = el.name.as_ref().unwrap().as_str();
            if name == "Box" {
                name = "MxBox";
            }
            let mut res = format!("#[derive(Deserialize)]\npub struct {} {{\n\t", name);
            res.push_str(attrs.as_str());
            res.push_str("\n}\n");
            Some(res)
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}", groups, elements)
}
