use crate::models::{ComplexType, Element, Schema, SchemaElement};
use std::collections::HashMap;

#[derive(Default)]
pub struct LogicalSchema {
    pub types: HashMap<String, ComplexType>,
    pub elements: HashMap<String, Element>,
}

impl From<Schema> for LogicalSchema {
    fn from(schema: Schema) -> Self {
        let mut logical = LogicalSchema::default();
        schema.schema_elements.into_iter().for_each(|el| match el {
            SchemaElement::AttributeGroup(_) => {}
            SchemaElement::ComplexType(typ) => {
                logical.types.insert(typ.name.clone(), typ);
            }
            SchemaElement::Element(el) => {
                if let Some(name) = el.name.clone() {
                    logical.elements.insert(name, el);
                }
            }
            SchemaElement::Group(_) => {}
        });
        logical
    }
}
