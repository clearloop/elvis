//! Elvis layout values
use crate::value::Unit;

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

impl ToString for Alignments {
    fn to_string(&self) -> String {
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

impl ToString for FlexBasis {
    fn to_string(&self) -> String {
        format!(
            "flex-basis: {};",
            match self {
                FlexBasis::Fill => "fill".into(),
                FlexBasis::FitContent => "fit-content".into(),
                FlexBasis::MaxContent => "max-content".into(),
                FlexBasis::MinContent => "min-content".into(),
                FlexBasis::Number(u) => u.to_string(),
            }
        )
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

impl ToString for FlexDirection {
    fn to_string(&self) -> String {
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

impl ToString for GridAuto {
    fn to_string(&self) -> String {
        match self {
            GridAuto::Auto => "auto".to_string(),
            GridAuto::MaxContent => "max-content".to_string(),
            GridAuto::MinContent => "min-content".to_string(),
            GridAuto::MinMax(c, r) => format!("minmax({}, {})", c.to_string(), r.to_string()),
            GridAuto::Fixed(u) => u.to_string(),
            GridAuto::Inherit => "inherit".to_string(),
            GridAuto::Initial => "initial".to_string(),
            GridAuto::Unset => "unset".to_string(),
            GridAuto::Plain(x) => {
                let mut s = "".to_string();
                for i in x {
                    s += &i.to_string();
                    s += " ";
                }

                s
            }
        }
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

impl ToString for GridFlow {
    fn to_string(&self) -> String {
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

impl ToString for GridTemplate {
    fn to_string(&self) -> String {
        match self {
            GridTemplate::FitContent(u) => format!("fit-content({})", u.to_string()),
            GridTemplate::Inherit => "inherit".to_string(),
            GridTemplate::Initial => "initial".to_string(),
            GridTemplate::MinMax(i, a) => format!("minmax({}, {})", i.to_string(), a.to_string()),
            GridTemplate::None => "none".to_string(),
            GridTemplate::Plain(x) => {
                let mut s = "".to_string();
                for i in x {
                    s += &i.to_string();
                    s += " ";
                }

                s
            }
            GridTemplate::Repeat(n, u) => format!("({}, {})", n, u.to_string()),
            GridTemplate::SubGrid => "subgrid".to_string(),
            GridTemplate::Unset => "unit".to_string(),
        }
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

impl ToString for MultiColumnLineStyle {
    fn to_string(&self) -> String {
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
