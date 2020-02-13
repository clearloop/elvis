use elvis::{
    Alignments as ElvisAlignments, FlexBasis as ElvisFlexBasis,
    FlexDirection as ElvisFlexDirection, GridAuto as ElvisGridAuto, GridFlow as ElvisGridFlow,
    GridTemplate as ElvisGridTemplate, MultiColumnLineStyle as ElvisMultiColumnLineStyle, Unit,
};
use std::convert::Into;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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

impl Into<ElvisAlignments> for Alignments {
    fn into(self) -> ElvisAlignments {
        match self {
            Alignments::BottomCenter => ElvisAlignments::BottomCenter,
            Alignments::BottomLeft => ElvisAlignments::BottomLeft,
            Alignments::BottomRight => ElvisAlignments::BottomRight,
            Alignments::Center => ElvisAlignments::Center,
            Alignments::CenterLeft => ElvisAlignments::CenterLeft,
            Alignments::CenterRight => ElvisAlignments::CenterRight,
            Alignments::TopCenter => ElvisAlignments::TopCenter,
            Alignments::TopLeft => ElvisAlignments::TopLeft,
            Alignments::TopRight => ElvisAlignments::TopRight,
        }
    }
}

#[wasm_bindgen]
pub struct FlexBasis(ElvisFlexBasis);

#[wasm_bindgen]
impl FlexBasis {
    #[wasm_bindgen(js_name = "Fill")]
    pub fn fill() -> FlexBasis {
        FlexBasis(ElvisFlexBasis::Fill)
    }

    #[wasm_bindgen(js_name = "MaxContent")]
    pub fn max_content() -> FlexBasis {
        FlexBasis(ElvisFlexBasis::MaxContent)
    }

    #[wasm_bindgen(js_name = "MinContent")]
    pub fn min_content() -> FlexBasis {
        FlexBasis(ElvisFlexBasis::MinContent)
    }

    #[wasm_bindgen(js_name = "FitContent")]
    pub fn fit_content() -> FlexBasis {
        FlexBasis(ElvisFlexBasis::FitContent)
    }

    #[wasm_bindgen(js_name = "Number")]
    pub fn number(n: f64) -> FlexBasis {
        FlexBasis(ElvisFlexBasis::Number(Unit::Rem(n)))
    }
}

#[wasm_bindgen]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl Into<ElvisFlexDirection> for FlexDirection {
    fn into(self) -> ElvisFlexDirection {
        match self {
            FlexDirection::Column => ElvisFlexDirection::Column,
            FlexDirection::ColumnReverse => ElvisFlexDirection::ColumnReverse,
            FlexDirection::Row => ElvisFlexDirection::Row,
            FlexDirection::RowReverse => ElvisFlexDirection::RowReverse,
        }
    }
}

#[wasm_bindgen]
pub struct GridAuto(ElvisGridAuto);

#[wasm_bindgen]
impl GridAuto {
    #[wasm_bindgen(js_name = "Auto")]
    pub fn auto() -> GridAuto {
        GridAuto(ElvisGridAuto::Auto)
    }

    #[wasm_bindgen(js_name = "Fixed")]
    pub fn fixed(n: f64) -> GridAuto {
        GridAuto(ElvisGridAuto::Fixed(Unit::Fr(n)))
    }

    #[wasm_bindgen(js_name = "Inherit")]
    pub fn inherit() -> GridAuto {
        GridAuto(ElvisGridAuto::Inherit)
    }

    #[wasm_bindgen(js_name = "Initial")]
    pub fn initial() -> GridAuto {
        GridAuto(ElvisGridAuto::Initial)
    }

    #[wasm_bindgen(js_name = "MaxContent")]
    pub fn max_content() -> GridAuto {
        GridAuto(ElvisGridAuto::MaxContent)
    }

    #[wasm_bindgen(js_name = "MinContent")]
    pub fn min_content() -> GridAuto {
        GridAuto(ElvisGridAuto::MinContent)
    }

    #[wasm_bindgen(js_name = "MinMax")]
    pub fn min_max(i: f64, a: f64) -> GridAuto {
        GridAuto(ElvisGridAuto::MinMax(Unit::Fr(i), Unit::Fr(a)))
    }

    #[wasm_bindgen(js_name = "Plain")]
    pub fn plain(src: Vec<f64>) -> GridAuto {
        let us = src.iter().map(|n| Unit::Fr(*n)).collect::<Vec<Unit>>();
        GridAuto(ElvisGridAuto::Plain(us))
    }

    #[wasm_bindgen(js_name = "Unset")]
    pub fn unset() -> GridAuto {
        GridAuto(ElvisGridAuto::Unset)
    }
}

#[wasm_bindgen]
pub struct GridFlow(ElvisGridFlow);

#[wasm_bindgen]
impl GridFlow {
    #[wasm_bindgen(js_name = "Column")]
    pub fn col() -> GridFlow {
        GridFlow(ElvisGridFlow::Column)
    }

    #[wasm_bindgen(js_name = "ColumnDense")]
    pub fn col_dense() -> GridFlow {
        GridFlow(ElvisGridFlow::ColumnDense)
    }

    #[wasm_bindgen(js_name = "Inherit")]
    pub fn inherit() -> GridFlow {
        GridFlow(ElvisGridFlow::Inherit)
    }

    #[wasm_bindgen(js_name = "Initial")]
    pub fn initial() -> GridFlow {
        GridFlow(ElvisGridFlow::Initial)
    }

    #[wasm_bindgen(js_name = "Row")]
    pub fn row() -> GridFlow {
        GridFlow(ElvisGridFlow::Row)
    }

    #[wasm_bindgen(js_name = "RowDense")]
    pub fn row_dense() -> GridFlow {
        GridFlow(ElvisGridFlow::RowDense)
    }

    #[wasm_bindgen(js_name = "Unset")]
    pub fn unset() -> GridFlow {
        GridFlow(ElvisGridFlow::Unset)
    }
}

#[wasm_bindgen]
pub struct GridTemplate(ElvisGridTemplate);

#[wasm_bindgen]
impl GridTemplate {
    #[wasm_bindgen(js_name = "FitContent")]
    pub fn fit_content(u: f64) -> GridTemplate {
        GridTemplate(ElvisGridTemplate::FitContent(Unit::Percent(u)))
    }

    #[wasm_bindgen(js_name = "Inherit")]
    pub fn inherit() -> GridTemplate {
        GridTemplate(ElvisGridTemplate::Inherit)
    }

    #[wasm_bindgen(js_name = "Initial")]
    pub fn initial() -> GridTemplate {
        GridTemplate(ElvisGridTemplate::Initial)
    }

    #[wasm_bindgen(js_name = "MinMax")]
    pub fn min_max(i: f64, a: f64) -> GridTemplate {
        GridTemplate(ElvisGridTemplate::MinMax(Unit::Fr(i), Unit::Fr(a)))
    }

    #[wasm_bindgen(js_name = "None")]
    pub fn none() -> GridTemplate {
        GridTemplate(ElvisGridTemplate::None)
    }

    #[wasm_bindgen(js_name = "Plain")]
    pub fn plain(src: Vec<f64>) -> GridTemplate {
        let us = src.iter().map(|n| Unit::Fr(*n)).collect::<Vec<Unit>>();
        GridTemplate(ElvisGridTemplate::Plain(us))
    }

    #[wasm_bindgen(js_name = "Repeat")]
    pub fn repeat(t: i32, s: f64) -> GridTemplate {
        GridTemplate(ElvisGridTemplate::Repeat(t, Unit::Fr(s)))
    }

    pub fn sub_grid() -> GridTemplate {
        GridTemplate(ElvisGridTemplate::SubGrid)
    }
}

into! {
    (FlexBasis, ElvisFlexBasis),
    (GridAuto, ElvisGridAuto),
    (GridFlow, ElvisGridFlow),
    (GridTemplate, ElvisGridTemplate),
}

#[wasm_bindgen]
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

impl Into<ElvisMultiColumnLineStyle> for MultiColumnLineStyle {
    fn into(self) -> ElvisMultiColumnLineStyle {
        match self {
            MultiColumnLineStyle::None => ElvisMultiColumnLineStyle::None,
            MultiColumnLineStyle::Hidden => ElvisMultiColumnLineStyle::Hidden,
            MultiColumnLineStyle::Dotted => ElvisMultiColumnLineStyle::Dotted,
            MultiColumnLineStyle::Dashed => ElvisMultiColumnLineStyle::Dashed,
            MultiColumnLineStyle::Solid => ElvisMultiColumnLineStyle::Solid,
            MultiColumnLineStyle::Double => ElvisMultiColumnLineStyle::Double,
            MultiColumnLineStyle::Groove => ElvisMultiColumnLineStyle::Groove,
            MultiColumnLineStyle::Ridge => ElvisMultiColumnLineStyle::Ridge,
            MultiColumnLineStyle::Inset => ElvisMultiColumnLineStyle::Inset,
            MultiColumnLineStyle::OutSet => ElvisMultiColumnLineStyle::OutSet,
        }
    }
}
