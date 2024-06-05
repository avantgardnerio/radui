use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Schema {
    #[serde(rename = "$value")]
    pub schema_elements: Vec<SchemaElement>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SchemaElement {
    AttributeGroup(AttributeGroup),
    ComplexType(ComplexType),
    Element(Element),
    Group(Group),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: Option<String>,
    pub choice: Option<Choice>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub min_occurs: Option<String>,
    pub max_occurs: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Choice {
    #[serde(rename = "$value")]
    pub options: Option<Vec<ChoiceOption>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ChoiceOption {
    Any(Any),
    Element(Element),
    Group(Group),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Any {
    pub namespace: String,
    pub process_contents: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub is_abstract: Option<bool>,
    pub substitution_group: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AttribGroupEl {
    AttributeGroup(AttributeGroup),
    Attribute(Attribute),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AttributeGroup {
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    #[serde(rename = "$value")]
    pub attributes: Option<Vec<AttribGroupEl>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComplexType {
    pub mixed: bool,
    pub name: String,
    pub complex_content: Option<ComplexContent>,
    pub sequence: Option<Sequence>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sequence {
    #[serde(rename = "$value")]
    pub elements: Vec<Element>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ComplexContent {
    #[serde(rename = "$value")]
    pub extension: Option<Extension>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Extension {
    pub base: String,
    #[serde(rename = "$value")]
    pub extensions: Option<Vec<ExtensionEl>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionEl {
    AttributeGroup(AttributeGroup),
    Group(Group),
    Sequence(Sequence),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
}
