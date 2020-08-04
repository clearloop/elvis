use crate::style::Style;
use std::cmp::Ordering;

// /// ORGB
// #[derive(Clone, Copy, Debug)]
// pub struct ORGB(pub f32, pub i16, pub i16, pub i16);
//
// impl PartialEq for ORGB {
//     fn eq(&self, o: &Self) -> bool {
//         format!("{:.2}", self.0) == format!("{:.2}", o.0)
//             && self.1 == o.1
//             && self.2 == o.2
//             && self.3 == o.3
//     }
// }
//
// impl Eq for ORGB {}
//
// impl PartialOrd for ORGB {
//     fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
//         Some(self.1.cmp(&o.1))
//     }
// }
//
// impl Ord for ORGB {
//     fn cmp(&self, o: &Self) -> Ordering {
//         self.1.cmp(&o.1)
//     }
// }

/// `Color` system, accroding to material design's color system.
#[derive(Clone, Copy, Debug)]
pub enum Color {
    /// Color::Inherit => "0xFFFFFFFF"
    Inherit,
    /// Color::ORGB => "0xOORRGGBB"
    ORGB(f32, i16, i16, i16),
    /// Color::Amber => "0xFFFFC107"
    Amber,
    /// Color::AmberAccent => "0xFFFFD740"
    AmberAccent,
    /// Color::Black => "0xFF000000"
    Black,
    /// Color::Blue => "0xFF2196F3"
    Blue,
    /// Color::BlueAccent => "0xFF448AFF"
    BlueAccent,
    /// Color::BlueGrey => "0xFF607D8B"
    BlueGrey,
    /// Color::Brown => "0xFF795548"
    Brown,
    /// Color::Cyan => "0xFF00BCD4"
    Cyan,
    /// Color::CyanAccent => "0xFF18FFFF"
    CyanAccent,
    /// Color::DeepOrange => "0xFFFF5722"
    DeepOrange,
    /// Color::DeepOrangeAccent => "0xFFFF6E40"
    DeepOrangeAccent,
    /// Color::DeepPurple => "0xFF673AB7"
    DeepPurple,
    /// Color::DeepPurpleAccent => "0xFF7C4DFF"
    DeepPurpleAccent,
    /// Color::Green => "0xFF4CAF50"
    Green,
    /// Color::GreenAccent => "0xFF69F0AE"
    GreenAccent,
    /// Color::Grey => "FF9E9E9E"
    Grey,
    /// Color::Indigo => "0xFF3F51B5"
    Indigo,
    /// Color::IndigoAccent => "0xFF536DFE"
    IndigoAccent,
    /// Color::LightBlue => "0xFF03A9FA"
    LightBlue,
    /// Color::LightBlueAccent => "0xFF40C4FF"
    LightBlueAccent,
    /// Color::LightGreen => "0xFF8BC34A"
    LightGreen,
    /// Color::LightGreenAccent => "0xFFB2FF59"
    LightGreenAccent,
    /// Color::Lime => "0xFFCDDC39"
    Lime,
    /// Color::LimeAccent => "0xFFEEFF41"
    LimeAccent,
    /// Color::Orange => "0xFFFF9800"
    Orange,
    /// Color::OrangeAccent => "0xFFFFAB40"
    OrangeAccent,
    /// Color::Pink => "0xFFE91E63"
    Pink,
    /// Color::PinkAccent => "0xFFFF4081"n
    PinkAccent,
    /// Color::Purple => "0xFF9C27B0"
    Purple,
    /// Color::PurpleAccent => "0xFFE040FB"
    PurpleAccent,
    /// Color::Red => "0xFFF44336"
    Red,
    /// Color::RedAccent => "0xFFFF5252"
    RedAccent,
    /// Color::Teal => "0xFF009688"
    Teal,
    /// Color::TealAccent => "0xFF64FFDA"
    TealAccent,
    /// Color::Transparent => "0xFFFFFFFF"
    Transparent,
    /// Color::White => "0xFFFFFFFF"
    White,
    /// Color::Yellow => "0xFFFFEB3B"
    Yellow,
    /// Color::YellowAccent => "0xFFFFFF00"
    YellowAccent,
}

impl Color {
    /// deserialize hex str(char) number to decimal
    fn dec(c: &str) -> i16 {
        match &c.trim()[0..1] {
            "F" => 15,
            "E" => 14,
            "D" => 13,
            "C" => 12,
            "B" => 11,
            "A" => 10,
            _ => c.parse().unwrap_or(0),
        }
    }

    /// convert `Color` from hex, to specfic color if the hex is in `Color`
    pub fn from_hex(mut h: String) -> Color {
        h.truncate(10);
        for c in [
            Self::Amber,
            Self::AmberAccent,
            Self::Black,
            Self::Blue,
            Self::BlueAccent,
            Self::BlueGrey,
            Self::Brown,
            Self::Cyan,
            Self::CyanAccent,
            Self::DeepOrange,
            Self::DeepOrangeAccent,
            Self::DeepPurple,
            Self::DeepPurpleAccent,
            Self::Green,
            Self::GreenAccent,
            Self::Grey,
            Self::Indigo,
            Self::IndigoAccent,
            Self::LightBlue,
            Self::LightBlueAccent,
            Self::LightGreen,
            Self::LightGreenAccent,
            Self::Lime,
            Self::LimeAccent,
            Self::Orange,
            Self::OrangeAccent,
            Self::Pink,
            Self::PinkAccent,
            Self::Purple,
            Self::PurpleAccent,
            Self::Red,
            Self::RedAccent,
            Self::Teal,
            Self::TealAccent,
            Self::Transparent,
            Self::White,
            Self::Yellow,
            Self::YellowAccent,
        ]
        .iter()
        {
            if c.to_hex().eq(&h) {
                return *c;
            }
        }

        Self::from_hex_to_orgb(h)
    }

    /// Set red
    pub fn red(self, red: i16) -> Self {
        match self {
            Color::ORGB(o, _, g, b) => Color::ORGB(o, red, g, b),
            _ => Color::from_hex_to_orgb(self.to_hex()).red(red),
        }
    }

    /// Set green
    pub fn green(self, green: i16) -> Self {
        match self {
            Color::ORGB(o, r, _, b) => Color::ORGB(o, r, green, b),
            _ => Color::from_hex_to_orgb(self.to_hex()).green(green),
        }
    }

    /// Set blue
    pub fn blue(self, blue: i16) -> Self {
        match self {
            Color::ORGB(o, r, g, blue) => Color::ORGB(o, r, g, blue),
            _ => Color::from_hex_to_orgb(self.to_hex()).blue(blue),
        }
    }

    /// Set transparency
    pub fn transparent(self, transparency: f32) -> Self {
        match self {
            Color::ORGB(_, r, g, b) => Color::ORGB(transparency, r, g, b),
            _ => Color::from_hex_to_orgb(self.to_hex()).transparent(transparency),
        }
    }

    /// convert hex to `Color::ORGB`
    pub fn from_hex_to_orgb(mut h: String) -> Color {
        h.truncate(10);
        Color::ORGB(
            ((Self::dec(&h[2..3]) * 16 + Self::dec(&h[3..4])) as f32 / 255.0) as f32,
            Self::dec(&h[4..5]) * 16 + Self::dec(&h[5..6]),
            Self::dec(&h[6..7]) * 16 + Self::dec(&h[7..8]),
            Self::dec(&h[8..9]) * 16 + Self::dec(&h[9..10]),
        )
    }

    /// convert `Color` to hex string
    pub fn to_hex(&self) -> String {
        match *self {
            Color::Inherit => "0xFFFFFFFF".into(),
            Color::ORGB(o, r, g, b) => format!("{:#X}{:X}{:X}{:X}", (o * 255.0) as i32, r, g, b),
            Color::Amber => "0xFFFFC107".into(),
            Color::AmberAccent => "0xFFFFD740".into(),
            Color::Black => "0xFF000000".into(),
            Color::Blue => "0xFF2196F3".into(),
            Color::BlueAccent => "0xFF448AFF".into(),
            Color::BlueGrey => "0xFF607D8B".into(),
            Color::Brown => "0xFF795548".into(),
            Color::Cyan => "0xFF00BCD4".into(),
            Color::CyanAccent => "0xFF18FFFF".into(),
            Color::DeepOrange => "0xFFFF5722".into(),
            Color::DeepOrangeAccent => "0xFFFF6E40".into(),
            Color::DeepPurple => "0xFF673AB7".into(),
            Color::DeepPurpleAccent => "0xFF7C4DFF".into(),
            Color::Green => "0xFF4CAF50".into(),
            Color::GreenAccent => "0xFF69F0AE".into(),
            Color::Grey => "FF9E9E9E".into(),
            Color::Indigo => "0xFF3F51B5".into(),
            Color::IndigoAccent => "0xFF536DFE".into(),
            Color::LightBlue => "0xFF03A9FA".into(),
            Color::LightBlueAccent => "0xFF40C4FF".into(),
            Color::LightGreen => "0xFF8BC34A".into(),
            Color::LightGreenAccent => "0xFFB2FF59".into(),
            Color::Lime => "0xFFCDDC39".into(),
            Color::LimeAccent => "0xFFEEFF41".into(),
            Color::Orange => "0xFFFF9800".into(),
            Color::OrangeAccent => "0xFFFFAB40".into(),
            Color::Pink => "0xFFE91E63".into(),
            Color::PinkAccent => "0xFFFF4081".into(),
            Color::Purple => "0xFF9C27B0".into(),
            Color::PurpleAccent => "0xFFE040FB".into(),
            Color::Red => "0xFFF44336".into(),
            Color::RedAccent => "0xFFFF5252".into(),
            Color::Teal => "0xFF009688".into(),
            Color::TealAccent => "0xFF64FFDA".into(),
            Color::Transparent => "0xFFFFFFFF".into(),
            Color::White => "0xFFFFFFFF".into(),
            Color::Yellow => "0xFFFFEB3B".into(),
            Color::YellowAccent => "0xFFFFFF00".into(),
        }
    }

    /// convert `Color` to `Color::ORGB`
    pub fn to_orgb(&self) -> Color {
        Color::from_hex_to_orgb(self.to_hex())
    }
}

impl Eq for Color {}
impl Ord for Color {
    fn cmp(&self, o: &Self) -> Ordering {
        self.to_hex().cmp(&o.to_hex())
    }
}

impl PartialEq for Color {
    fn eq(&self, o: &Self) -> bool {
        self.to_hex().eq(&o.to_hex())
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        self.to_hex().partial_cmp(&o.to_hex())
    }
}

/// Pink is the Pig
impl Default for Color {
    fn default() -> Color {
        Color::White
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::ORGB(o, r, g, b) => format!("rgba({}, {}, {}, {:.2})", r, g, b, o),
            Color::Inherit => "inherit".into(),
            _ => {
                if let Color::ORGB(o, r, g, b) = self.to_orgb() {
                    format!("rgba({}, {}, {}, {:.1})", r, g, b, o)
                } else {
                    "rgba(255, 255, 255, 255)".into()
                }
            }
        }
    }
}

impl Into<Style> for Color {
    fn into(self) -> Style {
        Style::Color(self)
    }
}
