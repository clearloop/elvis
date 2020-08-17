use elvis::{
    prelude::*,
    style::traits::BackgroundColor,
    traits::StyleWrapper,
    value::{BoxShadow, Color, FontFamily, Unit, VecUnit},
    widgets::{
        layouts::{Container, List},
        Scaffold,
    },
};

mod widget;

#[page]
struct Index;

pub fn font() -> FontFamily {
    FontFamily::Derive(vec![FontFamily::Mix(
        Box::new(FontFamily::Helvetica),
        Box::new(FontFamily::Neue),
    )])
}

/// A litter style wrapper of our app
fn todoapp(children: Vec<Node>) -> Node {
    Container::with(List::new().children(children))
        .max_width(Unit::Px(550.0))
        .margin(VecUnit(vec![Unit::Auto]))
        .background_color(Color::White)
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
                BoxShadow::Unit(Unit::None(0.0)),
                BoxShadow::Color(Color::ORGB(0.1, 0, 0, 0)),
            ]),
        ]))
        .wrap()
}

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Scaffold::new()
            .header(widget::header())
            .body(todoapp(vec![widget::body(), widget::footer()]))
            .background_color(Color::ORGB(1.0, 245, 245, 245))
    }
}
