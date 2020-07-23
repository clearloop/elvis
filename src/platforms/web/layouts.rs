use crate::{widgets::layouts::*, widgets::values::layouts::*, Error, Serde};

// styles
impl Serde<AlignStyle, String, Error> for AlignStyle {
    fn ser(&self) -> String {
        self.align.ser()
    }
}

impl Serde<ContainerStyle, String, Error> for ContainerStyle {
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
    fn ser(&self) -> String {
        let mut s = "".to_string();
        s += &self.align.ser();
        s += &self.basis.ser();
        s += &self.direction.ser();
        s += &format!("flex-grow: {};", self.grow.ser());
        s += &format!("flex-order: {};", self.order.ser());
        s += &format!("wrap: {};", if self.wrap { "wrap" } else { "no-wrap" });
        s
    }
}

impl Serde<GridStyle, String, Error> for GridStyle {
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
