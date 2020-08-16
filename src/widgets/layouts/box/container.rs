use elvis_core::{
    derive::{Setter, Wrapper},
    option_to_style,
    style::{Border, Style},
    value::{layouts::Alignment, BoxShadow, Color, Unit, VecUnit},
    Node, StyleWrapper,
};

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
#[derive(Default, Setter, Wrapper)]
pub struct Container {
    /// Container child
    pub child: Node,
    /// Container align
    pub align: Option<Alignment>,
    /// Container height
    pub height: Option<Unit>,
    /// SizedBox Max Height
    pub max_height: Option<Unit>,
    /// SizedBox Max Width
    pub max_width: Option<Unit>,
    /// Container width
    pub width: Option<Unit>,
    /// Container padding
    pub padding: Option<VecUnit>,
    /// Container margin
    pub margin: Option<VecUnit>,
    /// Container background
    pub background_color: Option<Color>,
    /// Container Border
    pub border: Option<Border>,
    /// Box Shadow
    pub shadow: Option<BoxShadow>,
}

impl Container {
    /// Shortcut of `Container::new().child(impl Into<Node>)`
    pub fn with(child: impl Into<Node>) -> Container {
        Container::new().child(child)
    }
}

impl Into<Node> for Container {
    fn into(self) -> Node {
        let mut styles: Vec<Style> = vec![];
        option_to_style! {
            styles, [
                (Height, self.height),
                (Width, self.width),
                (MaxHeight, self.max_height),
                (MaxWidth, self.max_width),
                (Padding, self.padding),
                (Margin, self.margin),
                (BackgroundColor, self.background_color),
                (BoxShadow, self.shadow),
            ],[
                self.align,
                self.border,
            ],
        }

        Node::default().style(styles).children(vec![self.child])
    }
}
