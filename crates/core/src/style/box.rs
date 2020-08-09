//! Basic style
use crate::{
    style::Border,
    style::Style,
    value::{layouts::Alignment, BoxShadow, Color, Unit, VecUnit},
};
use elvis_core_support::Setter;

/// `Container` style
#[derive(Clone, Default, Setter, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContainerStyle {
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

impl Into<Vec<Style>> for ContainerStyle {
    fn into(self) -> Vec<Style> {
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
            ]
        }

        if let Some(v) = self.align {
            styles.append(&mut v.into());
        }

        if let Some(v) = self.border {
            styles.append(&mut v.into());
        }

        styles
    }
}
