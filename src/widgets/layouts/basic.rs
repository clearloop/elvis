use crate::{
    widgets::values::{layouts::Alignments, Colors, Unit},
    Node,
};

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
pub struct Container {
    pub child: Node,
    pub style: ContainerStyle,
}

/// `Container` style
#[derive(Default)]
pub struct ContainerStyle {
    pub align: Alignments,
    pub height: Unit,
    pub width: Unit,
    pub padding: Unit,
    pub margin: Unit,
    pub background_color: Colors,
}

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
pub struct List {
    pub children: Vec<Node>,
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
pub struct SizedBox {
    pub child: Node,
    pub style: SizedBoxStyle,
}

/// `SizedBox` style
pub struct SizedBoxStyle {
    pub height: Unit,
    pub width: Unit,
}
