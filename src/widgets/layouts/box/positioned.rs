use elvis_core::{
    derive::{Setter, Wrapper},
    option_to_style,
    value::{Position, Unit},
    Node, Style, StyleWrapper,
};

/// Positioned Widget
#[derive(Default, Setter, Wrapper)]
pub struct Positioned {
    /// box child
    pub child: Node,
    /// box position
    pub pos: Option<Position>,
    /// position top
    pub top: Option<Unit>,
    /// position right
    pub right: Option<Unit>,
    /// position bottom
    pub bottom: Option<Unit>,
    /// position left
    pub left: Option<Unit>,
}

impl Into<Node> for Positioned {
    fn into(self) -> Node {
        let mut styles: Vec<Style> = vec![];
        option_to_style! {
            styles, [
                (Position, self.pos),
                (Top, self.top),
                (Right, self.right),
                (Bottom, self.bottom),
                (Left, self.left),
            ],
        }

        Node::default()
            .children(vec![self.child])
            .append_style(styles)
    }
}
