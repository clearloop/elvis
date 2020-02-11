use crate::{Alignments, Colors, Container, ContainerStyle, Error, Serde, Unit};

// impl Serde<Container, String> for Container {
//     fn de(s: String) -> Result<Container, Error> {
//
//     }
//
//     fn ser(&self) -> String {
//
//     }
// }

impl Serde<ContainerStyle, String> for ContainerStyle {
    fn de(s: String) -> Result<ContainerStyle, Error> {
        let mut cs = ContainerStyle::default();
        let mut align = "".to_string();
        s.split(";").collect::<Vec<&str>>().iter().for_each(|x| {
            let m = x.find(":").unwrap_or(0);
            let (k, v) = (x[..m].trim(), x[m..].trim().to_string());
            match k {
                "height" => cs.height = Unit::de(v).unwrap_or(Unit::Rem(1.0)),
                "width" => cs.width = Unit::de(v).unwrap_or(Unit::Rem(1.0)),
                "padding" => cs.padding = Unit::de(v).unwrap_or(Unit::Rem(0.0)),
                "margin" => cs.margin = Unit::de(v).unwrap_or(Unit::Rem(0.0)),
                "background-color" => {
                    cs.background_color = Colors::de(v).unwrap_or(Colors::Inherit)
                }
                "alignment-items" => align.push_str(&format!("alignment-items: {};", v)),
                "justify-content" => align.push_str(&format!("justify-content: {};", v)),
                _ => {}
            }
        });

        cs.align = Alignments::de(align)?;
        Ok(cs)
    }

    fn ser(&self) -> String {
        let mut s = "".to_string();
        s += &self.align.ser();
        s += &format!("height: {};", self.height.ser());
        s += &format!("width: {};", self.width.ser());
        s += &format!("padding: {};", self.padding.ser());
        s += &format!("margin: {};", self.margin.ser());
        s += &format!("background-color: {};", self.background_color.ser());
        s
    }
}

impl Serde<Alignments, String> for Alignments {
    fn de(s: String) -> Result<Alignments, Error> {
        let ss = s.split(";").collect::<Vec<&str>>();
        assert!(ss.len() == 2);

        let ai = ss[0][ss[0].find(":").unwrap_or(0)..].trim();
        let jc = ss[1][ss[1].find(":").unwrap_or(0)..].trim();

        Ok(match ai {
            "flex-end" => match jc {
                "center" => Alignments::BottomCenter,
                "flex-start" => Alignments::BottomLeft,
                "flex-end" => Alignments::BottomRight,
                _ => Alignments::Center,
            },
            "center" => match jc {
                "flex-start" => Alignments::CenterLeft,
                "flex-end" => Alignments::CenterRight,
                _ => Alignments::Center,
            },
            "flex-start" => match jc {
                "center" => Alignments::TopCenter,
                "flex-start" => Alignments::TopLeft,
                "flex-end" => Alignments::TopRight,
                _ => Alignments::Center,
            },
            _ => Alignments::Center,
        })
    }

    fn ser(&self) -> String {
        let (ai, jc) = match self {
            Alignments::BottomCenter => ("flex-end", "center"),
            Alignments::BottomLeft => ("flex-end", "flex-start"),
            Alignments::BottomRight => ("flex-end", "flex-end"),
            Alignments::Center => ("center", "center"),
            Alignments::CenterLeft => ("center", "flex-start"),
            Alignments::CenterRight => ("center", "flex-end"),
            Alignments::TopCenter => ("flex-start", "center"),
            Alignments::TopLeft => ("flex-start", "flex-start"),
            Alignments::TopRight => ("flex-start", "flex-end"),
        };

        format!("alignment-items: {}; justify-content: {};", ai, jc)
    }
}
