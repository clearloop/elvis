use elvis_core::{
    derive::{Setter, Wrapper},
    value::{Position, Unit, VecUnit},
    Node, Style, StyleWrapper,
};

/// Positioned Widget
#[derive(Setter, Wrapper)]
pub struct Positioned {
    /// box child
    pub child: Node,
    /// box position
    pub pos: Position,
    /// position top
    pub top: Unit,
    /// position right
    pub right: Unit,
    /// position bottom
    pub bottom: Unit,
    /// position left
    pub left: Unit,
    /// position margin
    pub margin: VecUnit,
    /// position padding
    pub padding: VecUnit,
}

impl Default for Positioned {
    fn default() -> Positioned {
        Positioned {
            child: Default::default(),
            pos: Position::Relative,
            top: Unit::None(0.0),
            right: Unit::None(0.0),
            bottom: Unit::None(0.0),
            left: Unit::None(0.0),
            margin: VecUnit(vec![Unit::None(0.0)]),
            padding: VecUnit(vec![Unit::None(0.0)]),
        }
    }
}

impl Into<Node> for Positioned {
    fn into(self) -> Node {
        Node::default()
            .children(vec![self.child])
            .append_style(vec![
                Style::Position(self.pos),
                Style::Top(self.top),
                Style::Right(self.right),
                Style::Bottom(self.bottom),
                Style::Left(self.left),
                Style::Width(Unit::Percent(100.0)),
                Style::Margin(self.margin),
                Style::Padding(self.padding),
            ])
    }
}
