use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "$value")]
    pub schema_elements: Vec<SchemaElement>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SchemaElement {
    AttributeGroup(AttributeGroup),
    ComplexType(ComplexType),
    Element(Element),
    Group(Group),
}

#[derive(Deserialize, Debug)]
pub struct Group {
    pub name: Option<String>,
    pub choice: Option<Choice>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    pub minOccurs: Option<String>,
    pub maxOccurs: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    #[serde(rename = "$value")]
    pub options: Vec<ChoiceOption>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ChoiceOption {
    Any(Any),
    Element(Element),
}

#[derive(Deserialize, Debug)]
pub struct Any {
    pub namespace: String,
    pub processContents: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AttribGroupEl {
    AttributeGroup(AttributeGroup),
    Attribute(Attribute),
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
#[serde(rename_all = "camelCase")]
pub struct ComplexType {
    pub mixed: bool,
    pub name: String,
    pub complex_content: Option<ComplexContent>,
}

#[derive(Deserialize, Debug)]
pub struct ComplexContent {
    #[serde(rename = "$value")]
    pub extension: Option<Extension>,
}

#[derive(Deserialize, Debug)]
pub struct Extension {
    pub base: String,
    #[serde(rename = "$value")]
    pub extensions: Option<Vec<ExtensionEl>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionEl {
    AttributeGroup(AttributeGroup),
    Group(Group),
}

#[derive(Deserialize, Debug)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
}
