//! Shared macros
#![macro_use]

/// Doc with expr
#[macro_export]
macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

pub use elvis_core_support::Setter;
