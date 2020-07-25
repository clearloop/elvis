/// `Color` system, accroding to material design's color system.
#[derive(Clone, Copy, Debug)]
pub enum Colors {
    /// Colors::Inherit => "0xFFFFFFFF"
    Inherit,
    /// Colors::Amber => "0xFFFFC107"
    ORGB(f32, i16, i16, i16),
    /// Colors::Amber => "0xFFFFC107"
    Amber,
    /// Colors::AmberAccent => "0xFFFFD740"
    AmberAccent,
    /// Colors::Black => "0xFF000000"
    Black,
    /// Colors::Blue => "0xFF2196F3"
    Blue,
    /// Colors::BlueAccent => "0xFF448AFF"
    BlueAccent,
    /// Colors::BlueGrey => "0xFF607D8B"
    BlueGrey,
    /// Colors::Brown => "0xFF795548"
    Brown,
    /// Colors::Cyan => "0xFF00BCD4"
    Cyan,
    /// Colors::CyanAccent => "0xFF18FFFF"
    CyanAccent,
    /// Colors::DeepOrange => "0xFFFF5722"
    DeepOrange,
    /// Colors::DeepOrangeAccent => "0xFFFF6E40"
    DeepOrangeAccent,
    /// Colors::DeepPurple => "0xFF673AB7"
    DeepPurple,
    /// Colors::DeepPurpleAccent => "0xFF7C4DFF"
    DeepPurpleAccent,
    /// Colors::Green => "0xFF4CAF50"
    Green,
    /// Colors::GreenAccent => "0xFF69F0AE"
    GreenAccent,
    /// Colors::Grey => "FF9E9E9E"
    Grey,
    /// Colors::Indigo => "0xFF3F51B5"
    Indigo,
    /// Colors::IndigoAccent => "0xFF536DFE"
    IndigoAccent,
    /// Colors::LightBlue => "0xFF03A9FA"
    LightBlue,
    /// Colors::LightBlueAccent => "0xFF40C4FF"
    LightBlueAccent,
    /// Colors::LightGreen => "0xFF8BC34A"
    LightGreen,
    /// Colors::LightGreenAccent => "0xFFB2FF59"
    LightGreenAccent,
    /// Colors::Lime => "0xFFCDDC39"
    Lime,
    /// Colors::LimeAccent => "0xFFEEFF41"
    LimeAccent,
    /// Colors::Orange => "0xFFFF9800"
    Orange,
    /// Colors::OrangeAccent => "0xFFFFAB40"
    OrangeAccent,
    /// Colors::Pink => "0xFFE91E63"
    Pink,
    /// Colors::PinkAccent => "0xFFFF4081"n
    PinkAccent,
    /// Colors::Purple => "0xFF9C27B0"
    Purple,
    /// Colors::PurpleAccent => "0xFFE040FB"
    PurpleAccent,
    /// Colors::Red => "0xFFF44336"
    Red,
    /// Colors::RedAccent => "0xFFFF5252"
    RedAccent,
    /// Colors::Teal => "0xFF009688"
    Teal,
    /// Colors::TealAccent => "0xFF64FFDA"
    TealAccent,
    /// Colors::Transparent => "0xFFFFFFFF"
    Transparent,
    /// Colors::White => "0xFFFFFFFF"
    White,
    /// Colors::Yellow => "0xFFFFEB3B"
    Yellow,
    /// Colors::YellowAccent => "0xFFFFFF00"
    YellowAccent,
}

impl Colors {
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

    /// convert `Colors` from hex, to specfic color if the hex is in `Colors`
    pub fn from_hex(mut h: String) -> Colors {
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

    /// convert hex to `Colors::ORGB`
    pub fn from_hex_to_orgb(mut h: String) -> Colors {
        h.truncate(10);
        Colors::ORGB(
            ((Self::dec(&h[2..3]) * 16 + Self::dec(&h[3..4])) as f32 / 255.0) as f32,
            Self::dec(&h[4..5]) * 16 + Self::dec(&h[5..6]),
            Self::dec(&h[6..7]) * 16 + Self::dec(&h[7..8]),
            Self::dec(&h[8..9]) * 16 + Self::dec(&h[9..10]),
        )
    }

    /// convert `Colors` to hex string
    pub fn to_hex(&self) -> String {
        match *self {
            Colors::Inherit => "0xFFFFFFFF".into(),
            Colors::ORGB(o, r, g, b) => format!("{:#X}{:X}{:X}{:X}", (o * 255.0) as i32, r, g, b),
            Colors::Amber => "0xFFFFC107".into(),
            Colors::AmberAccent => "0xFFFFD740".into(),
            Colors::Black => "0xFF000000".into(),
            Colors::Blue => "0xFF2196F3".into(),
            Colors::BlueAccent => "0xFF448AFF".into(),
            Colors::BlueGrey => "0xFF607D8B".into(),
            Colors::Brown => "0xFF795548".into(),
            Colors::Cyan => "0xFF00BCD4".into(),
            Colors::CyanAccent => "0xFF18FFFF".into(),
            Colors::DeepOrange => "0xFFFF5722".into(),
            Colors::DeepOrangeAccent => "0xFFFF6E40".into(),
            Colors::DeepPurple => "0xFF673AB7".into(),
            Colors::DeepPurpleAccent => "0xFF7C4DFF".into(),
            Colors::Green => "0xFF4CAF50".into(),
            Colors::GreenAccent => "0xFF69F0AE".into(),
            Colors::Grey => "FF9E9E9E".into(),
            Colors::Indigo => "0xFF3F51B5".into(),
            Colors::IndigoAccent => "0xFF536DFE".into(),
            Colors::LightBlue => "0xFF03A9FA".into(),
            Colors::LightBlueAccent => "0xFF40C4FF".into(),
            Colors::LightGreen => "0xFF8BC34A".into(),
            Colors::LightGreenAccent => "0xFFB2FF59".into(),
            Colors::Lime => "0xFFCDDC39".into(),
            Colors::LimeAccent => "0xFFEEFF41".into(),
            Colors::Orange => "0xFFFF9800".into(),
            Colors::OrangeAccent => "0xFFFFAB40".into(),
            Colors::Pink => "0xFFE91E63".into(),
            Colors::PinkAccent => "0xFFFF4081".into(),
            Colors::Purple => "0xFF9C27B0".into(),
            Colors::PurpleAccent => "0xFFE040FB".into(),
            Colors::Red => "0xFFF44336".into(),
            Colors::RedAccent => "0xFFFF5252".into(),
            Colors::Teal => "0xFF009688".into(),
            Colors::TealAccent => "0xFF64FFDA".into(),
            Colors::Transparent => "0xFFFFFFFF".into(),
            Colors::White => "0xFFFFFFFF".into(),
            Colors::Yellow => "0xFFFFEB3B".into(),
            Colors::YellowAccent => "0xFFFFFF00".into(),
        }
    }

    /// convert `Colors` to `Colors::ORGB`
    pub fn to_orgb(&self) -> Colors {
        Colors::from_hex_to_orgb(self.to_hex())
    }
}

impl Eq for Colors {}

impl PartialEq for Colors {
    fn eq(&self, o: &Self) -> bool {
        self.to_hex().eq(&o.to_hex())
    }
}

/// Pink is the Pig
impl Default for Colors {
    fn default() -> Colors {
        Colors::Pink
    }
}

impl ToString for Colors {
    fn to_string(&self) -> String {
        match self {
            Colors::ORGB(o, r, g, b) => format!("rgba({}, {}, {}, {:.1})", r, g, b, o),
            Colors::Inherit => "inherit".into(),
            _ => {
                if let Colors::ORGB(o, r, g, b) = self.to_orgb() {
                    format!("rgba({}, {}, {}, {:.1})", r, g, b, o)
                } else {
                    "rgba(255, 255, 255, 255)".into()
                }
            }
        }
    }
}
