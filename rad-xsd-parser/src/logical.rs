use crate::models::{AttributeGroup, ComplexType, Element, Group, Schema, SchemaElement};
use std::collections::HashMap;

#[derive(Default)]
pub struct LogicalSchema {
    pub attribute_groups: HashMap<String, AttributeGroup>,
    pub types: HashMap<String, ComplexType>,
    pub elements: HashMap<String, Element>,
    pub groups: HashMap<String, Group>,
}

impl From<Schema> for LogicalSchema {
    fn from(schema: Schema) -> Self {
        let mut logical = LogicalSchema::default();
        schema.schema_elements.into_iter().for_each(|el| match el {
            SchemaElement::AttributeGroup(grp) => {
                if let Some(name) = grp.name.clone() {
                    logical.attribute_groups.insert(name, grp);
                }
            }
            SchemaElement::ComplexType(typ) => {
                logical.types.insert(typ.name.clone(), typ);
            }
            SchemaElement::Element(el) => {
                if let Some(name) = el.name.clone() {
                    logical.elements.insert(name, el);
                }
            }
            SchemaElement::Group(grp) => {
                if let Some(name) = grp.name.clone() {
                    logical.groups.insert(name, grp);
                }
            }
        });
        logical
    }
}
