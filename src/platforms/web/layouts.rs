use crate::{
    widgets::layouts::*,
    widgets::values::{layouts::*, *},
    Error, Node, Serde,
};

/// serde single child widgets with match style
macro_rules! ss {
    {$(($widget:ident, $style:ident),)*} => {
        $(
            impl Serde<$widget, String, Error> for $widget {
                fn de(s: String) -> Result<$widget, Error> {
                    let t = Node::de(s)?;
                    assert!(t.children.len() == 1);

                    let child = t.children[0].borrow().to_owned();
                    let style = t.attrs.get("style").unwrap_or(&"".to_string()).to_string();

                    Ok($widget {
                        child,
                        style: $style::de(style)?,
                    })
                }

                fn ser(&self) -> String {
                    let t: Node = self.into();
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
            impl Serde<$widget, String, Error> for $widget {
                fn de(s: String) -> Result<$widget, Error> {
                    let t = Node::de(s)?;

                    let children = t.children.iter().map(
                        |w| w.borrow().to_owned()
                    ).collect::<Vec<Node>>();
                    let style = t.attrs.get("style").unwrap_or(&"".to_string()).to_string();

                    Ok($widget {
                        children,
                        style: $style::de(style)?,
                    })
                }

                fn ser(&self) -> String {
                    let t: Node = self.into();
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
    s.split(';').collect::<Vec<&str>>().iter().for_each(|x| {
        if let Some(m) = x.find(':') {
            attrs.push((x[..m].trim(), &x[m..].trim()));
        }
    });

    attrs
}

// styles
impl Serde<AlignStyle, String, Error> for AlignStyle {
    fn de(s: String) -> Result<AlignStyle, Error> {
        Ok(AlignStyle {
            align: Alignments::de(s).unwrap_or(Alignments::Center),
        })
    }

    fn ser(&self) -> String {
        self.align.ser()
    }
}

impl Serde<ContainerStyle, String, Error> for ContainerStyle {
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
            "align-items" => align.push_str(&format!("align-items: {};", v.to_string())),
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

impl Serde<FlexStyle, String, Error> for FlexStyle {
    fn de(s: String) -> Result<FlexStyle, Error> {
        let mut fs = FlexStyle {
            align: Alignments::Center,
            basis: FlexBasis::Fill,
            direction: FlexDirection::Column,
            grow: Unit::None(1.0),
            order: Unit::None(1.0),
            wrap: true,
        };

        let mut align = "".to_string();
        parse(&s).iter().for_each(|(k, v)| match *k {
            "flex-basis" => fs.basis = FlexBasis::de(v.to_string()).unwrap_or(FlexBasis::Fill),
            "flex-direction" => {
                fs.direction = FlexDirection::de(v.to_string()).unwrap_or(FlexDirection::Column)
            }
            "flex-grow" => fs.grow = Unit::de(v.to_string()).unwrap_or(Unit::Percent(0.0)),
            "flex-order" => fs.order = Unit::de(v.to_string()).unwrap_or(Unit::None(1.0)),
            "flex-wrap" => {
                fs.wrap = match *v {
                    "wrap" => true,
                    _ => false,
                }
            }
            "align-items" => align.push_str(&format!("align-items: {};", v.to_string())),
            "justify-content" => align.push_str(&format!("justify-content: {};", v.to_string())),
            _ => {}
        });

        fs.align = Alignments::de(align)?;
        Ok(fs)
    }

    fn ser(&self) -> String {
        let mut s = "".to_string();
        s += &self.align.ser();
        s += &self.basis.ser();
        s += &self.direction.ser();
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

impl Serde<GridStyle, String, Error> for GridStyle {
    fn de(s: String) -> Result<GridStyle, Error> {
        let mut gs = GridStyle {
            col: GridAuto::Auto,
            col_gap: Unit::None(0.0),
            flow: GridFlow::Row,
            row: GridAuto::Auto,
            row_gap: Unit::None(0.0),
            template_col: GridTemplate::None,
            template_row: GridTemplate::None,
        };

        parse(&s).iter().for_each(|(k, v)| match *k {
            "grid-auto-columns" => gs.col = GridAuto::de(v.to_string()).unwrap_or(GridAuto::Auto),
            "grid-auto-flow" => gs.flow = GridFlow::de(v.to_string()).unwrap_or(GridFlow::Row),
            "grid-auto-rows" => gs.row = GridAuto::de(v.to_string()).unwrap_or(GridAuto::Auto),
            "grid-column-gap" => gs.col_gap = Unit::de(v.to_string()).unwrap_or(Unit::None(0.0)),
            "grid-row-gap" => gs.row_gap = Unit::de(v.to_string()).unwrap_or(Unit::None(0.0)),
            "grid-template-columns" => {
                gs.template_col = GridTemplate::de(v.to_string()).unwrap_or(GridTemplate::None)
            }
            "grid-template-rows" => {
                gs.template_row = GridTemplate::de(v.to_string()).unwrap_or(GridTemplate::None)
            }
            _ => {}
        });

        Ok(gs)
    }

    fn ser(&self) -> String {
        let mut ss = "".to_string();

        ss.push_str("display: grid;");
        ss.push_str(&format!("grid-auto-columns: {};", self.col.ser()));
        ss.push_str(&format!("grid-auto-flow: {};", self.flow.ser()));
        ss.push_str(&format!("grid-auto-rows: {};", self.row.ser()));
        ss.push_str(&format!("grid-column-gap: {};", self.col_gap.ser()));
        ss.push_str(&format!("grid-row-gap: {};", self.row_gap.ser()));
        ss.push_str(&format!(
            "grid-template-columns: {};",
            self.template_col.ser()
        ));
        ss.push_str(&format!("grid-template-rows: {};", self.template_row.ser()));
        ss
    }
}

impl Serde<MultiColumnStyle, String, Error> for MultiColumnStyle {
    fn de(s: String) -> Result<MultiColumnStyle, Error> {
        let mut mc = MultiColumnStyle {
            color: Colors::Inherit,
            count: Unit::Auto,
            gap: Unit::Auto,
            style: MultiColumnLineStyle::None,
        };

        parse(&s).iter().for_each(|(k, v)| match *k {
            "column-count" => mc.count = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "column-gap" => mc.gap = Unit::de(v.to_string()).unwrap_or(Unit::Auto),
            "column-rule-color" => mc.color = Colors::de(v.to_string()).unwrap_or(Colors::Inherit),
            "column-rule-style" => {
                mc.style =
                    MultiColumnLineStyle::de(v.to_string()).unwrap_or(MultiColumnLineStyle::None)
            }
            _ => {}
        });

        Ok(mc)
    }

    fn ser(&self) -> String {
        let mut ss = "".to_string();
        ss.push_str(&format!("column-count: {}", self.count.ser()));
        ss.push_str(&format!("column-gap: {}", self.gap.ser()));
        ss.push_str(&format!("column-rule-color: {}", self.color.ser()));
        ss.push_str(&format!("column-rule-style: {}", self.style.ser()));
        ss
    }
}

impl Serde<SizedBoxStyle, String, Error> for SizedBoxStyle {
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
impl Serde<Alignments, String, Error> for Alignments {
    fn de(s: String) -> Result<Alignments, Error> {
        let ss = s.split(';').collect::<Vec<&str>>();
        assert!(ss.len() == 2);

        let ai = ss[0][ss[0].find(':').unwrap_or(0)..].trim();
        let jc = ss[1][ss[1].find(':').unwrap_or(0)..].trim();

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

        format!("align-items: {}; justify-content: {};", ai, jc)
    }
}

impl Serde<FlexBasis, String, Error> for FlexBasis {
    fn de(s: String) -> Result<FlexBasis, Error> {
        let kv = s.split(':').collect::<Vec<&str>>();
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

impl Serde<FlexDirection, String, Error> for FlexDirection {
    fn de(s: String) -> Result<FlexDirection, Error> {
        let kv = s.split(':').collect::<Vec<&str>>();
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

impl Serde<GridAuto, String, Error> for GridAuto {
    fn de(s: String) -> Result<GridAuto, Error> {
        let v = s.split(':').collect::<Vec<&str>>()[1].trim();
        Ok(match v {
            "auto" => GridAuto::Auto,
            "max-content" => GridAuto::MaxContent,
            "min-content" => GridAuto::MinContent,
            "inherit" => GridAuto::Inherit,
            "initial" => GridAuto::Initial,
            "unset" => GridAuto::Unset,
            m if m.contains("minmax") => {
                if let Some(i) = m.find('(') {
                    let cr = m[i..(m.len() - 1)].split(',').collect::<Vec<&str>>();
                    assert!(cr.len() == 2);

                    GridAuto::MinMax(
                        Unit::de(cr[0].trim().to_string())?,
                        Unit::de(cr[1].trim().to_string())?,
                    )
                } else {
                    GridAuto::Unset
                }
            }
            x if x.trim().contains(|w: char| w.is_whitespace()) => {
                let mut o: Vec<Unit> = vec![];
                x.split(|c: char| c.is_whitespace())
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|u| o.push(Unit::de(u.to_string()).unwrap_or(Unit::Auto)));

                GridAuto::Plain(o)
            }
            u => GridAuto::Fixed(Unit::de(u.to_string()).unwrap_or(Unit::Auto)),
        })
    }

    fn ser(&self) -> String {
        match self {
            GridAuto::Auto => "auto".to_string(),
            GridAuto::MaxContent => "max-content".to_string(),
            GridAuto::MinContent => "min-content".to_string(),
            GridAuto::MinMax(c, r) => format!("minmax({}, {})", c.ser(), r.ser()),
            GridAuto::Fixed(u) => u.ser(),
            GridAuto::Inherit => "inherit".to_string(),
            GridAuto::Initial => "initial".to_string(),
            GridAuto::Unset => "unset".to_string(),
            GridAuto::Plain(x) => {
                let mut s = "".to_string();
                for i in x {
                    s += &i.ser();
                    s += " ";
                }

                s
            }
        }
    }
}

impl Serde<GridFlow, String, Error> for GridFlow {
    fn de(s: String) -> Result<GridFlow, Error> {
        let v = s.split(':').collect::<Vec<&str>>()[1].trim();
        Ok(match v {
            "column" => GridFlow::Column,
            "column dense" => GridFlow::ColumnDense,
            "dense" => GridFlow::Dense,
            "inherit" => GridFlow::Inherit,
            "initial" => GridFlow::Initial,
            "row" => GridFlow::Row,
            "row dense" => GridFlow::RowDense,
            _ => GridFlow::Unset,
        })
    }

    fn ser(&self) -> String {
        match self {
            GridFlow::Column => "column",
            GridFlow::ColumnDense => "column dense",
            GridFlow::Dense => "dense",
            GridFlow::Inherit => "inherit",
            GridFlow::Initial => "initial",
            GridFlow::Row => "row",
            GridFlow::RowDense => "row dense",
            GridFlow::Unset => "unset",
        }
        .to_string()
    }
}

impl Serde<GridTemplate, String, Error> for GridTemplate {
    fn de(s: String) -> Result<GridTemplate, Error> {
        let kv = s.split(':').collect::<Vec<&str>>();
        assert!(kv.len() == 2);

        Ok(match kv[1].trim() {
            "inherit" => GridTemplate::Inherit,
            "initial" => GridTemplate::Initial,
            "subgrid" => GridTemplate::SubGrid,
            "unset" => GridTemplate::Unset,
            "none" => GridTemplate::None,
            x if x.contains("fit-content") => {
                if let Some(i) = x.find('(') {
                    GridTemplate::FitContent(Unit::de(x[i..(x.len() - 1)].trim().to_string())?)
                } else {
                    GridTemplate::Unset
                }
            }
            m if m.contains("minmax") => {
                if let Some(i) = m.find('(') {
                    let ai = m[i..(m.len() - 1)].split(',').collect::<Vec<&str>>();
                    assert!(ai.len() == 2);

                    GridTemplate::MinMax(
                        Unit::de(ai[0].trim().to_string())?,
                        Unit::de(ai[1].trim().to_string())?,
                    )
                } else {
                    GridTemplate::Unset
                }
            }
            _ => {
                if let Some(b) = kv[1].find('(') {
                    let nv = kv[1][b..(kv[1].len() - 1)]
                        .split(',')
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
        match self {
            GridTemplate::FitContent(u) => format!("fit-content({})", u.ser()),
            GridTemplate::Inherit => "inherit".to_string(),
            GridTemplate::Initial => "initial".to_string(),
            GridTemplate::MinMax(i, a) => format!("minmax({}, {})", i.ser(), a.ser()),
            GridTemplate::None => "none".to_string(),
            GridTemplate::Plain(x) => {
                let mut s = "".to_string();
                for i in x {
                    s += &i.ser();
                    s += " ";
                }

                s
            }
            GridTemplate::Repeat(n, u) => format!("({}, {})", n, u.ser()),
            GridTemplate::SubGrid => "subgrid".to_string(),
            GridTemplate::Unset => "unit".to_string(),
        }
    }
}

impl Serde<MultiColumnLineStyle, String, Error> for MultiColumnLineStyle {
    fn de(s: String) -> Result<MultiColumnLineStyle, Error> {
        let kv = s.split(':').collect::<Vec<&str>>();
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
