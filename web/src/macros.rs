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
