use crate::{Colors, Error, Serde, TextStyle, Unit};

impl Serde<TextStyle, String> for TextStyle {
    fn de(s: String) -> Result<TextStyle, Error> {
        let mut ts = TextStyle::default();
        s.split(";").collect::<Vec<&str>>().iter().for_each(|x| {
            let v = x[x.find(":").unwrap_or(0)..].trim();
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

    fn ser(self) -> String {
        format!(
            "color: {}; font-weight: {}; font-style: {}; fontSize: {}; height: {}, font-stretch: {};",
            self.color.ser(), match self.bold {
                true => "700".into(),
                false => self.weight.ser(),
            },
            match self.italic {
                true => "italic",
                false => "normal"
            },
            self.size.ser(),
            self.height.ser(),
            self.stretch.ser(),
        )
    }
}
