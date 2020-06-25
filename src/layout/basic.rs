use crate::{
    values::layout::Alignments,
    values::{Colors, Unit},
    Tree,
};

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
pub struct Container {
    pub child: Tree,
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
    pub children: Vec<Tree>,
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
pub struct SizedBox {
    pub child: Tree,
    pub style: SizedBoxStyle,
}

/// `SizedBox` style
pub struct SizedBoxStyle {
    pub height: Unit,
    pub width: Unit,
}
