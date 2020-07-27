//! Elvis Class

/// Evlis classes
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Class {
    /// Center Class
    Center,
    /// Flex Class
    Flex,
    /// Row Class
    Row,
    /// Column Class
    Col,
    /// Empty Class
    Empty,
}

impl From<&str> for Class {
    fn from(s: &str) -> Class {
        match s {
            "center" => Class::Center,
            "flex" => Class::Flex,
            "row" => Class::Row,
            "col" => Class::Col,
            _ => Class::Empty,
        }
    }
}
