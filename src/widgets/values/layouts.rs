//! Elvis layout values
use crate::widgets::values::Unit;

/// `Flex` Alignments
pub enum Alignments {
    /// Align bottom-center
    BottomCenter,
    /// Align bottom-left
    BottomLeft,
    /// Align bottom-right
    BottomRight,
    /// Align center
    Center,
    /// Align center-left
    CenterLeft,
    /// Align center-right
    CenterRight,
    /// Align top-center
    TopCenter,
    /// Align top-left
    TopLeft,
    /// Align top-right
    TopRight,
}

impl Default for Alignments {
    fn default() -> Alignments {
        Alignments::Center
    }
}

/// `flex-basis` property
pub enum FlexBasis {
    /// Fill the flex box
    Fill,
    /// Sizing max content
    MaxContent,
    /// Sizing min content
    MinContent,
    /// Fit content
    FitContent,
    /// Specify width
    Number(Unit),
}

impl Default for FlexBasis {
    fn default() -> FlexBasis {
        FlexBasis::Fill
    }
}

/// `flex-direction` property
pub enum FlexDirection {
    /// The direction in which lines of text are stacked
    Column,
    /// Like `FlexDirection::Column`, but reversed
    ColumnReverse,
    /// The direction text is laid out in a line
    Row,
    /// Like `FlexDirection::Row`, but reversed
    RowReverse,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Column
    }
}

/// config columns and rows in `Grid`
pub enum GridAuto {
    /// Auto Grid
    Auto,
    /// Fix items in every line
    Fixed(Unit),
    /// Inherit the style of parents
    Inherit,
    /// Use initial value
    Initial,
    /// Max content
    MaxContent,
    /// Min content
    MinContent,
    /// Use both max and min content
    MinMax(Unit, Unit),
    /// Use grid template
    Plain(Vec<Unit>),
    /// Unset the value
    Unset,
}

impl Default for GridAuto {
    fn default() -> GridAuto {
        GridAuto::Unset
    }
}

/// Manage `Grid` direction
///
/// ### `Row`
///
/// Items are placed by filling each row in turn, adding new rows as necessary.
/// If neither row nor column is provided, row is assumed.
///
/// ### `Column`
///
/// Items are placed by filling each column in turn, adding new columns as necessary.
///
/// ### `Dense`
///
/// dense" packing algorithm attempts to fill in holes earlier in the grid, if smaller items come up later. This may cause items to appear out-of-order, when doing so would fill in holes left by larger items.
///
/// If it is omitted, a "sparse" algorithm is used, where the placement algorithm only ever moves "forward" in the grid when placing items, never backtracking to fill holes. This ensures that all of the auto-placed items appear "in order", even if this leaves holes that could have been filled by later items.
pub enum GridFlow {
    /// Grid Column
    Column,
    /// Grid Row
    Row,
    /// Grid deesen
    Dense,
    /// Grid column desen
    ColumnDense,
    /// Grid row desen
    RowDense,
    /// Inherit the style of parent
    Inherit,
    /// Use initial grid
    Initial,
    /// Unset the flow
    Unset,
}

impl Default for GridFlow {
    fn default() -> GridFlow {
        GridFlow::Unset
    }
}

/// template rule in `Grid` columns an rows
pub enum GridTemplate {
    /// Fit content
    FitContent(Unit),
    /// Inherit the style of parent
    Inherit,
    /// Use initial style
    Initial,
    /// Set min and max width
    MinMax(Unit, Unit),
    /// No template
    None,
    /// Customize grid
    Plain(Vec<Unit>),
    /// Repeat width
    Repeat(i32, Unit),
    /// Use sub grid
    SubGrid,
    /// Unset grid template
    Unset,
}

impl Default for GridTemplate {
    fn default() -> GridTemplate {
        GridTemplate::Repeat(1, Unit::Fr(1.0))
    }
}

/// line-style in `MultiColumn`
pub enum MultiColumnLineStyle {
    /// None style
    None,
    /// Hide the line style
    Hidden,
    /// Dotted line
    Dotted,
    /// Dashed line
    Dashed,
    /// Solid line
    Solid,
    /// Double line
    Double,
    /// Groove line
    Groove,
    /// Ridge line
    Ridge,
    /// Inset line
    Inset,
    /// OutSet line
    OutSet,
}

impl Default for MultiColumnLineStyle {
    fn default() -> MultiColumnLineStyle {
        MultiColumnLineStyle::None
    }
}
