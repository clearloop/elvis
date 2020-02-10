use crate::Colors;
use elvis::{TextStyle as ElvisTextStyle, Tree, Unit};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

/// TextStyle Interface
#[wasm_bindgen]
#[derive(Default)]
pub struct TextStyle {
    pub bold: Option<bool>,
    pub color: Option<Colors>,
    pub italic: Option<bool>,
    pub size: Option<f64>,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub stretch: Option<f64>,
}

#[wasm_bindgen]
impl TextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bold: Option<bool>,
        color: Option<Colors>,
        italic: Option<bool>,
        size: Option<f64>,
        weight: Option<f64>,
        height: Option<f64>,
        stretch: Option<f64>,
    ) -> TextStyle {
        TextStyle {
            bold,
            color,
            italic,
            size,
            weight,
            height,
            stretch,
        }
    }
}

impl Into<ElvisTextStyle> for TextStyle {
    fn into(self) -> ElvisTextStyle {
        let mut height = Unit::Auto;
        if let Some(u) = self.height {
            height = Unit::Rem(u);
        }

        ElvisTextStyle {
            bold: self.bold.unwrap_or(false),
            color: self.color.unwrap_or_default().into(),
            italic: self.italic.unwrap_or(false),
            size: Unit::Rem(self.size.unwrap_or(1.0)),
            weight: Unit::Rem(self.weight.unwrap_or(1.0)),
            height: height,
            stretch: Unit::Percent(self.stretch.unwrap_or(100.0)),
        }
    }
}

/// style sheet
pub struct StyleSheet(pub String);

impl<'s> StyleSheet {
    pub fn batch(t: &'s mut Tree, hs: &mut HashSet<String>) -> String {
        let mut ss = StyleSheet("".into());
        if let Some(style) = t.attrs.remove("style") {
            let id = t.attrs.get("id").unwrap_or(&"".to_string()).to_string();
            ss.id(&id, &style);
        }

        let class = t.attrs.get("class").unwrap_or(&"".to_string()).to_string();
        class.split(|x: char| x.is_whitespace()).for_each(|c| {
            let ct = c.trim();
            if !ct.is_empty() {
                if !hs.contains(ct) {
                    hs.insert(ct.into());
                    ss.class(ct);
                }
            }
        });

        t.children
            .iter()
            .for_each(|it| ss.0.push_str(&StyleSheet::batch(&mut it.borrow_mut(), hs)));
        ss.0
    }

    pub fn class(&mut self, name: &'s str) {
        match name {
            "elvis-image" => self.0.push_str(
                &vec![
                    "\n\n.elvis-image {",
                    "  background-position: center;",
                    "  background-repeat: no-repeat;",
                    "  background-size: cover;",
                    "  height: 100%;",
                    "  width: 100%;",
                    "}",
                ]
                .join("\n"),
            ),
            _ => {}
        }
    }

    pub fn id(&mut self, ti: &'s str, s: &'s str) {
        let mut style = "".to_string();
        s.split(";").collect::<Vec<&str>>().iter().for_each(|x| {
            style.push_str("  ");
            style.push_str(x.trim());
            style.push_str(";\n");
        });

        self.0.push_str(&format!(
            "{}{}",
            &format!("\n\n#{} ", ti),
            vec!["{\n", &style[..(style.len() - 5)], "\n}"].join("")
        ));
    }
}
