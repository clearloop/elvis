use elvis::{
    prelude::*,
    style::traits::{BackgroundColor, MarginBottom, PaddingTop},
    traits::StyleWrapper,
    value::{BoxShadow, Color, FontFamily, TextAlign, Unit, VecUnit},
    widgets::{
        layouts::{Container, List},
        Scaffold, Text, TextField,
    },
};

#[page]
struct Index;

fn font() -> FontFamily {
    FontFamily::Derive(vec![FontFamily::Mix(
        Box::new(FontFamily::Helvetica),
        Box::new(FontFamily::Neue),
    )])
}

fn header() -> Node {
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

fn body() -> Node {
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
    .shadow(BoxShadow::Derive(vec![
        BoxShadow::Customize(vec![
            BoxShadow::Unit(Unit::None(0.0)),
            BoxShadow::Unit(Unit::Px(2.0)),
            BoxShadow::Unit(Unit::Px(4.0)),
            BoxShadow::Unit(Unit::Px(0.0)),
            BoxShadow::Color(Color::ORGB(0.2, 0, 0, 0)),
        ]),
        BoxShadow::Customize(vec![
            BoxShadow::Inset,
            BoxShadow::Unit(Unit::None(0.0)),
            BoxShadow::Unit(Unit::Px(-2.0)),
            BoxShadow::Unit(Unit::Px(1.0)),
            BoxShadow::Color(Color::ORGB(0.03, 0, 0, 0)),
        ]),
    ]))
    .wrap()
}

fn todoapp(children: Vec<Node>) -> Node {
    Container::with(List::new().children(children))
        .max_width(Unit::Px(550.0))
        .margin(VecUnit(vec![Unit::Auto]))
        .wrap()
}

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Scaffold::new()
            .body(todoapp(vec![header(), body(), body()]))
            .background_color(Color::ORGB(1.0, 245, 245, 245))
    }
}
