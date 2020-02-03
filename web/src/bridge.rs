use crate::{Colors, TextStyle, Unit, Units};
use elvis::{Colors as ElvisColors, TextStyle as ElvisTextStyle, Unit as ElvisUnit};
use std::convert::Into;

impl Into<ElvisColors> for Colors {
    fn into(self) -> ElvisColors {
        self.elvis()
    }
}

impl Into<ElvisUnit> for Unit {
    fn into(self) -> ElvisUnit {
        match self.1 {
            Units::Ch => ElvisUnit::Ch(self.0),
            Units::Cm => ElvisUnit::Cm(self.0),
            Units::Dpi => ElvisUnit::Dpi(self.0),
            Units::Dpcm => ElvisUnit::Dpcm(self.0),
            Units::Dppx => ElvisUnit::Dppx(self.0),
            Units::Em => ElvisUnit::Em(self.0),
            Units::Fr => ElvisUnit::Fr(self.0),
            Units::In => ElvisUnit::In(self.0),
            Units::Mm => ElvisUnit::Mm(self.0),
            Units::Pc => ElvisUnit::Pc(self.0),
            Units::Pt => ElvisUnit::Pt(self.0),
            Units::Px => ElvisUnit::Px(self.0),
            Units::Q => ElvisUnit::Q(self.0),
            Units::Rem => ElvisUnit::Rem(self.0),
            Units::Vh => ElvisUnit::Vh(self.0),
            Units::Vmax => ElvisUnit::Vmax(self.0),
            Units::Vmin => ElvisUnit::Vmin(self.0),
            Units::Vw => ElvisUnit::Vw(self.0),
            Units::Percent => ElvisUnit::Percent(self.0),
            Units::None => ElvisUnit::None(self.0),
        }
    }
}

impl Into<ElvisTextStyle> for TextStyle {
    fn into(self) -> ElvisTextStyle {
        ElvisTextStyle::new(
            self.bold,
            self.color.into(),
            self.italic,
            self.size.into(),
            self.weight.into(),
            self.height.into(),
            self.stretch.into(),
        )
    }
}
