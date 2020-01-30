use crate::{Error, Serde, Unit};

impl Serde<Unit, String> for Unit {
    fn de(s: String) -> Result<Unit, Error> {
        Ok(Unit::from_str(s))
    }

    fn ser(self) -> String {
        self.to_string()
    }
}
