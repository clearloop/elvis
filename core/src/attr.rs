use elvis_core_support::Setter;

/// Node Attributes
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Setter)]
pub struct Attribute {
    /// Node Id
    pub id: String,
    /// Node Tag
    pub tag: String,
    /// Source
    pub src: String,
    /// Href
    pub href: String,
    /// Text
    pub text: String,
    /// Type
    pub r#type: String,
}
