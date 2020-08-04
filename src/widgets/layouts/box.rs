//! Basic layout widgets
use elvis_core::{
    derive::Setter,
    style::ContainerStyle,
    value::{Position, Unit},
    Class, Node, Style,
};
use elvis_support::IntoNode;

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
#[derive(Default, IntoNode, Setter)]
pub struct Container {
    /// Container child
    pub child: Node,
    /// Container style
    pub style: ContainerStyle,
}

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

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
#[derive(Default, Setter)]
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
}

impl Into<Node> for Positioned {
    fn into(self) -> Node {
        Node::default().children(vec![self.child]).style(vec![
            Style::Position(self.pos),
            Style::Top(self.top),
            Style::Right(self.right),
            Style::Bottom(self.bottom),
            Style::Left(self.left),
        ])
    }
}
