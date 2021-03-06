use crate::font;
use elvis::{
    prelude::Node,
    style::traits::{
        Border, BorderRadius as BorderRadiusTrait, BorderTop, Display as DisplayTrait, Margin,
        Padding, TextAlign as TextAlignTrait,
    },
    value::{
        BorderRadius, BorderStyle, BoxBorder, BoxShadow, Color, Display, TextAlign, Unit, VecUnit,
    },
    widgets::{
        layouts::{Container, List},
        Text,
    },
};

pub fn button(text: &str) -> Node {
    Text::with(text)
        .family(font())
        .height(Unit::None(1.2))
        .weight(Unit::None(100.0))
        .border(
            BoxBorder::new()
                .width(Unit::Px(1.0))
                .style(BorderStyle::Solid)
                .color(Color::ORGB(0.2, 175, 47, 47)),
        )
        .padding(VecUnit(vec![Unit::Px(3.0), Unit::Px(7.0)]))
        .margin(VecUnit(vec![Unit::Px(3.0)]))
        .border_radius(BorderRadius::new().all(Unit::Px(3.0)))
        .display(Display::InlineBlock)
}

pub fn footer() -> Node {
    Container::with(
        List::with(vec![button("All"), button("Active"), button("Completed")])
            .padding(VecUnit(vec![Unit::Px(10.0), Unit::Px(15.0)]))
            .text_align(TextAlign::Center)
            .border_top(
                BoxBorder::with(Unit::Px(1.0))
                    .style(BorderStyle::Solid)
                    .color(Color::ORGB(1.0, 230, 230, 230)),
            ),
    )
    .shadow(BoxShadow::Derive(vec![
        BoxShadow::Customize(vec![
            BoxShadow::Unit(Unit::None(0.0)),
            BoxShadow::Unit(Unit::Px(1.0)),
            BoxShadow::Unit(Unit::Px(1.0)),
            BoxShadow::Color(Color::ORGB(0.2, 0, 0, 0)),
        ]),
        BoxShadow::Customize(vec![
            BoxShadow::Unit(Unit::None(0.0)),
            BoxShadow::Unit(Unit::Px(8.0)),
            BoxShadow::Unit(Unit::Px(0.0)),
            BoxShadow::Unit(Unit::Px(-3.0)),
            BoxShadow::Color(Color::ORGB(1.0, 246, 246, 246)),
        ]),
        BoxShadow::Customize(vec![
            BoxShadow::Unit(Unit::None(0.0)),
            BoxShadow::Unit(Unit::Px(9.0)),
            BoxShadow::Unit(Unit::Px(1.0)),
            BoxShadow::Unit(Unit::Px(-3.0)),
            BoxShadow::Color(Color::ORGB(0.2, 0, 0, 0)),
        ]),
        BoxShadow::Customize(vec![
            BoxShadow::Unit(Unit::None(0.0)),
            BoxShadow::Unit(Unit::Px(16.0)),
            BoxShadow::Unit(Unit::Px(0.0)),
            BoxShadow::Unit(Unit::Px(-6.0)),
            BoxShadow::Color(Color::ORGB(1.0, 246, 246, 246)),
        ]),
        BoxShadow::Customize(vec![
            BoxShadow::Unit(Unit::None(0.0)),
            BoxShadow::Unit(Unit::Px(17.0)),
            BoxShadow::Unit(Unit::Px(2.0)),
            BoxShadow::Unit(Unit::None(-6.0)),
            BoxShadow::Color(Color::ORGB(0.2, 0, 0, 0)),
        ]),
    ]))
    .into()
}
