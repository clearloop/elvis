//! parser in #[cfg(feature = "web")]
use crate::err::Error;
use crate::{Node, Serde};

impl<'t> Serde<Node, String, Error> for Node {
    fn ser(&self) -> String {
        let mut html = "".to_string();
        let mut attrs = " ".to_string();
        let mut children = "".to_string();

        // plain text
        if self.tag == "plain" {
            html.push_str(&self.attrs.get("text").unwrap_or(&"".into()));
        } else {
            for (k, v) in self.attrs.iter() {
                attrs.push_str(&format!("{}=\"{}\" ", k, v));
            }

            for i in &self.children {
                children.push_str(&i.borrow().to_owned().ser());
            }

            if attrs.trim().is_empty() {
                attrs.drain(..);
            }

            html.push_str(&format!(
                "<{}{}>{}</{}>",
                &self.tag,
                attrs.trim_end(),
                children,
                &self.tag,
            ));
        }

        html
    }
}
