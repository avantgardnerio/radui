mod models;
mod generate;
mod expected;
mod logical;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::models::Schema;
    use super::*;

    #[test]
    fn it_works() {
        let filename = "../radui/resources/mxml.xsd";
        let content = fs::read_to_string(filename).expect("Unable to read file");

        let schema: Schema = serde_xml_rs::from_str(&*content).unwrap();
    }
}
