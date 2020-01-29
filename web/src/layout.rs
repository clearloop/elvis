use crate::Element;
use wasm_bindgen::prelude::*;

/// Alignmment
///
/// follow the [MDN CSS Box Alignmment][1]
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Alignment
#[wasm_bindgen]
pub enum Alignments {
    // Positional alignment keyword values
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
    Left,
    Right,

    // Baseline alignment
    Baseline,
    FirstBaseline,
    LastBaseline,

    // Distributed alignment
    Stretch,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

/// Layout contains Box, Flex, and Grid.
pub struct Layout;
impl Layout {
    pub fn align(_child: Element, _direction: &str) -> Result<Element, JsValue> {
        let container = Element::new("div")?;
        Ok(container)
    }

    /// layout use flex.
    pub fn center(child: Element) -> Result<Element, JsValue> {
        Ok(Element::new("div")?
            .css(
                &vec![
                    "height: 100%;",
                    "width: 100%;",
                    "display: flex;",
                    "flex-wrap: wrap;",
                    "align-content: center;",
                    "justify-content: center;",
                ]
                .join(""),
            )
            .append_child(child)?)
    }
}

/// CSS Flexible Box Layout
///
/// [MDN doc]https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout
mod flex {}
