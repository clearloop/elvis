use crate::{driver::WebDriver, event::EventListener};
use elvis_shared::{Driver, Node};

/// Bind gesture to node
pub fn bind(node: &Node) {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("button").unwrap();
    val.set_inner_html("Hello from Rust!");

    EventListener::new(&val, "click", |_| {
        WebDriver::alert("hello from Rust!");
    })
    .forget();

    body.append_child(&val).unwrap();
}
