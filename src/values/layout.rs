use crate::Unit;

/// `Flex` Alignments
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

/// `flex-basis` property
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

/// `flex-direction` property
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

/// config columns and rows in `Grid`
pub enum GridAuto {
    Auto,
    Fixed(Unit),
    Inherit,
    Initial,
    MaxContent,
    MinContent,
    MinMax(Unit, Unit),
    Plain(Vec<Unit>),
    Unset,
}

impl Default for GridAuto {
    fn default() -> GridAuto {
        GridAuto::Unset
    }
}

/// manage `Grid` direction
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

/// template rule in `Grid` columns an rows
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

/// line-style in `MultiColumn`
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
