//! Elvis Class

/// Evlis classes
#[derive(Clone)]
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

macro_rules! construct_class {
    ($(($n:ident, $s:expr),)*) => {
        impl ToString for Class {
            fn to_string(&self) -> String {
                match self {
                    $(
                        Class::$n => $s,
                    )*
                    Class::Empty => "",
                }.to_string()
            }
        }

        impl AsRef<str> for Class {
            fn as_ref(&self) -> &str {
                match self {
                    $(
                        Class::$n => $s,
                    )*
                    Class::Empty => "",
                }
            }
        }

        impl From<&str> for Class {
            fn from(s: &str) -> Class {
                match s {
                    $(
                        $s => Class::$n,
                    )*
                    _ => Class::Empty,
                }
            }
        }
    }
}

construct_class! {
    (Center, "center"),
    (Flex, "flex"),
    (Row, "row"),
    (Col, "col"),
}
