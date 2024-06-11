use crate::logical::LogicalSchema;
use crate::models::{ComplexContentEl, ComplexType, Element, ExtensionEl};
use convert_case::{Case, Casing};
use std::collections::HashMap;

pub fn generate(schema: LogicalSchema) -> String {
    let groups = schema
        .groups
        .iter()
        .filter_map(|(name, _group)| {
            let mut str = format!("#[derive(Deserialize, Debug, Clone)]\npub enum {name} {{\n");
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

    let elements =
        schema.elements.iter().filter_map(|(_name, el)| generate_struct(&schema, &el)).collect::<Vec<_>>().join("\n");

    format!("{}\n{}", groups, elements)
}

fn generate_struct(schema: &LogicalSchema, el: &Element) -> Option<String> {
    if Some(true) == el.is_abstract {
        return None;
    }
    let Some(typ) = &el.typ else {
        return None;
    };
    let typ = typ.strip_prefix("mx:").unwrap().to_string();
    let mut attrs: Vec<String> = vec![];
    if el.name.as_ref().unwrap().as_str() == "FlexSprite" {
        // attrs.push(format!("#[serde(default)]\n\tpub children: Vec<Components>,"));
    }
    let typ = schema.types.get(&typ).expect(format!("Can't find complexType: {typ}").as_str());
    let Some(content) = &typ.complex_content else {
        return None;
    };
    let Some(content) = &content.content else {
        return None;
    };
    let ComplexContentEl::Extension(extension) = content.clone() else {
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
        let str = format!("\tpub {}: {typ},\n", attr.name.to_case(Case::Snake));
        attrs.push(str);
    }

    if let Some(typ) = schema.types.get(extension.base.strip_prefix("mx:").unwrap()) {
        let mut name = typ.name.strip_prefix("I").unwrap();
        if name == "Box" {
            name = "MxBox";
        }
        let str = format!("pub {}: {},\n", name.to_case(Case::Snake), name);
        attrs.push(str);
    }
    attrs.push(format!("pub children: Vec<Components>,"));

    let attrs = attrs.join("\n\t");
    let mut name = el.name.as_ref().unwrap().as_str();
    if name == "Box" {
        name = "MxBox";
    }
    let mut res = format!("#[derive(Debug, Clone, Default)]\npub struct {} {{\n\t", name);
    res.push_str(attrs.as_str());
    res.push_str("\n}\n\n");

    res.push_str(generate_deserializer(schema, el).as_str());

    Some(res)
}

pub fn recursive_fields(
    schema: &LogicalSchema,
    typ: &ComplexType,
    path: &Vec<String>,
    map: &mut HashMap<String, String>,
) {
    let Some(content) = &typ.complex_content else {
        return;
    };
    let Some(content) = &content.content else {
        return;
    };
    let ComplexContentEl::Extension(extension) = content.clone() else {
        return;
    };
    let Some(extensions) = &extension.extensions else {
        return;
    };

    // push to path
    let mut path = path.clone();
    let mut name = typ.name.strip_prefix("I").unwrap();
    if name == "Box" {
        name = "MxBox";
    }
    path.push(name.to_case(Case::Snake));

    // add own elements
    for ext in extensions {
        let ExtensionEl::Attribute(attr) = ext else {
            continue;
        };
        let path = path.iter().cloned().skip(1).collect::<Vec<_>>().join(".");
        map.insert(attr.name.clone(), path);
    }

    // recurse
    let typ = extension.base.strip_prefix("mx:").unwrap().to_string();
    if let Some(typ) = schema.types.get(&typ) {
        recursive_fields(schema, typ, &path, map);
    }
}

pub fn generate_deserializer(schema: &LogicalSchema, el: &Element) -> String {
    let mut name = el.name.as_ref().unwrap().clone();
    if name == "Box" {
        name = "MxBox".to_string();
    }
    let typ = el.typ.as_ref().unwrap().clone();
    let typ = typ.strip_prefix("mx:").unwrap().to_string();
    let typ = schema.types.get(&typ).expect(format!("Can't find complexType: {typ}").as_str());
    let mut fields = HashMap::new();
    recursive_fields(schema, typ, &vec![], &mut fields);

    let mut str = format!("impl<'de> serde::Deserialize<'de> for {name} {{");
    str.push_str(
        r#"
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
"#,
    );
    str.push_str(format!("\t\t\ttype Value = {name};\n\n").as_str());
    str.push_str(
        r#"            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    "#,
    );
    str.push_str(format!("\t\t\tformatter.write_str(\"struct {name}\")\n").as_str());
    str.push_str("\t\t\t}\n\n");
    str.push_str(format!("\t\t\tfn visit_map<V>(self, mut map: V) -> Result<{name}, V::Error>").as_str());
    str.push_str(
        r#"
                where V: MapAccess<'de>,
            {
"#,
    );
    str.push_str(format!("\t\t\t\tlet mut strct = {name}::default();\n").as_str());
    str.push_str(
        r#"
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
"#,
    );
    for (field, path) in &fields {
        let snake = if path.is_empty() {
            format!("{}", field.to_case(Case::Snake))
        } else {
            format!("{path}.{}", field.to_case(Case::Snake))
        };
        str.push_str(format!("\t\t\t\t\t\t\"@{field}\" => strct.{snake} = Some(map.next_value()?),\n").as_str());
    }
    str.push_str(
        r#"                        _ => { let _: de::IgnoredAny = map.next_value()?; },
                    }
                }
                Ok(strct)
            }
        }
        "#,
    );
    let fields = fields.iter().map(|(k, _v)| format!("\"{k}\"")).collect::<Vec<_>>().join(",\n\t\t\t");
    str.push_str(format!("const FIELDS: &[&str] = &[\n\t\t\t{fields}\n\t\t];\n").as_str());
    str.push_str(format!("\t\tdeserializer.deserialize_struct(\"{name}\", FIELDS, MyVisitor)").as_str());
    str.push_str(
        r#"
    }
}

"#,
    );
    str
}
