pub mod expected;
pub mod generate;
pub mod logical;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::generate::generate;
    use crate::logical::LogicalSchema;
    use crate::models::Schema;
    use std::fs;
    use quick_xml::de::from_str;

    #[test]
    fn it_works() {
        let filename = "../radui/resources/mxml.xsd";
        let content = fs::read_to_string(filename).expect("Unable to read file");

        let schema: Schema = from_str(&*content).unwrap();
        let logical: LogicalSchema = schema.into();
        let code = generate(logical);
        println!("{}", code);
    }
}
