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

pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

pub enum GridAutoRows {
    Auto(Unit, Option<Unit>),
    Fixed(Unit),
}

pub enum GridTemplate {
    Plain(Vec<Unit>),
    Repeat(i32, Unit),
}

pub enum MultiColumnStyle {
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
