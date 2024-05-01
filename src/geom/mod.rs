use std::num::ParseFloatError;
use std::str::FromStr;

pub enum Size {
    Absolute(f64),
    Relative(f64),
}

impl FromStr for Size {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: real (regex) parsing
        if s.contains("%") {
            let num = s.replace("%", "").parse::<f64>()?;
            Ok(Size::Relative(num))
        } else {
            let num = s.parse::<f64>()?;
            Ok(Size::Absolute(num))
        }
    }
}