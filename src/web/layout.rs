use crate::{
    Alignments, Colors, Container, ContainerStyle, Error, FlexBasis, FlexDirection, GridAutoRows,
    GridTemplate, MultiColumnLineStyle, Serde, Tree, Unit,
};

// widgets
impl Serde<Container, String> for Container {
    fn de(s: String) -> Result<Container, Error> {
        let t = Tree::de(s)?;
        assert!(t.children.len() == 1);

        let child = t.children[0].borrow().to_owned();
        let style = t.attrs.get("style").unwrap_or(&"".to_string()).to_string();

        Ok(Container {
            child,
            style: ContainerStyle::de(style)?,
        })
    }

    fn ser(&self) -> String {
        let t: Tree = self.into();
        t.ser()
    }
}

// styles
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

// enums
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

impl Serde<FlexBasis, String> for FlexBasis {
    fn de(s: String) -> Result<FlexBasis, Error> {
        let kv = s.split(":").collect::<Vec<&str>>();
        assert!(kv.len() == 2);

        Ok(match kv[1].trim() {
            "fill" => FlexBasis::Fill,
            "fit-content" => FlexBasis::FitContent,
            "max-content" => FlexBasis::MaxContent,
            "min-content" => FlexBasis::MinContent,
            x => FlexBasis::Number(Unit::de(x.to_string()).unwrap_or(Unit::Auto)),
        })
    }

    fn ser(&self) -> String {
        format!(
            "flex-basis: {};",
            match self {
                FlexBasis::Fill => "fill".into(),
                FlexBasis::FitContent => "fit-content".into(),
                FlexBasis::MaxContent => "max-content".into(),
                FlexBasis::MinContent => "min-content".into(),
                FlexBasis::Number(u) => u.ser(),
            }
        )
    }
}

impl Serde<FlexDirection, String> for FlexDirection {
    fn de(s: String) -> Result<FlexDirection, Error> {
        let kv = s.split(":").collect::<Vec<&str>>();
        assert!(kv.len() == 2);

        Ok(match kv[1].trim() {
            "column" => FlexDirection::Column,
            "column-reverse" => FlexDirection::ColumnReverse,
            "row" => FlexDirection::Row,
            "row-reverse" => FlexDirection::RowReverse,
            _ => FlexDirection::Column,
        })
    }

    fn ser(&self) -> String {
        format!(
            "flex-direction: {};",
            match self {
                FlexDirection::Column => "column",
                FlexDirection::ColumnReverse => "column-reverse",
                FlexDirection::Row => "row",
                FlexDirection::RowReverse => "row-reverse",
            }
        )
    }
}

impl Serde<GridAutoRows, String> for GridAutoRows {
    fn de(s: String) -> Result<GridAutoRows, Error> {
        let v = s.split(":").collect::<Vec<&str>>()[1].trim();
        Ok(match v {
            "auto" => GridAutoRows::Auto,
            "max-content" => GridAutoRows::MaxContent,
            "min-content" => GridAutoRows::MinContent,
            u => GridAutoRows::Fixed(Unit::de(u.to_string()).unwrap_or(Unit::Auto)),
        })
    }

    fn ser(&self) -> String {
        format!(
            "grid-auto-rows: {};",
            match self {
                GridAutoRows::Auto => "auto".to_string(),
                GridAutoRows::MaxContent => "max-content".to_string(),
                GridAutoRows::MinContent => "min-content".to_string(),
                GridAutoRows::Fixed(u) => u.ser(),
            }
        )
    }
}

impl Serde<GridTemplate, String> for GridTemplate {
    fn de(s: String) -> Result<GridTemplate, Error> {
        let kv = s.split(":").collect::<Vec<&str>>();
        assert!(kv.len() == 2);

        Ok(match kv[1].trim() {
            "auto" => GridTemplate::Auto,
            "max-content" => GridTemplate::MaxContent,
            "min-content" => GridTemplate::MinContent,
            _ => {
                if let Some(b) = kv[1].find("(") {
                    let nv = kv[1][b..(kv[1].len() - 1)]
                        .split(",")
                        .collect::<Vec<&str>>();
                    assert!(nv.len() == 2);

                    GridTemplate::Repeat(
                        nv[0].parse().unwrap_or(1),
                        Unit::de(nv[1].to_string()).unwrap_or(Unit::Auto),
                    )
                } else {
                    let mut o: Vec<Unit> = vec![];
                    kv[1]
                        .split(|x: char| x.is_whitespace())
                        .collect::<Vec<&str>>()
                        .iter()
                        .for_each(|x| o.push(Unit::de(x.to_string()).unwrap_or(Unit::Auto)));

                    GridTemplate::Plain(o)
                }
            }
        })
    }

    fn ser(&self) -> String {
        format!(
            "{}",
            match self {
                GridTemplate::Auto => "auto".to_string(),
                GridTemplate::MaxContent => "max-content".to_string(),
                GridTemplate::MinContent => "min-content".to_string(),
                GridTemplate::Plain(x) => {
                    let mut s = "".to_string();
                    for i in x {
                        s += &i.ser();
                    }

                    s
                }
                GridTemplate::Repeat(n, u) => {
                    format!("({}, {})", n, u.ser())
                }
            }
        )
    }
}

impl Serde<MultiColumnLineStyle, String> for MultiColumnLineStyle {
    fn de(s: String) -> Result<MultiColumnLineStyle, Error> {
        let kv = s.split(":").collect::<Vec<&str>>();
        assert!(kv.len() == 2);

        Ok(match kv[1] {
            "none" => MultiColumnLineStyle::None,
            "dotted" => MultiColumnLineStyle::Dotted,
            "double" => MultiColumnLineStyle::Double,
            "groove" => MultiColumnLineStyle::Groove,
            "hidden" => MultiColumnLineStyle::Hidden,
            "inset" => MultiColumnLineStyle::Inset,
            "outset" => MultiColumnLineStyle::OutSet,
            "ridge" => MultiColumnLineStyle::Ridge,
            "solid" => MultiColumnLineStyle::Solid,
            _ => MultiColumnLineStyle::None,
        })
    }

    fn ser(&self) -> String {
        format!(
            "style: {};",
            match self {
                MultiColumnLineStyle::Dashed => "dashed",
                MultiColumnLineStyle::Dotted => "dotted",
                MultiColumnLineStyle::Double => "double",
                MultiColumnLineStyle::Groove => "groove",
                MultiColumnLineStyle::Hidden => "hidden",
                MultiColumnLineStyle::Inset => "inset",
                MultiColumnLineStyle::None => "none",
                MultiColumnLineStyle::OutSet => "outset",
                MultiColumnLineStyle::Ridge => "ridge",
                MultiColumnLineStyle::Solid => "solid",
            }
        )
    }
}
