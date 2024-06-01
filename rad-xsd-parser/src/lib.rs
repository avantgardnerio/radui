mod expected;
mod generate;
mod logical;
mod models;

#[cfg(test)]
mod tests {
    use crate::models::Schema;
    use std::fs;

    #[test]
    fn it_works() {
        let filename = "../radui/resources/mxml.xsd";
        let content = fs::read_to_string(filename).expect("Unable to read file");

        let schema: Schema = serde_xml_rs::from_str(&*content).unwrap();
    }
}
