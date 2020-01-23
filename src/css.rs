/// CSS generator
#[derive(Debug)]
pub struct CSS(pub String);

impl AsRef<str> for CSS {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

use std::any::{Any, TypeId};
macro_rules! impl_tys {
    (
        $($key:ident,)*
    ) => {
        impl CSS {
            $(
                pub fn $key(&mut self, v: &str) {
                    self.0.push_str(&format!("{}: {};", stringify!($key), v));
                }
            )*
        }
    };
}

impl_tys!(height, width, color,);
