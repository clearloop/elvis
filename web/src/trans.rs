use crate::{Colors, Unit, UnitAbbr};
use elvis::{Colors as ElvisColors, Unit as ElvisUnit};

/// trans elvis-web to elvis
pub trait Trans<T> {
    fn trans(self) -> T;
}

impl Trans<ElvisColors> for Colors {
    fn trans(self) -> ElvisColors {
        self.elvis()
    }
}

impl Trans<ElvisUnit> for Unit {
    fn trans(self) -> ElvisUnit {
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
