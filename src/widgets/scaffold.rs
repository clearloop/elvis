use elvis_core::{derive::Setter, value::Unit, Node, Style};

/// App Scaffold
#[derive(Default, Setter)]
pub struct Scaffold {
    /// App Header
    pub header: Node,
    /// App Footer
    pub body: Node,
    /// App footer
    pub footer: Node,
}

impl Into<Node> for Scaffold {
    fn into(self) -> Node {
        let mut nodes = vec![];
        for n in [self.header, self.body, self.footer].iter() {
            if !n.children.is_empty() {
                let mut node = n.clone();
                node.attr.tag = "section".to_string();
                nodes.push(node);
            }
        }
        Node::default().children(nodes).style(vec![
            Style::Height(Unit::Percent(100.0)),
            Style::Width(Unit::Percent(100.0)),
        ])
    }
}
