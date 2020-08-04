use elvis::{
    prelude::*,
    style::{ContainerStyle, TextStyle},
    value::{BoxShadow, Color, FontFamily, TextAlign, Unit},
    widgets::{
        layouts::{Col, Container},
        Text, TextField,
    },
};

#[page]
struct Index;

fn title() -> Text {
    Text::new().text("todos").style(
        TextStyle::new()
            .color(Color::ORGB(0.15, 175, 47, 47))
            .size(Unit::Px(100.0))
            .weight(Unit::None(100.0))
            .family(FontFamily::Derive(vec![
                FontFamily::Mix(Box::new(FontFamily::Helvetica), Box::new(FontFamily::Neue)),
                FontFamily::Helvetica,
                FontFamily::Neue,
                FontFamily::Arial,
            ]))
            .align(TextAlign::Center),
    )
}

fn input() -> TextField {
    TextField::new()
        .text(
            Text::new()
                .text("hello")
                .style(TextStyle::new().height(Unit::Em(1.4)).size(Unit::Px(24.0))),
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

impl LifeCycle<Container> for Index {
    fn create(&self) -> Container {
        Container::new()
            .child(Col::new().children(vec![title().into(), input().into()]))
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
    }
}
