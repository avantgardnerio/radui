pub mod expected;
pub mod generate;
pub mod logical;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::generate::generate;
    use crate::logical::LogicalSchema;
    use crate::models::Schema;
    use quick_xml::de::from_str;
    use std::fs;
    use quick_xml::se::to_string;
    use serde_derive::{Deserialize, Serialize};

    #[test]
    fn it_works() {
        let filename = "../radui/resources/mxml.xsd";
        let content = fs::read_to_string(filename).expect("Unable to read file");

        let schema: Schema = from_str(&*content).unwrap();
        let logical: LogicalSchema = schema.into();
        let code = generate(logical);
        println!("{}", code);
    }

    #[test]
    fn enums() {
        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct WindowedApplication {
            #[serde(rename = "$value")]
            children: Vec<Enum>,
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct HBox {
            #[serde(rename = "@width")]
            pub width: String,
            #[serde(rename = "$value")]
            children: Vec<Enum>,
        }
        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct Label {}
        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct DataGrid {}

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        enum Enum {
            HBox(HBox),
            Label(Label),
            DataGrid(DataGrid),
        }

        let obj = WindowedApplication { children: vec![
            Enum::HBox(HBox {
                width: "100%".to_string(),
                children: vec![
                    Enum::Label(Label {}),
                ],
            }),
            Enum::DataGrid(DataGrid {})
        ] };
        let xml = to_string(&obj).unwrap();
        let expected = r#"
<WindowedApplication>
    <HBox width="100%">
        <Label/>
    </HBox>
    <DataGrid/>
</WindowedApplication>
"#.split("\n").map(|line| line.trim()).collect::<Vec<_>>().join("");
        assert_eq!(
            xml,
            expected
        );

        let object: WindowedApplication = from_str(&xml).unwrap();
        assert_eq!(object, obj);
    }
}
