use crate::font;
use elvis::{
    prelude::*,
    traits::StyleWrapper,
    value::{BoxShadow, Color, Unit},
    widgets::{layouts::Container, Text, TextField},
};

pub fn text() -> Node {
    Container::with(TextField::with(
        Text::new()
            .height(Unit::Em(1.4))
            .weight(Unit::None(100.0))
            .family(font())
            .size(Unit::Px(24.0)),
    ))
    .padding(vec![
        Unit::Px(16.0),
        Unit::Px(16.0),
        Unit::Px(16.0),
        Unit::Px(60.0),
    ])
    .height(Unit::Auto)
    .shadow(BoxShadow::Derive(vec![BoxShadow::Customize(vec![
        BoxShadow::Inset,
        BoxShadow::Unit(Unit::None(0.0)),
        BoxShadow::Unit(Unit::Px(-2.0)),
        BoxShadow::Unit(Unit::Px(1.0)),
        BoxShadow::Color(Color::ORGB(0.03, 0, 0, 0)),
    ])]))
    .background_color(Color::White)
    .wrap()
}

pub fn body() -> Node {
    text()
}
