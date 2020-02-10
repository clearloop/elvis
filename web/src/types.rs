use wasm_bindgen::prelude::*;

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
