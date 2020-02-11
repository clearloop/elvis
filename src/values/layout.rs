use crate::Unit;

pub enum Alignments {
    BottomCenter,
    BottomLeft,
    BottomRight,
    Center,
    CenterLeft,
    CenterRight,
    TopCenter,
    TopLeft,
    TopRight,
}

impl Default for Alignments {
    fn default() -> Alignments {
        Alignments::Center
    }
}

pub enum FlexBasis {
    Fill,
    MaxContent,
    MinContent,
    FitContent,
    Number(Unit),
}

impl Default for FlexBasis {
    fn default() -> FlexBasis {
        FlexBasis::Fill
    }
}

pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Column
    }
}

pub enum GridAutoRows {
    Auto,
    MaxContent,
    MinContent,
    Fixed(Unit),
}

impl Default for GridAutoRows {
    fn default() -> GridAutoRows {
        GridAutoRows::Auto
    }
}

pub struct GridTemplateColumns(GridTemplate);
pub struct GridTemplateRow(GridTemplate);

pub enum GridTemplate {
    Auto,
    MaxContent,
    MinContent,
    Plain(Vec<Unit>),
    Repeat(i32, Unit),
}

impl Default for GridTemplate {
    fn default() -> GridTemplate {
        GridTemplate::Repeat(1, Unit::Fr(1.0))
    }
}

pub enum MultiColumnLineStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    OutSet,
}

impl Default for MultiColumnLineStyle {
    fn default() -> MultiColumnLineStyle {
        MultiColumnLineStyle::None
    }
}
