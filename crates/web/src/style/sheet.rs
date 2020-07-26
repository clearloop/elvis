use elvis_core::Node;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// style sheet
#[derive(Clone, Default, Debug)]
pub struct StyleSheet {
    /// Style table
    pub table: HashMap<String, String>,
}

impl<'s> StyleSheet {
    /// Share style to stylesheet tag
    pub fn shared() -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        if document.query_selector("#elvis-style-shared")?.is_none() {
            let sheet = document.create_element("style").unwrap();
            sheet.set_id("elvis-style-shared");
            sheet.set_inner_html(
                &vec![
                    "html, body {",
                    "  margin: 0;",
                    "  padding: 0;",
                    "  height: 100%;",
                    "  width: 100%;",
                    "  overflow: hidden;",
                    "}",
                ]
                .join("\n"),
            );

            document
                .query_selector("html")?
                .unwrap()
                .append_child(&sheet)?;
        }

        Ok(())
    }

    /// Serialize style into html
    pub fn ser(&self, id: String) -> Result<bool, JsValue> {
        let mut should_append_class_sheet = false;
        let mut should_append_widget_sheet = false;
        let mut should_reset_class_sheet = false;
        let mut should_reset_widget_sheet = false;
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let class_ss = document.query_selector("#classes")?.unwrap_or_else(|| {
            should_append_class_sheet = true;
            let sheet = document.create_element("style").unwrap();
            sheet.set_id("classes");
            sheet
        });

        let widget_ss = document
            .query_selector(&format!("#elvis-style-{}", &id))?
            .unwrap_or_else(|| {
                should_append_widget_sheet = true;
                let sheet = document.create_element("style").unwrap();
                sheet.set_id(&format!("elvis-style-{}", &id));
                sheet
            });

        let class_ss_inner = class_ss.inner_html();
        let widget_ss_inner = widget_ss.inner_html();

        let mut class = class_ss_inner.clone();
        let mut widget = widget_ss_inner.clone();
        self.table.iter().for_each(|(k, v)| {
            let css_text = format!("\n\n{} {{\n{}\n}}", k, v);
            if k.starts_with(".") {
                if !class.contains(&k[..]) {
                    class.push_str(&css_text);
                    if !should_reset_class_sheet {
                        should_reset_class_sheet = true;
                    }
                }
            } else if k.starts_with("#") {
                if !widget.contains(&css_text.trim()) {
                    widget.push_str(&css_text);
                    if !should_reset_widget_sheet {
                        should_reset_widget_sheet = true;
                    }
                }
            }
        });

        if should_reset_class_sheet {
            class_ss.set_inner_html(class.trim());
        }

        if widget_ss_inner.ne(&widget) {
            widget_ss.set_inner_html(widget.trim());
        }

        if should_append_class_sheet {
            document
                .query_selector("html")?
                .unwrap()
                .append_child(&class_ss)?;
        }

        if should_append_widget_sheet {
            document
                .query_selector("html")?
                .unwrap()
                .append_child(&widget_ss)?;
        }

        Ok(should_reset_class_sheet && should_reset_widget_sheet)
    }

    /// Batch style from node
    pub fn batch(&mut self, t: &mut Node) {
        if let Some(style) = t.attrs.remove("style") {
            let id = t.attrs.get("id").unwrap_or(&"".to_string()).to_string();

            // Generate id-style into table
            self.id(&id, &style);
        }

        for c in t.class.iter() {
            self.class(super::parse_class(c));
        }

        t.children
            .iter()
            .for_each(|it| self.batch(&mut it.borrow_mut()));
    }

    /// Set style to element with id
    fn id(&mut self, ti: &'s str, s: &'s str) {
        let mut style = "".to_string();
        s.split(";").collect::<Vec<&str>>().iter().for_each(|x| {
            if !x.is_empty() {
                style.push_str("  ");
                style.push_str(x.trim());
                style.push_str(";\n");
            }
        });

        let v = self.table.entry(format!("#{}", ti)).or_default();
        if v != &style {
            *v = style[..(style.len() - 1)].to_string();
        }
    }

    /// Set style to element with class
    fn class(&mut self, name: &'s str) {
        if self.table.contains_key(name) && self.table.get(name) != Some(&"".to_string()) {
            return;
        }

        let style = match name {
            "center" => vec![
                "  align-items: center;",
                "  height: 100%;",
                "  justify-content: center;",
                "  width: 100%;",
            ]
            .join("\n"),
            "col" => vec!["  flex-direction: column;"].join("\n"),
            "flex" => vec![
                "  display: flex;",
                "  height: 100%;",
                "  flex: 1;",
                "  width: 100%;",
            ]
            .join("\n"),
            "image" => vec![
                "  background-position: center;",
                "  background-repeat: no-repeat;",
                "  background-size: cover;",
                "  height: 100%;",
                "  width: 100%;",
            ]
            .join("\n"),
            "row" => vec!["  flex-direction: row;"].join("\n"),
            _ => "".to_string(),
        };

        if style == "".to_string() {
            return;
        }

        self.table.insert(format!(".{}", name), style);
    }
}
