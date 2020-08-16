use elvis::{
    prelude::*,
    style::traits::BackgroundColor,
    traits::StyleWrapper,
    value::{Color, FontFamily, Unit, VecUnit},
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
        .wrap()
}

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Scaffold::new()
            .body(todoapp(vec![widget::header(), widget::body()]))
            .background_color(Color::ORGB(1.0, 245, 245, 245))
    }
}
