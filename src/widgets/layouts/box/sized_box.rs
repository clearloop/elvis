use elvis_core::{derive::Setter, value::Unit, Node, Style};

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
#[derive(Default, Setter)]
pub struct SizedBox {
    /// SizedBox child
    pub child: Node,
    /// SizedBox height
    pub height: Unit,
    /// SizedBox width
    pub width: Unit,
    /// SizedBox Max Height
    pub max_height: Unit,
    /// SizedBox Max Width
    pub max_width: Unit,
}

impl Into<Node> for SizedBox {
    fn into(self) -> Node {
        Node::default().children(vec![self.child]).style(vec![
            Style::Height(self.height),
            Style::Width(self.width),
            Style::MaxHeight(self.max_height),
            Style::MaxWidth(self.max_width),
        ])
    }
}
