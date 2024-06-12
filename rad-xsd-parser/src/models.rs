use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "schema")]
pub struct Schema {
    #[serde(rename = "$value")]
    pub schema_elements: Vec<SchemaElement>,
    #[serde(rename = "@targetNamespace")]
    pub target_namespace: String,
    #[serde(rename = "@xmlns:mx")]
    pub mx: String,
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@elementFormDefault")]
    pub element_form_default: String,
    #[serde(rename = "@attributeFormDefault")]
    pub attribute_form_default: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SchemaElement {
    AttributeGroup(AttributeGroup),
    ComplexType(ComplexType),
    Element(Element),
    Group(Group),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice: Option<Choice>,
    #[serde(rename = "@ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ChoiceOption>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ChoiceOption {
    Any(Any),
    Element(Element),
    Group(Group),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Any {
    #[serde(rename = "@namespace")]
    pub namespace: String,
    #[serde(rename = "@processContents")]
    pub process_contents: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,

    #[serde(rename = "@isAbstract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_group: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AttribGroupEl {
    AttributeGroup(AttributeGroup),
    Attribute(Attribute),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AttributeGroup {
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttribGroupEl>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComplexType {
    #[serde(rename = "@mixed")]
    pub mixed: bool,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub complex_content: Option<ComplexContent>,

    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<ComplexTypeEl>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ComplexTypeEl {
    Attribute(Attribute),
    Sequence(Sequence),
    ComplexContent(ComplexContent),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    #[serde(rename = "$value")]
    pub elements: Vec<SequenceEl>,
    #[serde(rename = "@minOccurs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_occurs: Option<String>,
    #[serde(rename = "@maxOccurs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_occurs: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SequenceEl {
    Element(Element),
    Sequence,
    All,
    Annotation,
    Any,
    Choice,
    Group(Group),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComplexContent {
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ComplexContentEl>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ComplexContentEl {
    Annotation,
    Extension(Extension),
    Restriction,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    #[serde(rename = "@base")]
    pub base: String,
    #[serde(rename = "$value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<ExtensionEl>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionEl {
    Attribute(Attribute),
    AttributeGroup(AttributeGroup),
    Group(Group),
    Sequence(Sequence),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub typ: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotation>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Annotation {
    pub documentation: String,
}
