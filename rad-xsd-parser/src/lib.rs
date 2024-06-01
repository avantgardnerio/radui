use serde_derive::Deserialize;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "$value")]
    pub schema_elements: Vec<SchemaElement>,
}

#[derive(Deserialize, Debug)]
pub enum SchemaElement {
    #[serde(rename = "attributeGroup")]
    AttributeGroup(AttributeGroup),
    #[serde(rename = "complexType")]
    ComplexType(ComplexType),
    #[serde(rename = "element")]
    Element(Element),
    #[serde(rename = "group")]
    Group(Group),
}

#[derive(Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub choice: Choice,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    #[serde(rename = "$value")]
    pub options: Vec<ChoiceOption>,
}

#[derive(Deserialize, Debug)]
pub enum ChoiceOption {
    #[serde(rename = "any")]
    Any(Any),
    #[serde(rename = "element")]
    Element(Element),
}

#[derive(Deserialize, Debug)]
pub struct Any {
    pub namespace: String,
    pub processContents: String,
}

#[derive(Deserialize, Debug)]
pub struct Element {
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
    #[serde(rename = "abstract")]
    pub is_abstract: Option<bool>,
    pub substitutionGroup: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum AttribGroupEl {
    #[serde(rename = "attributeGroup")]
    AttributeGroup(AttributeGroup),
    #[serde(rename = "attribute")]
    Attribute(Attribute)
}

#[derive(Deserialize, Debug)]
pub struct AttributeGroup {
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    #[serde(rename = "$value")]
    pub attributes: Option<Vec<AttribGroupEl>>,
}

#[derive(Deserialize, Debug)]
pub struct ComplexType {
    pub mixed: bool,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn it_works() {
        let filename = "../radui/resources/mxml.xsd";
        let content= fs::read_to_string(filename).expect("Unable to read file");

        let schema: Schema =  serde_xml_rs::from_str(&*content).unwrap();
    }
}
