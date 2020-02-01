use crate::{Colors, Text, TextStyle, Unit, UnitAbbr, Widget};
use elvis::{
    Colors as ElvisColors, Text as ElvisText, TextStyle as ElvisTextStyle, Unit as ElvisUnit,
};
use std::convert::Into;

impl Into<ElvisColors> for Colors {
    fn into(self) -> ElvisColors {
        self.elvis()
    }
}

impl Into<ElvisUnit> for Unit {
    fn into(self) -> ElvisUnit {
        match self.1 {
            UnitAbbr::Ch => ElvisUnit::Ch(self.0),
            UnitAbbr::Cm => ElvisUnit::Cm(self.0),
            UnitAbbr::Dpi => ElvisUnit::Dpi(self.0),
            UnitAbbr::Dpcm => ElvisUnit::Dpcm(self.0),
            UnitAbbr::Dppx => ElvisUnit::Dppx(self.0),
            UnitAbbr::Em => ElvisUnit::Em(self.0),
            UnitAbbr::Fr => ElvisUnit::Fr(self.0),
            UnitAbbr::In => ElvisUnit::In(self.0),
            UnitAbbr::Mm => ElvisUnit::Mm(self.0),
            UnitAbbr::Pc => ElvisUnit::Pc(self.0),
            UnitAbbr::Pt => ElvisUnit::Pt(self.0),
            UnitAbbr::Px => ElvisUnit::Px(self.0),
            UnitAbbr::Q => ElvisUnit::Q(self.0),
            UnitAbbr::Rem => ElvisUnit::Rem(self.0),
            UnitAbbr::Vh => ElvisUnit::Vh(self.0),
            UnitAbbr::Vmax => ElvisUnit::Vmax(self.0),
            UnitAbbr::Vmin => ElvisUnit::Vmin(self.0),
            UnitAbbr::Vw => ElvisUnit::Vw(self.0),
            UnitAbbr::Percent => ElvisUnit::Percent(self.0),
            UnitAbbr::None => ElvisUnit::None(self.0),
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

impl Into<ElvisText> for Text {
    fn into(self) -> ElvisText {
        let et: ElvisText = self.into();
        et.into()
    }
}

impl Into<Widget> for Text {
    fn into(self) -> Widget {
        let et: ElvisText = self.into();
        Widget::new(et.into())
    }
}
