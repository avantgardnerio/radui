use std::num::ParseIntError;
use std::str::FromStr;

pub enum Size {
    Absolute(u32),
    Relative(u32),
}

impl FromStr for Size {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: real (regex) parsing
        if s.contains("%") {
            let num = s.replace("%", "").parse::<u32>()?;
            Ok(Size::Relative(num))
        } else {
            let num = s.parse::<u32>()?;
            Ok(Size::Absolute(num))
        }
    }
}