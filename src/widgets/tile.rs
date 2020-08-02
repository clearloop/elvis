use elvis_core::{Class, Node};

/// Tile component, usually used in in list
pub struct ListTile {
    leading: Node,
    text: Node,
    trailing: Node,
}

impl Into<Node> for ListTile {
    fn into(self) -> Node {
        Node::default()
            .children(vec![
                Node::default().children(vec![self.leading, self.text]),
                self.trailing,
            ])
            .class(vec![Class::Row])
    }
}
