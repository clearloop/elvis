use crate::{
    Alignments, Colors, FlexBasis, FlexDirection, GridAutoRows, GridTemplate, MultiColumnLineStyle,
};
use elvis::{
    AlignStyle as ElvisAlignStyle, ContainerStyle as ElvisContainerStyle,
    FlexStyle as ElvisFlexStyle, GridStyle as ElvisGridStyle,
    MultiColumnStyle as ElvisMultiColumnStyle, SizedBoxStyle as ElvisSizedBoxStyle,
    TextStyle as ElvisTextStyle, Tree, Unit,
};
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
            italic: self.italic.unwrap_or(false),
            size: Unit::Rem(self.size.unwrap_or(1.0)),
            weight: Unit::Rem(self.weight.unwrap_or(1.0)),
            height: height,
            stretch: Unit::Percent(self.stretch.unwrap_or(100.0)),
        }
    }
}

/// AlignStyle Interface
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

/// ContainerStyle Interface
#[wasm_bindgen]
#[derive(Default)]
pub struct ContainerStyle(ElvisContainerStyle);

#[wasm_bindgen(typescript_custom_section)]
const ICONTAINER_STYLE: &'static str = r#"
export interface IContainerStyle {
  align?: Alignments;
  height?: number;
  width?: number;
  padding?: number;
  margin?: number;
  color?: Colors,
}
"#;

#[wasm_bindgen]
impl ContainerStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        align: Option<Alignments>,
        height: Option<f64>,
        width: Option<f64>,
        padding: Option<f64>,
        margin: Option<f64>,
        color: Option<Colors>,
    ) -> ContainerStyle {
        ContainerStyle(ElvisContainerStyle {
            align: align.unwrap_or(Alignments::Center).into(),
            height: Unit::Rem(height.unwrap_or(1.0)),
            width: Unit::Rem(width.unwrap_or(1.0)),
            padding: Unit::Rem(padding.unwrap_or(0.0)),
            margin: Unit::Rem(margin.unwrap_or(0.0)),
            background_color: color.unwrap_or(Colors::inherit()).into(),
        })
    }
}

/// SizedBox Interface
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
        SizedBoxStyle(ElvisSizedBoxStyle {
            height: Unit::Rem(height.unwrap_or(1.0)),
            width: Unit::Rem(width.unwrap_or(1.0)),
        })
    }
}

impl Default for SizedBoxStyle {
    fn default() -> SizedBoxStyle {
        SizedBoxStyle::new(None, None)
    }
}

/// FlexStyle Interface
#[wasm_bindgen]
pub struct FlexStyle(ElvisFlexStyle);

#[wasm_bindgen(typescript_custom_section)]
const IFLEX_STYLE: &'static str = r#"
export interface IFlexStyle {
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
        basis: Option<FlexBasis>,
        direction: Option<FlexDirection>,
        grow: Option<f64>,
        order: Option<f64>,
        wrap: Option<bool>,
    ) -> FlexStyle {
        FlexStyle(ElvisFlexStyle {
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
        FlexStyle::new(None, None, None, None, None)
    }
}

/// GridStyle Interface
#[wasm_bindgen]
pub struct GridStyle(ElvisGridStyle);

#[wasm_bindgen(typescript_custom_section)]
const IGRID_STYLE: &'static str = r#"
export interface IGridStyle {
  col?: number;
  row?: number;
  gap?: number;
  template_col?: GridTemplate;
  template_row?: GridTemplate;
  auto_rows?: GridAutoRows;
}
"#;

#[wasm_bindgen]
impl GridStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        col: Option<f64>,
        row: Option<f64>,
        gap: Option<f64>,
        template_col: Option<GridTemplate>,
        template_row: Option<GridTemplate>,
        auto_rows: Option<GridAutoRows>,
    ) -> GridStyle {
        GridStyle(ElvisGridStyle {
            col: Unit::Fr(col.unwrap_or(1.0)),
            row: Unit::Fr(row.unwrap_or(1.0)),
            gap: Unit::Fr(gap.unwrap_or(1.0)),
            template_col: template_col.unwrap_or(GridTemplate::auto()).into(),
            template_row: template_row.unwrap_or(GridTemplate::auto()).into(),
            auto_rows: auto_rows.unwrap_or(GridAutoRows::auto()).into(),
        })
    }
}

impl Default for GridStyle {
    fn default() -> GridStyle {
        GridStyle::new(None, None, None, None, None, None)
    }
}

/// GridStyle Interface
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
            count: Unit::None(count.unwrap_or(0.0)),
            gap: Unit::Fr(gap.unwrap_or(1.0)),
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
            if !x.is_empty() {
                style.push_str("  ");
                style.push_str(x.trim());
                style.push_str(";\n");
            }
        });

        self.0.push_str(&format!(
            "{}{}",
            &format!("\n\n#{} ", ti),
            vec!["{\n", &style[..(style.len() - 1)], "\n}"].join("")
        ));
    }
}
