use crate::{Colors, Error, Serde};

/// unify in `rgba(r, g, b, a)`
impl Serde<Colors, String> for Colors {
    fn de(s: String) -> Result<Colors, Error> {
        let vs: Vec<&str> = s[5..s.len() - 1].split(",").collect();
        Ok(Colors::ORGB(
            vs[3].trim().parse().unwrap_or(0.0),
            vs[0].trim().parse().unwrap_or(0),
            vs[1].trim().parse().unwrap_or(0),
            vs[2].trim().parse().unwrap_or(0),
        ))
    }

    fn ser(self) -> String {
        match self {
            Colors::ORGB(o, r, g, b) => format!("rgba({}, {}, {}, {:.1})", r, g, b, o),
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