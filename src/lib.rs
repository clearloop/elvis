#[macro_use]
extern crate elvis_macro;

mod elvis_element;
mod elvis_text;
mod page;
pub use elvis_element::{ElvisElement, IElvisElementArgs};
pub use elvis_text::{ElvisText, ITextStyle};
pub use page::Page;
