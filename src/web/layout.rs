use crate::{
    Align, AlignStyle, Alignments, Colors, Container, ContainerStyle, Error, FlexBasis,
    FlexDirection, FlexStyle, Grid, GridAutoRows, GridStyle, GridTemplate, MultiColumn,
    MultiColumnLineStyle, MultiColumnStyle, Serde, SizedBox, SizedBoxStyle, Tree, Unit,
};

/// serde single child widgets with match style
macro_rules! ss {
    {$(($widget:ident, $style:ident),)*} => {
        $(
            impl Serde<$widget, String> for $widget {
                fn de(s: String) -> Result<$widget, Error> {
                    let t = Tree::de(s)?;
                    assert!(t.children.len() == 1);

                    let child = t.children[0].borrow().to_owned();
                    let style = t.attrs.get("style").unwrap_or(&"".to_string()).to_string();

                    Ok($widget {
                        child,
                        style: $style::de(style)?,
                    })
                }

                fn ser(&self) -> String {
                    let t: Tree = self.into();
                    t.ser()
                }
            }

        )*
    };
}

/// serde single child widgets with match style
macro_rules! sm {
    {$(($widget:ident, $style:ident),)*} => {
        $(
            impl Serde<$widget, String> for $widget {
                fn de(s: String) -> Result<$widget, Error> {
                    let t = Tree::de(s)?;

                    let children = t.children.iter().map(
                        |w| w.borrow().to_owned()
                    ).collect::<Vec<Tree>>();
                    let style = t.attrs.get("style").unwrap_or(&"".to_string()).to_string();

                    Ok($widget {
                        children,
                        style: $style::de(style)?,
                    })
                }

                fn ser(&self) -> String {
                    let t: Tree = self.into();
                    t.ser()
                }
            }

        )*
    };
}

ss! {
    (Align, AlignStyle),
    (Container, ContainerStyle),
    (SizedBox, SizedBoxStyle),
}

sm! {
    (Grid, GridStyle),
    (MultiColumn, MultiColumnStyle),
}

fn parse<'p>(s: &'p str) -> Vec<(&'p str, &'p str)> {
    let mut attrs: Vec<(&str, &str)> = vec![];
    s.split(";").collect::<Vec<&str>>().iter().for_each(|x| {
        if let Some(m) = x.find(":") {
            attrs.push((x[..m].trim(), &x[m..].trim()));
        }
    });

    attrs
}

// styles
impl Serde<AlignStyle, String> for AlignStyle {
    fn de(s: String) -> Result<AlignStyle, Error> {
        Ok(AlignStyle {
            align: Alignments::de(s).unwrap_or(Alignments::Center),
        })
    }

    fn ser(&self) -> String {
        self.align.ser()
    }
}

impl Serde<ContainerStyle, String> for ContainerStyle {
    fn de(s: String) -> Result<ContainerStyle, Error> {
        let mut cs = ContainerStyle::default();
        let mut align = "".to_string();
        parse(&s).iter().for_each(|(k, v)| match *k {
            "height" => cs.height = Unit::de(v.to_string()).unwrap_or(Unit::Rem(1.0)),
            "width" => cs.width = Unit::de(v.to_string()).unwrap_or(Unit::Rem(1.0)),
            "padding" => cs.padding = Unit::de(v.to_string()).unwrap_or(Unit::Rem(0.0)),
            "margin" => cs.margin = Unit::de(v.to_string()).unwrap_or(Unit::Rem(0.0)),
            "background-color" => {
                cs.background_color = Colors::de(v.to_string()).unwrap_or(Colors::Inherit)
            }
            "alignment-items" => align.push_str(&format!("alignment-items: {};", v.to_string())),
            "justify-content" => align.push_str(&format!("justify-content: {};", v.to_string())),
            _ => {}
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

impl Serde<FlexStyle, String> for FlexStyle {
    fn de(s: String) -> Result<FlexStyle, Error> {
        let mut fs = FlexStyle {
            basis: FlexBasis::Fill,
            direction: FlexDirection::Column,
            grow: Unit::None(1.0),
            order: Unit::None(1.0),
            wrap: true,
        };

        parse(&s).iter().for_each(|(k, v)| match *k {
            "flex-basis" => fs.basis = FlexBasis::de(v.to_string()).unwrap_or(FlexBasis::Fill),
            "flex-direction" => {
                fs.direction = FlexDirection::de(v.to_string()).unwrap_or(FlexDirection::Column)
            }
            "flex-grow" => fs.grow = Unit::de(v.to_string()).unwrap_or(Unit::None(1.0)),
            "flex-order" => fs.order = Unit::de(v.to_string()).unwrap_or(Unit::None(1.0)),
            "flex-wrap" => {
                fs.wrap = match *v {
                    "wrap" => true,
                    _ => false,
                }
            }
            _ => {}
        });

        Ok(fs)
    }

    fn ser(&self) -> String {
        let mut s = "".to_string();
        s += &format!("flex-basis: {};", self.basis.ser());
        s += &format!("flex-direction: {};", self.direction.ser());
        s += &format!("flex-grow: {};", self.grow.ser());
        s += &format!("flex-order: {};", self.order.ser());
        s += &format!(
            "wrap: {};",
            match self.wrap {
                true => "wrap",
                false => "no-wrap",
            }
        );
        s
    }
}

impl Serde<GridStyle, String> for GridStyle {
    fn de(s: String) -> Result<GridStyle, Error> {
        let mut gs = GridStyle {
            col: Unit::Auto,
            row: Unit::Auto,
            gap: Unit::Auto,
            template_col: GridTemplate::Auto,
            template_row: GridTemplate::Auto,
            auto_rows: GridAutoRows::Auto,
        };

        parse(&s).iter().for_each(|(k, v)| match *k {
            "col" => gs.col = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "row" => gs.row = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "gap" => gs.gap = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "template-columns" => {
                gs.template_col = GridTemplate::de(v.to_string()).unwrap_or(GridTemplate::Auto)
            }
            "template-rows" => {
                gs.template_row = GridTemplate::de(v.to_string()).unwrap_or(GridTemplate::Auto)
            }
            "auto-rows" => {
                gs.auto_rows = GridAutoRows::de(v.to_string()).unwrap_or(GridAutoRows::Auto)
            }
            _ => {}
        });

        Ok(gs)
    }

    fn ser(&self) -> String {
        let mut ss = "".to_string();

        ss.push_str(&format!("col: {}", self.col.ser()));
        ss.push_str(&format!("row: {}", self.row.ser()));
        ss.push_str(&format!("gap: {}", self.gap.ser()));
        ss.push_str(&format!("template-columns: {}", self.template_col.ser()));
        ss.push_str(&format!("template-rows: {}", self.template_row.ser()));
        ss.push_str(&format!("auto-rows: {}", self.template_row.ser()));

        ss
    }
}

impl Serde<MultiColumnStyle, String> for MultiColumnStyle {
    fn de(s: String) -> Result<MultiColumnStyle, Error> {
        let mut mc = MultiColumnStyle {
            color: Colors::Inherit,
            count: Unit::Auto,
            gap: Unit::Auto,
            style: MultiColumnLineStyle::None,
        };

        parse(&s).iter().for_each(|(k, v)| match *k {
            "color" => mc.color = Colors::de(v.to_string()).unwrap_or(Colors::Inherit),
            "count" => mc.count = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "gap" => mc.gap = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "style" => {
                mc.style =
                    MultiColumnLineStyle::de(v.to_string()).unwrap_or(MultiColumnLineStyle::None)
            }
            _ => {}
        });

        Ok(mc)
    }

    fn ser(&self) -> String {
        let mut ss = "".to_string();

        ss.push_str(&format!("color: {}", self.color.ser()));
        ss.push_str(&format!("count: {}", self.count.ser()));
        ss.push_str(&format!("gap: {}", self.gap.ser()));
        ss.push_str(&format!("style: {}", self.style.ser()));

        ss
    }
}

impl Serde<SizedBoxStyle, String> for SizedBoxStyle {
    fn de(s: String) -> Result<SizedBoxStyle, Error> {
        let mut sbs = SizedBoxStyle {
            height: Unit::Auto,
            width: Unit::Auto,
        };

        parse(&s).iter().for_each(|(k, v)| match *k {
            "height" => sbs.height = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "width" => sbs.width = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            _ => {}
        });

        Ok(sbs)
    }

    fn ser(&self) -> String {
        format!(
            "height: {}; width: {};",
            self.height.ser(),
            self.width.ser()
        )
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
