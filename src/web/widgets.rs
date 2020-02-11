use crate::{Colors, Error, Image, ImageSrc, Serde, Text, TextStyle, Tree, Unit};

/// Serde for Image
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

impl Serde<ImageSrc, String> for ImageSrc {
    fn de(s: String) -> Result<ImageSrc, Error> {
        Ok(ImageSrc(s))
    }

    fn ser(&self) -> String {
        format!("background-image: url({})", self.0)
    }
}

/// Serde for Text
impl Serde<Text, String> for Text {
    fn de(s: String) -> Result<Text, Error> {
        let t = Tree::de(s)?;
        if t.children.len() != 1 {
            return Err(Error::DeserializeHtmlError(
                "deserialize Text failed, children's length should be 1".into(),
            ));
        }

        let text = t.children[0].borrow();
        Ok(Text::new(
            text.attrs
                .get("text")
                .unwrap_or(&"".to_string())
                .to_string(),
            TextStyle::de(t.attrs.get("style").unwrap_or(&"".into()).to_string())?.into(),
        ))
    }

    fn ser(&self) -> String {
        let t: Tree = self.into();
        t.ser()
    }
}

/// Serde for TextStyle
impl Serde<TextStyle, String> for TextStyle {
    fn de(s: String) -> Result<TextStyle, Error> {
        let mut ts = TextStyle::default();
        s.split(";").collect::<Vec<&str>>().iter().for_each(|x| {
            if x.len() < 1 {
                return;
            }

            let v = x[(x.find(":").unwrap_or(0) + 1)..].trim();
            match x {
                k if k.contains("color") => {
                    ts.color = Colors::de(v.into()).unwrap_or(Colors::Black)
                }
                k if k.contains("font-weight") => {
                    ts.weight = Unit::de(v.into()).unwrap_or(Unit::None(400.0));
                    ts.bold = match ts.weight {
                        Unit::None(x) => x == 700.0,
                        _ => false,
                    }
                }
                k if k.contains("font-style") => {
                    ts.italic = match v {
                        "italic" => true,
                        _ => false,
                    };
                }
                k if k.contains("font-size") => {
                    ts.size = Unit::de(v.into()).unwrap_or(Unit::Rem(1.0))
                }
                k if k.contains("height") => {
                    ts.height = Unit::de(v.into()).unwrap_or(Unit::Rem(1.0))
                }
                k if k.contains("font-stretch") => {
                    ts.stretch = Unit::de(v.into()).unwrap_or(Unit::Percent(100.0))
                }
                _ => {}
            }
        });

        Ok(ts)
    }

    fn ser(&self) -> String {
        format!(
            "color: {}; font-weight: {}; font-style: {}; font-size: {}; font-stretch: {}; line-height: {};",
            self.color.ser(), match self.bold {
                true => "700".into(),
                false => self.weight.ser(),
            },
            match self.italic {
                true => "italic",
                false => "normal"
            },
            self.size.ser(),
            self.stretch.ser(),
            self.height.ser(),
        )
    }
}
