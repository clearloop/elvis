use crate::font;
use elvis::{
    prelude::Node,
    style::traits::{MarginBottom, PaddingTop},
    value::{Color, TextAlign, Unit},
    widgets::Text,
};

/// The header of todo-mvc
pub fn header() -> Node {
    let bg = Text::with("todos")
        .color(Color::ORGB(0.15, 175, 47, 47))
        .size(Unit::Px(100.0))
        .weight(Unit::None(100.0))
        .family(font())
        .align(TextAlign::Center)
        .padding_top(Unit::None(30.0))
        .margin_bottom(Unit::None(30.0));

    bg.into()
}
