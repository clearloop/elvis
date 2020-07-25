//! Basic layout widgets
use crate::Node;
use elvis_core::value::{layouts::Alignments, Colors, Unit};

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
pub struct Container {
    /// Container child
    pub child: Node,
    /// Container style
    pub style: ContainerStyle,
}

/// `Container` style
#[derive(Default)]
pub struct ContainerStyle {
    /// Container align
    pub align: Alignments,
    /// Container height
    pub height: Unit,
    /// Container width
    pub width: Unit,
    /// Container padding
    pub padding: Unit,
    /// Container margin
    pub margin: Unit,
    /// Container background
    pub background_color: Colors,
}

impl ToString for ContainerStyle {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        s += &self.align.to_string();
        s += &format!("height: {};", self.height.to_string());
        s += &format!("width: {};", self.width.to_string());
        s += &format!("padding: {};", self.padding.to_string());
        s += &format!("margin: {};", self.margin.to_string());
        s += &format!("background-color: {};", self.background_color.to_string());
        s
    }
}

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
pub struct List {
    /// List children
    pub children: Vec<Node>,
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
pub struct SizedBox {
    /// SizedBox child
    pub child: Node,
    /// SizedBox style
    pub style: SizedBoxStyle,
}

/// `SizedBox` style
pub struct SizedBoxStyle {
    /// SizedBox height
    pub height: Unit,
    /// SizedBox width
    pub width: Unit,
}

impl ToString for SizedBoxStyle {
    fn to_string(&self) -> String {
        format!(
            "height: {}; width: {};",
            self.height.to_string(),
            self.width.to_string()
        )
    }
}
