use crate::widgets::ListTile;
use elvis_core::{
    derive::Setter,
    option_to_style,
    style::Border,
    value::{Color, FontFamily, FontStyle, TextAlign, Unit},
    Attribute, Node, Style,
};

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[derive(Default, Setter)]
pub struct Text {
    /// Plain text
    #[skip]
    pub text: String,
    /// Bold text
    pub bold: bool,
    /// The color of the text
    pub color: Option<Color>,
    /// Italic text
    pub italic: bool,
    /// Text size
    pub size: Option<Unit>,
    /// Text weight
    pub weight: Option<Unit>,
    /// Text height
    pub height: Option<Unit>,
    /// Text stretch
    pub stretch: Option<Unit>,
    /// Font Family
    pub family: Option<FontFamily>,
    /// Text Align
    pub align: Option<TextAlign>,
}

impl Text {
    /// Set Text
    pub fn text(mut self, s: &str) -> Self {
        self.text = s.into();
        self
    }
}

impl Into<Node> for Text {
    fn into(mut self) -> Node {
        let mut child = Node::default();
        child.attr.tag = "plain".into();
        child.attr.text = self.text.to_string();

        let mut styles: Vec<Style> = vec![];
        if self.italic {
            styles.push(Style::FontStyle(FontStyle::Normal));
        }

        if self.bold {
            self.weight = Some(Unit::None(700.0));
        }

        option_to_style! {
            styles, [
                (Color, self.color),
                (FontWeight, self.weight),
                (FontSize, self.size),
                (FontStretch, self.stretch),
                (LineHeight, self.height),
                (FontFamily, self.family),
                (TextAlign, self.align),
            ],
        }

        let mut node = Node::default().children(vec![child]).style(styles);

        // Set Tag
        node.attr.tag = "p".into();
        node
    }
}

/// Text Field
#[derive(Default, Setter)]
pub struct TextField {
    /// Leading widget
    #[skip]
    pub leading: Node,
    /// Trailing widget
    #[skip]
    pub trailing: Node,
    /// Plain text
    pub text: Text,
}

impl TextField {
    /// Set leading
    pub fn leading(mut self, l: impl Into<Node>) -> Self {
        self.leading = l.into();
        self
    }

    /// Set trailing
    pub fn trailing(mut self, t: impl Into<Node>) -> Self {
        self.trailing = t.into();
        self
    }
}

impl Into<Node> for TextField {
    fn into(self) -> Node {
        let mut style: Vec<Style> = Border::default().into();
        style.append(&mut vec![
            Style::Width(Unit::Percent(100.0)),
            Style::OutlineWidth(Unit::None(0.0)),
        ]);

        ListTile::new()
            .leading(self.leading)
            .text(
                Into::<Node>::into(self.text)
                    .attr(Attribute::new().tag("input"))
                    .append_style(style),
            )
            .trailing(self.trailing)
            .into()
    }
}
