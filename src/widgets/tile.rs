use elvis_core::{derive::Setter, value::Unit, Class, Node, Style};

/// Tile component, usually used in in list
#[derive(Default, Setter)]
pub struct ListTile {
    leading: Node,
    text: Node,
    trailing: Node,
}

impl Into<Node> for ListTile {
    fn into(self) -> Node {
        Node::default()
            .children(vec![
                Node::default()
                    .children(vec![self.leading, self.text])
                    .style(vec![Style::Width(Unit::Percent(100.0))]),
                self.trailing,
            ])
            .class(vec![Class::Flex, Class::Row])
    }
}
