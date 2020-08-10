use elvis::{
    prelude::*,
    style::ContainerStyle,
    traits::StyleWrapper,
    value::{BoxShadow, Color, FontFamily, Position, TextAlign, Unit},
    widgets::{
        layouts::{Container, List, Positioned},
        Text, TextField,
    },
};

#[page]
struct Index;

fn header() -> Node {
    Positioned::new()
        .child(
            Text::new()
                .text("todos")
                .color(Color::ORGB(0.15, 175, 47, 47))
                .size(Unit::Px(100.0))
                .weight(Unit::None(100.0))
                .height(Unit::Em(1.4))
                .family(FontFamily::Derive(vec![
                    FontFamily::Mix(Box::new(FontFamily::Helvetica), Box::new(FontFamily::Neue)),
                    FontFamily::Helvetica,
                    FontFamily::Neue,
                    FontFamily::Arial,
                ]))
                .align(TextAlign::Center),
        )
        .pos(Position::Absolute)
        .top(Unit::Px(-155.0))
        .into()
}

fn body() -> TextField {
    TextField::new()
        .text(
            Text::new()
                .text("hello")
                .height(Unit::Em(1.4))
                .size(Unit::Px(24.0)),
        )
        .style(
            ContainerStyle::new()
                .padding(vec![
                    Unit::Px(16.0),
                    Unit::Px(16.0),
                    Unit::Px(16.0),
                    Unit::Px(60.0),
                ])
                .shadow(BoxShadow::Customize(vec![
                    BoxShadow::Inset,
                    BoxShadow::Unit(Unit::None(0.0)),
                    BoxShadow::Unit(Unit::Px(-2.0)),
                    BoxShadow::Unit(Unit::Px(1.0)),
                    BoxShadow::Color(Color::ORGB(0.03, 0, 0, 0)),
                ])),
        )
}

fn todoapp(children: Vec<Node>) -> Node {
    Positioned::new()
        .child(List::new().children(children))
        .wrap()
}

impl LifeCycle for Index {
    fn create(&self) -> Node {
        todoapp(vec![
            header(),
            Container::new()
                .child(body())
                .style(
                    ContainerStyle::new()
                        .max_width(Unit::Px(550.0))
                        .margin(vec![Unit::None(0.0), Unit::Auto])
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
                        ])),
                )
                .into(),
        ])
    }
}
