use crate::{
    widgets::values::{Colors, Unit},
    Error, Serde,
};
use std::str::FromStr;

impl Serde<Colors, String, Error> for Colors {
    fn de(s: String) -> Result<Colors, Error> {
        if s.contains("inherit") {
            return Ok(Colors::Inherit);
        }

        let vs: Vec<&str> = s[5..s.len() - 1].split(',').collect();
        Ok(Colors::ORGB(
            vs[3].trim().parse().unwrap_or(0.0),
            vs[0].trim().parse().unwrap_or(0),
            vs[1].trim().parse().unwrap_or(0),
            vs[2].trim().parse().unwrap_or(0),
        ))
    }

    fn ser(&self) -> String {
        match self {
            Colors::ORGB(o, r, g, b) => format!("rgba({}, {}, {}, {:.1})", r, g, b, o),
            Colors::Inherit => "inherit".into(),
            _ => {
                if let Colors::ORGB(o, r, g, b) = self.to_orgb() {
                    format!("rgba({}, {}, {}, {:.1})", r, g, b, o)
                } else {
                    "rgba(255, 255, 255, 255)".into()
                }
            }
        }
    }
}

impl Serde<Unit, String, Error> for Unit {
    fn de(s: String) -> Result<Unit, Error> {
        Unit::from_str(&s)
    }

    fn ser(&self) -> String {
        self.to_string()
    }
}
