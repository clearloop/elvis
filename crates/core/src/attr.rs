/// Node Attributes
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute {
    /// Node Id
    pub id: String,
    /// Node Tag
    pub tag: String,
    /// Source
    pub src: String,
    /// Text
    pub text: String,
}
