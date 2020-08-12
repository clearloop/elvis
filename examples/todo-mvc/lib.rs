use elvis::{
    prelude::*,
    style::ContainerStyle,
    traits::StyleWrapper,
    value::{BoxShadow, Color, FontFamily, TextAlign, Unit, VecUnit},
    widgets::{
        layouts::{Container, List, Positioned},
        Text, TextField,
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
    let bg = Text::new()
        .text("todos")
        .color(Color::ORGB(0.15, 175, 47, 47))
        .size(Unit::Px(100.0))
        .weight(Unit::None(100.0))
        .family(font())
        .align(TextAlign::Center);

    bg.into()
}

fn body() -> Node {
    Container::new()
        .child(
            TextField::new().text(
                Text::new()
                    .height(Unit::Em(1.4))
                    .weight(Unit::None(100.0))
                    .family(font())
                    .size(Unit::Px(24.0)),
            ),
        )
        .style(
            ContainerStyle::new()
                .padding(vec![
                    Unit::Px(16.0),
                    Unit::Px(16.0),
                    Unit::Px(16.0),
                    Unit::Px(60.0),
                ])
                .height(Unit::Auto)
                .margin(VecUnit(vec![
                    Unit::Auto,
                    Unit::Auto,
                    Unit::Auto,
                    Unit::Auto,
                ]))
                .max_width(Unit::Px(550.0))
                .shadow(BoxShadow::Derive(vec![
                    BoxShadow::Customize(vec![
                        BoxShadow::Unit(Unit::None(0.0)),
                        BoxShadow::Unit(Unit::Px(2.0)),
                        BoxShadow::Unit(Unit::Px(4.0)),
                        BoxShadow::Unit(Unit::Px(0.0)),
                        BoxShadow::Color(Color::ORGB(0.2, 0, 0, 0)),
                    ]),
                    BoxShadow::Customize(vec![
                        BoxShadow::Unit(Unit::None(0.0)),
                        BoxShadow::Unit(Unit::Px(25.0)),
                        BoxShadow::Unit(Unit::Px(50.0)),
                        BoxShadow::Unit(Unit::Px(0.0)),
                        BoxShadow::Color(Color::ORGB(0.1, 0, 0, 0)),
                    ]),
                    BoxShadow::Customize(vec![
                        BoxShadow::Inset,
                        BoxShadow::Unit(Unit::None(0.0)),
                        BoxShadow::Unit(Unit::Px(-2.0)),
                        BoxShadow::Unit(Unit::Px(1.0)),
                        BoxShadow::Color(Color::ORGB(0.03, 0, 0, 0)),
                    ]),
                    BoxShadow::Customize(vec![
                        BoxShadow::Inset,
                        BoxShadow::Unit(Unit::None(0.0)),
                        BoxShadow::Unit(Unit::Px(-2.0)),
                        BoxShadow::Unit(Unit::Px(1.0)),
                        BoxShadow::Color(Color::ORGB(0.03, 0, 0, 0)),
                    ]),
                ])),
        )
        .wrap()
}

fn todoapp(children: Vec<Node>) -> Node {
    Positioned::new()
        .child(List::new().children(children))
        .wrap()
}

impl LifeCycle for Index {
    fn create(&self) -> Node {
        todoapp(vec![header(), body().into()])
    }
}
