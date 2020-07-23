use crate::{
    widgets::values::{Colors, Unit},
    Error, Serde,
};

impl Serde<Colors, String, Error> for Colors {
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
    fn ser(&self) -> String {
        self.to_string()
    }
}
