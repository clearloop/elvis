use crate::{
    widgets::{ImageSrc, TextStyle},
    Error, Serde,
};

// impl Serde<Image, String, Error> for Image {
//     fn ser(&self) -> String {
//         let t: Node = self.into();
//         t.ser()
//     }
// }

impl Serde<ImageSrc, String, Error> for ImageSrc {
    fn ser(&self) -> String {
        format!("background-image: url({})", self.0)
    }
}

// impl Serde<Text, String, Error> for Text {
//     fn ser(&self) -> String {
//         let t: Node = self.into();
//         t.ser()
//     }
// }

impl Serde<TextStyle, String, Error> for TextStyle {
    fn ser(&self) -> String {
        format!(
            "color: {}; font-weight: {}; font-style: {}; font-size: {}; font-stretch: {}; line-height: {};",
            self.color.ser(),
            if self.bold {
                "700".into()
            } else {
                self.weight.ser()
            },
            if self.italic {
                "italic"
            } else {
                "normal"
            },
            self.size.ser(),
            self.stretch.ser(),
            self.height.ser(),
        )
    }
}
