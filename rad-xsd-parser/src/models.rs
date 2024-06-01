use serde_derive::Deserialize;

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
    #[serde(rename = "$value")]
    pub complexContent: Extension,
}

#[derive(Deserialize, Debug)]
pub struct Extension {
    pub base: String,
    #[serde(rename = "$value")]
    pub extensions: Vec<ExtensionEl>,
}

#[derive(Deserialize, Debug)]
pub enum ExtensionEl {
    #[serde(rename = "attributeGroup")]
    AttributeGroup(AttributeGroup),
    #[serde(rename = "group")]
    Group(Group),
}

#[derive(Deserialize, Debug)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
}
