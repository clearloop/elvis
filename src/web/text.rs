use crate::{Error, Serde, TextStyle};

impl Serde<TextStyle, String> for TextStyle {
    fn de(s: String) -> Result<TextStyle, Error> {
        unimplemented!()
    }

    fn ser(self) -> String {
        unimplemented!()
    }
}
