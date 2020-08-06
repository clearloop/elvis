use elvis_core::{derive::Setter, Node, Style};

/// Style wrapper widget
#[derive(Default, Setter)]
pub struct StyleWrapper {
    #[skip]
    style: Vec<Style>,
    child: Node,
}

impl StyleWrapper {
    /// Push style
    pub fn push_style(mut self, style: Style) -> Self {
        self.style.push(style);
        self
    }

    /// Append style
    pub fn append_style(mut self, mut style: Vec<Style>) -> Self {
        self.style.append(&mut style);
        self
    }
}

impl Into<Node> for StyleWrapper {
    fn into(self) -> Node {
        Into::<Node>::into(self.child).append_style(self.style)
    }
}
