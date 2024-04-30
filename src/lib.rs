use yaserde_derive::{YaDeserialize, YaSerialize};
use xsd_types::types as xs;
use xsd_parser::generator::validator::Validate;
use xsd_macro_utils::UtilsTupleIo;
use xsd_macro_utils::UtilsDefaultSerde;
use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/models.rs"));

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
