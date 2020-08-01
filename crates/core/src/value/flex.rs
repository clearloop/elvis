use crate::value::Unit;

/// `Flex` position
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum FlexPosition {
    /// Flex center
    Center,
    /// Flex end
    End,
    /// Flex start
    Start,
}

impl ToString for FlexPosition {
    fn to_string(&self) -> String {
        match self {
            FlexPosition::Center => "center",
            FlexPosition::End => "flex-end",
            FlexPosition::Start => "flex-start",
        }
        .into()
    }
}

impl Default for FlexPosition {
    fn default() -> FlexPosition {
        FlexPosition::Center
    }
}

/// `Flex` Alignment
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Alignment {
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

impl Default for Alignment {
    fn default() -> Alignment {
        Alignment::Center
    }
}

impl ToString for Alignment {
    fn to_string(&self) -> String {
        let (ai, jc) = match self {
            Alignment::BottomCenter => ("flex-end", "center"),
            Alignment::BottomLeft => ("flex-end", "flex-start"),
            Alignment::BottomRight => ("flex-end", "flex-end"),
            Alignment::Center => ("center", "center"),
            Alignment::CenterLeft => ("center", "flex-start"),
            Alignment::CenterRight => ("center", "flex-end"),
            Alignment::TopCenter => ("flex-start", "center"),
            Alignment::TopLeft => ("flex-start", "flex-start"),
            Alignment::TopRight => ("flex-start", "flex-end"),
        };

        format!("align-items: {}; justify-content: {};", ai, jc)
    }
}

/// `flex-basis` property
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
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
