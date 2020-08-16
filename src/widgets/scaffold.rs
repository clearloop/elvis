use elvis_core::{derive::Setter, Node};

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
        let [mut header, mut body, mut footer] = [self.header, self.body, self.footer];
        header.attr.tag = "body".to_string();
        body.attr.tag = "section".to_string();
        footer.attr.tag = "footer".to_string();
        Node::default().children(vec![header, body, footer])
    }
}
