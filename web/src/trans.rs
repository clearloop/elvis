use crate::{Colors, TextStyle, Unit, UnitAbbr};
use elvis::{Colors as ElvisColors, Serde, TextStyle as ElvisTextStyle, Unit as ElvisUnit};

pub trait Trans<T> {
    fn trans(self) -> T;
}

impl Trans<ElvisColors> for Colors {
    fn trans(self) -> ElvisColors {
        ElvisColors::de(self.ser()).unwrap_or(ElvisColors::Black)
    }
}

impl Trans<ElvisUnit> for Unit {
    fn trans(self) -> ElvisUnit {
        match self.unit() {
            UnitAbbr::Percent => ElvisUnit::de(self.ser()).unwrap_or(ElvisUnit::Percent(100.0)),
            _ => ElvisUnit::de(self.ser()).unwrap_or(ElvisUnit::None(1.0)),
        }
    }
}

impl Trans<ElvisTextStyle> for TextStyle {
    fn trans(self) -> ElvisTextStyle {
        ElvisTextStyle::de(self.ser()).unwrap_or(ElvisTextStyle::default())
    }
}
