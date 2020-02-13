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

pub enum GridAuto {
    Auto,
    MaxContent,
    MinContent,
    MinMax(Unit, Unit),
    Fixed(Unit),
    Inherit,
    Initial,
    Unset,
}

impl Default for GridAuto {
    fn default() -> GridAuto {
        GridAuto::Unset
    }
}

pub enum GridFlow {
    Column,
    Row,
    Dense,
    ColumnDense,
    RowDense,
    Inherit,
    Initial,
    Unset,
}

impl Default for GridFlow {
    fn default() -> GridFlow {
        GridFlow::Unset
    }
}

pub enum GridTemplate {
    FitContent(Unit),
    Inherit,
    Initial,
    MinMax(Unit, Unit),
    None,
    Plain(Vec<Unit>),
    Repeat(i32, Unit),
    SubGrid,
    Unset,
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
