use crate::{
    Alignments, Colors, FlexBasis, FlexDirection, GridAuto, GridFlow, GridTemplate,
    MultiColumnLineStyle,
};
use elvis::{
    widgets::{
        layouts::{
            AlignStyle as ElvisAlignStyle, ContainerStyle as ElvisContainerStyle,
            FlexStyle as ElvisFlexStyle, GridStyle as ElvisGridStyle,
            MultiColumnStyle as ElvisMultiColumnStyle, SizedBoxStyle as ElvisSizedBoxStyle,
        },
        values::Unit,
        TextStyle as ElvisTextStyle,
    },
    Node,
};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// `TextStyle` Interface
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

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
export interface ITextStyle {
  bold?: boolean;
  color?: Colors;
  italic?: boolean;
  size?: number;
  weight?: number;
  height?: number;
  stretch?: number;
}
"#;

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
            height: height,
            italic: self.italic.unwrap_or(false),
            size: Unit::Rem(self.size.unwrap_or(1.0)),
            stretch: Unit::Percent(self.stretch.unwrap_or(100.0)),
            weight: Unit::Rem(self.weight.unwrap_or(1.0)),
        }
    }
}

/// `AlignStyle` Interface
#[wasm_bindgen]
pub struct AlignStyle(ElvisAlignStyle);

#[wasm_bindgen(typescript_custom_section)]
const IALIGN_STYLE: &'static str = r#"
export interface IAlignStyle {
  align?: Alignments;
}
"#;

#[wasm_bindgen]
impl AlignStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(align: Option<Alignments>) -> AlignStyle {
        AlignStyle(ElvisAlignStyle {
            align: align.unwrap_or(Alignments::Center).into(),
        })
    }
}

impl Default for AlignStyle {
    fn default() -> AlignStyle {
        AlignStyle(ElvisAlignStyle {
            align: Alignments::Center.into(),
        })
    }
}

/// `ContainerStyle` Interface
#[wasm_bindgen]
#[derive(Default)]
pub struct ContainerStyle(ElvisContainerStyle);

#[wasm_bindgen(typescript_custom_section)]
const ICONTAINER_STYLE: &'static str = r#"
export interface IContainerStyle {
  align?: Alignments;
  color?: Colors,
  height?: number;
  margin?: number;
  padding?: number;
  width?: number;
}
"#;

#[wasm_bindgen]
impl ContainerStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        align: Option<Alignments>,
        color: Option<Colors>,
        height: Option<f64>,
        margin: Option<f64>,
        padding: Option<f64>,
        width: Option<f64>,
    ) -> ContainerStyle {
        let mut oheight = Unit::Percent(100.0);
        let mut owidth = Unit::Percent(100.0);
        if let Some(h) = height {
            oheight = Unit::Rem(h);
        }

        if let Some(w) = width {
            owidth = Unit::Rem(w);
        }

        ContainerStyle(ElvisContainerStyle {
            align: align.unwrap_or(Alignments::Center).into(),
            height: oheight,
            width: owidth,
            padding: Unit::Rem(padding.unwrap_or(0.0)),
            margin: Unit::Rem(margin.unwrap_or(0.0)),
            background_color: color.unwrap_or(Colors::inherit()).into(),
        })
    }
}

/// `SizedBox` interface
#[wasm_bindgen]
pub struct SizedBoxStyle(ElvisSizedBoxStyle);

#[wasm_bindgen(typescript_custom_section)]
const ISIZEDBOX_STYLE: &'static str = r#"
export interface ISizedBox {
  child?: Widget;
  style?: ISizedBoxStyle;
}

export interface ISizedBoxStyle {
  height?: number;
  width?: number;
}
"#;

#[wasm_bindgen]
impl SizedBoxStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(height: Option<f64>, width: Option<f64>) -> SizedBoxStyle {
        let (mut oheight, mut owidth) = (Unit::Auto, Unit::Auto);
        if let Some(h) = height {
            oheight = Unit::Rem(h);
        }
        if let Some(w) = width {
            owidth = Unit::Rem(w);
        }

        SizedBoxStyle(ElvisSizedBoxStyle {
            height: oheight,
            width: owidth,
        })
    }
}

impl Default for SizedBoxStyle {
    fn default() -> SizedBoxStyle {
        SizedBoxStyle::new(None, None)
    }
}

/// `FlexStyle` Interface
#[wasm_bindgen]
pub struct FlexStyle(ElvisFlexStyle);

#[wasm_bindgen(typescript_custom_section)]
const IFLEX_STYLE: &'static str = r#"
export interface IFlexStyle {
  align?: Alignments;
  basis?: FlexBasis;
  direction?: FlexDirection;
  grow?: number;
  order?: number;
  wrap?: boolean;
}
"#;

#[wasm_bindgen]
impl FlexStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        align: Option<Alignments>,
        basis: Option<FlexBasis>,
        direction: Option<FlexDirection>,
        grow: Option<f64>,
        order: Option<f64>,
        wrap: Option<bool>,
    ) -> FlexStyle {
        FlexStyle(ElvisFlexStyle {
            align: align.unwrap_or(Alignments::Center).into(),
            basis: basis.unwrap_or(FlexBasis::fill()).into(),
            direction: direction.unwrap_or(FlexDirection::Column).into(),
            grow: Unit::None(grow.unwrap_or(1.0)),
            order: Unit::None(order.unwrap_or(1.0)),
            wrap: wrap.unwrap_or_default(),
        })
    }
}

impl Default for FlexStyle {
    fn default() -> FlexStyle {
        FlexStyle::new(None, None, None, None, None, None)
    }
}

/// `GridStyle` Interface
#[wasm_bindgen]
pub struct GridStyle(ElvisGridStyle);

#[wasm_bindgen(typescript_custom_section)]
const IGRID_STYLE: &'static str = r#"
export interface IGridStyle {
  col?: GridAuto;
  col_gap?: number;
  flow?: GridFlow;
  row?: GridAuto;
  row_gap?: number;
  template_col?: GridTemplate;
  template_row?: GridTemplate;
}
"#;

#[wasm_bindgen]
impl GridStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        col: Option<GridAuto>,
        col_gap: Option<f64>,
        flow: Option<GridFlow>,
        row: Option<GridAuto>,
        row_gap: Option<f64>,
        template_col: Option<GridTemplate>,
        template_row: Option<GridTemplate>,
    ) -> GridStyle {
        GridStyle(ElvisGridStyle {
            col: col.unwrap_or(GridAuto::auto()).into(),
            col_gap: Unit::Rem(col_gap.unwrap_or(0.0)),
            flow: flow.unwrap_or(GridFlow::col()).into(),
            row: row.unwrap_or(GridAuto::auto()).into(),
            row_gap: Unit::Rem(row_gap.unwrap_or(0.0)),
            template_col: template_col.unwrap_or(GridTemplate::none()).into(),
            template_row: template_row.unwrap_or(GridTemplate::none()).into(),
        })
    }
}

impl Default for GridStyle {
    fn default() -> GridStyle {
        GridStyle::new(None, None, None, None, None, None, None)
    }
}

/// `GridStyle` Interface
#[wasm_bindgen]
pub struct MultiColumnStyle(ElvisMultiColumnStyle);

#[wasm_bindgen(typescript_custom_section)]
const IMULTICOLUMN_STYLE: &'static str = r#"
export interface IMultiColumnStyle {
  color?: Colors;
  count?: number;
  gap?: number;
  style?: MultiColumnLineStyle;
}
"#;

#[wasm_bindgen]
impl MultiColumnStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: Option<Colors>,
        count: Option<f64>,
        gap: Option<f64>,
        style: Option<MultiColumnLineStyle>,
    ) -> MultiColumnStyle {
        MultiColumnStyle(ElvisMultiColumnStyle {
            color: color.unwrap_or(Colors::inherit()).into(),
            count: Unit::None(count.unwrap_or(1.0)),
            gap: Unit::None(gap.unwrap_or(0.0)),
            style: style.unwrap_or(MultiColumnLineStyle::None).into(),
        })
    }
}

impl Default for MultiColumnStyle {
    fn default() -> MultiColumnStyle {
        MultiColumnStyle::new(None, None, None, None)
    }
}

into! {
    (AlignStyle, ElvisAlignStyle),
    (ContainerStyle, ElvisContainerStyle),
    (FlexStyle, ElvisFlexStyle),
    (GridStyle, ElvisGridStyle),
    (MultiColumnStyle, ElvisMultiColumnStyle),
    (SizedBoxStyle, ElvisSizedBoxStyle),
}

/// style sheet
#[derive(Clone, Default, Debug)]
pub struct StyleSheet {
    pub table: HashMap<String, String>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl<'s> StyleSheet {
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

    pub fn ser(&self, id: String) -> Result<bool, JsValue> {
        let mut should_append_class_sheet = false;
        let mut should_append_widget_sheet = false;
        let mut should_reset_class_sheet = false;
        let mut should_reset_widget_sheet = false;
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let class_ss = document
            .query_selector("#elvis-style-classes")?
            .unwrap_or_else(|| {
                should_append_class_sheet = true;
                let sheet = document.create_element("style").unwrap();
                sheet.set_id("elvis-style-classes");
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

    pub fn batch(&mut self, t: &mut Node) {
        if let Some(style) = t.attrs.remove("style") {
            let id = t.attrs.get("id").unwrap_or(&"".to_string()).to_string();
            self.id(&id, &style);
        }

        let class = t.attrs.get("class").unwrap_or(&"".to_string()).to_string();
        class.split(|x: char| x.is_whitespace()).for_each(|c| {
            let ct = c.trim();
            self.class(ct);
        });

        t.children
            .iter()
            .for_each(|it| self.batch(&mut it.borrow_mut()));
    }

    pub fn class(&mut self, name: &'s str) {
        if self.table.contains_key(name) && self.table.get(name) != Some(&"".to_string()) {
            return;
        }

        let style = match name {
            "elvis-center" => vec![
                "  align-items: center;",
                "  height: 100%;",
                "  justify-content: center;",
                "  width: 100%;",
            ]
            .join("\n"),
            "elvis-col" => vec!["  flex-direction: column;"].join("\n"),
            "elvis-flex" => vec![
                "  display: flex;",
                "  height: 100%;",
                "  flex: 1;",
                "  width: 100%;",
            ]
            .join("\n"),
            "elvis-image" => vec![
                "  background-position: center;",
                "  background-repeat: no-repeat;",
                "  background-size: cover;",
                "  height: 100%;",
                "  width: 100%;",
            ]
            .join("\n"),
            "elvis-row" => vec!["  flex-direction: row;"].join("\n"),
            _ => "".to_string(),
        };

        if style == "".to_string() {
            return;
        }

        self.table.insert(format!(".{}", name), style);
    }

    pub fn id(&mut self, ti: &'s str, s: &'s str) {
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
}
