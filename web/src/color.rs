use elvis::Colors as ElvisColors;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Colors(ElvisColors);

impl Default for Colors {
    fn default() -> Colors {
        Colors(ElvisColors::Black)
    }
}

impl std::convert::Into<ElvisColors> for Colors {
    fn into(self) -> ElvisColors {
        self.0
    }
}

#[wasm_bindgen]
impl Colors {
    #[wasm_bindgen(js_name = "RGBA")]
    pub fn rgba(r: i16, g: i16, b: i16, a: f32) -> Colors {
        Colors(ElvisColors::ORGB(a, r, g, b))
    }

    #[wasm_bindgen(js_name = "Amber")]
    pub fn amber() -> Colors {
        Colors(ElvisColors::Amber)
    }

    #[wasm_bindgen(js_name = "AmberAccent")]
    pub fn amber_accent() -> Colors {
        Colors(ElvisColors::AmberAccent)
    }

    #[wasm_bindgen(js_name = "Black")]
    pub fn black() -> Colors {
        Colors(ElvisColors::Black)
    }

    #[wasm_bindgen(js_name = "Blue")]
    pub fn blue() -> Colors {
        Colors(ElvisColors::Blue)
    }

    #[wasm_bindgen(js_name = "BlueAccent")]
    pub fn blue_accent() -> Colors {
        Colors(ElvisColors::BlueAccent)
    }

    #[wasm_bindgen(js_name = "BlueGrey")]
    pub fn blue_grey() -> Colors {
        Colors(ElvisColors::BlueGrey)
    }

    #[wasm_bindgen(js_name = "Brown")]
    pub fn brown() -> Colors {
        Colors(ElvisColors::Brown)
    }
    #[wasm_bindgen(js_name = "Cyan")]
    pub fn cyan() -> Colors {
        Colors(ElvisColors::Cyan)
    }

    #[wasm_bindgen(js_name = "CyanAccent")]
    pub fn cyan_accent() -> Colors {
        Colors(ElvisColors::CyanAccent)
    }

    #[wasm_bindgen(js_name = "DeepOrange")]
    pub fn deep_orange() -> Colors {
        Colors(ElvisColors::DeepOrange)
    }

    #[wasm_bindgen(js_name = "DeepOrangeAccent")]
    pub fn deep_orange_accent() -> Colors {
        Colors(ElvisColors::DeepOrangeAccent)
    }

    #[wasm_bindgen(js_name = "DeepPurple")]
    pub fn deep_purple() -> Colors {
        Colors(ElvisColors::DeepPurple)
    }

    #[wasm_bindgen(js_name = "DeepPurpleAccent")]
    pub fn deep_purple_accent() -> Colors {
        Colors(ElvisColors::DeepPurpleAccent)
    }

    #[wasm_bindgen(js_name = "Green")]
    pub fn green() -> Colors {
        Colors(ElvisColors::Green)
    }

    #[wasm_bindgen(js_name = "GreenAccent")]
    pub fn green_accent() -> Colors {
        Colors(ElvisColors::GreenAccent)
    }

    #[wasm_bindgen(js_name = "Grey")]
    pub fn grey() -> Colors {
        Colors(ElvisColors::Grey)
    }

    #[wasm_bindgen(js_name = "Indigo")]
    pub fn indigo() -> Colors {
        Colors(ElvisColors::Indigo)
    }

    #[wasm_bindgen(js_name = "IndigoAccent")]
    pub fn indigo_accent() -> Colors {
        Colors(ElvisColors::IndigoAccent)
    }

    #[wasm_bindgen(js_name = "Inherit")]
    pub fn inherit() -> Colors {
        Colors(ElvisColors::Inherit)
    }

    #[wasm_bindgen(js_name = "LightBlue")]
    pub fn light_blue() -> Colors {
        Colors(ElvisColors::LightBlue)
    }

    #[wasm_bindgen(js_name = "LightBlueAccent")]
    pub fn light_blue_accent() -> Colors {
        Colors(ElvisColors::LightBlueAccent)
    }

    #[wasm_bindgen(js_name = "LightGreen")]
    pub fn light_green() -> Colors {
        Colors(ElvisColors::LightGreen)
    }

    #[wasm_bindgen(js_name = "LightGreenAccent")]
    pub fn light_green_accent() -> Colors {
        Colors(ElvisColors::LightGreenAccent)
    }

    #[wasm_bindgen(js_name = "Lime")]
    pub fn lime() -> Colors {
        Colors(ElvisColors::Lime)
    }

    #[wasm_bindgen(js_name = "LimeAccent")]
    pub fn lime_accent() -> Colors {
        Colors(ElvisColors::LimeAccent)
    }

    #[wasm_bindgen(js_name = "Orange")]
    pub fn orange() -> Colors {
        Colors(ElvisColors::Orange)
    }

    #[wasm_bindgen(js_name = "OrangeAccent")]
    pub fn orange_accent() -> Colors {
        Colors(ElvisColors::OrangeAccent)
    }

    #[wasm_bindgen(js_name = "Pink")]
    pub fn pink() -> Colors {
        Colors(ElvisColors::Pink)
    }

    #[wasm_bindgen(js_name = "PinkAccent")]
    pub fn pink_accent() -> Colors {
        Colors(ElvisColors::PinkAccent)
    }

    #[wasm_bindgen(js_name = "Purple")]
    pub fn purple() -> Colors {
        Colors(ElvisColors::Purple)
    }

    #[wasm_bindgen(js_name = "PurpleAccent")]
    pub fn purple_accent() -> Colors {
        Colors(ElvisColors::PurpleAccent)
    }

    #[wasm_bindgen(js_name = "Red")]
    pub fn red() -> Colors {
        Colors(ElvisColors::Red)
    }

    #[wasm_bindgen(js_name = "RedAccent")]
    pub fn red_accent() -> Colors {
        Colors(ElvisColors::RedAccent)
    }

    #[wasm_bindgen(js_name = "Teal")]
    pub fn teal() -> Colors {
        Colors(ElvisColors::Teal)
    }

    #[wasm_bindgen(js_name = "TealAccent")]
    pub fn teal_accent() -> Colors {
        Colors(ElvisColors::TealAccent)
    }

    #[wasm_bindgen(js_name = "Transparent")]
    pub fn transparent() -> Colors {
        Colors(ElvisColors::Transparent)
    }

    #[wasm_bindgen(js_name = "White")]
    pub fn white() -> Colors {
        Colors(ElvisColors::White)
    }

    #[wasm_bindgen(js_name = "Yellow")]
    pub fn yellow() -> Colors {
        Colors(ElvisColors::Yellow)
    }

    #[wasm_bindgen(js_name = "YellowAccent")]
    pub fn yellow_accent() -> Colors {
        Colors(ElvisColors::YellowAccent)
    }
}
