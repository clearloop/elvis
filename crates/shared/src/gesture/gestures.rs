//! Gesture traits
use crate::{doc_comment, Closure};

/// Construct gestures
macro_rules! construct_gesture {
    ($([$name:tt, $method:tt],)*) => {$(construct_gesture!([$name, $method]);)*};
    ([$name:tt, $method:tt]) => {
        doc_comment!{
            concat!("The `", stringify!($name), "` gesture"),
            pub trait $name {
                doc_comment!{
                    concat!("Trigger when `", stringify!($method), "` widget"),
                    fn $method<P>() -> Option<Box<Closure<P>>> {
                        None
                    }
                }
            }
        }
    }
}

construct_gesture! {
    [Tap, tap],
    [LongTap, long_tap],
}
