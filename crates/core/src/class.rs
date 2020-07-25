//! Elvis Class

/// Evlis classes
#[derive(Clone)]
pub enum Class {
    /// Center Class
    Center,
    /// Flex Class
    Flex,
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match self {
            Class::Center => "center",
            Class::Flex => "flex",
        }
        .into()
    }
}
