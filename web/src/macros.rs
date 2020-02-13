/// shared `into` to elvis widgets
#[macro_export]
macro_rules! into {
    {$(($widget:ident, $target:ident),)*} => {
        $(
            impl Into<$target> for $widget {
                fn into(self) -> $target {
                    self.0
                }
            }
        )*
    };
}

/// shared `deref` to elvis widgets
#[macro_export]
macro_rules! deref {
    ($name: ident, $target: ident) => {
        impl Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
