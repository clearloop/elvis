use crate::{Error, Image, Serde, Tree};

impl Serde<Image, String> for Image {
    fn de(s: String) -> Result<Image, Error> {
        let t = Tree::de(s)?;
        if t.children.len() != 1 {
            return Err(Error::DeserializeHtmlError(
                "deserialize Text failed, children's length should be 1".into(),
            ));
        }

        let child: Tree = match t.children.len() > 0 {
            true => t.children[0].borrow().to_owned(),
            false => Tree::default(),
        };

        Ok(Image::new(
            t.attrs.get("src").unwrap_or(&"".into()).to_string(),
            child,
        ))
    }

    fn ser(&self) -> String {
        let t: Tree = self.into();
        t.ser()
    }
}
