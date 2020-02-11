use crate::{
    Alignments, Colors, FlexBasis, FlexDirection, GridAutoRows, GridTemplate, MultiColumnStyle,
    Tree, Unit,
};

// Core Layout
/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
pub struct Container {
    pub child: Tree,
    pub style: ContainerStyle,
}

/// Container style
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

/// SizedBox just has width and height two arguments, we use this component to take some white space usually.
pub struct SizedBox {
    pub child: Tree,
    pub height: Unit,
    pub width: Unit,
}

// Flex
/// Align inherits the core usage of Alignments, it's quite simple, just one property.
pub struct Align {
    pub align: Alignments,
    pub child: Tree,
}

/// Align inherits the core usage of Alignment, it's quite simple, just one property.
pub struct Center {
    pub child: Tree,
}

/// Col is the typical flow in html, with flexible enhance.
pub struct Col {
    pub children: Vec<Tree>,
}

/// This is a Lunatic Widget to Ground Control, I'm stepping throw the Window.
pub struct Flex {
    pub basis: FlexBasis,
    pub direction: FlexDirection,
    pub grow: Unit,
    pub order: Unit,
    pub wrap: bool,
}

// Grid
/// Grid is quite complex in some way, usually, we just `Grid` our contains.
pub struct Grid {
    pub child: Tree,
    pub col: Unit,
    pub row: Unit,
    pub gap: Unit,
    pub template: GridTemplate,
    pub auto_rows: GridAutoRows,
}

// MultiColumn
/// Homework: code a New York Times.
pub struct MultiColumn {
    pub child: Tree,
    pub color: Colors,
    pub count: Unit,
    pub gap: Unit,
    pub style: MultiColumnStyle,
}
