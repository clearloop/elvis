//! unit system
use crate::Error;
use std::{cmp::Ordering, str::FromStr};

/// Follows [CSS Values 3][1] drafted in [csswg.org][2].
///
/// ## Absolute Lengths
/// | unit | name                | equivalence          |
/// |------|---------------------|----------------------|
/// | cm   | centermeters        | 1cm = 96px/2.54      |
/// | mm   | millimeters         | 1mm == 1/10th of 1cm |
/// | Q    | quarter-millimeters | 1Q = 1/40th of 1cm   |
/// | in   | inches              | 1in = 2.54cm = 96px  |
/// | pc   | picas               | 1pc = 1/6th of 1in   |
/// | pt   | points              | 1pt = 1/72th of 1in  |
/// | px   | pixels              | 1px = 1/96th of 1in  |
///
/// ## Relative Lengths
/// | unit | relative to                                                 |
/// |------|-------------------------------------------------------------|
/// | em   | font size of element                                        |
/// | ex   | x-height of element's font                                  |
/// | ch   | width of the "0" (ZERO, U+0030) glyph in the element’s font |
/// | rem  | font size of the root element                               |
/// | vw   | 1% of viewport’s width                                      |
/// | vh   | 1% of viewport’s height                                     |
/// | vmin | 1% of viewport’s smaller dimension                          |
/// | vmax | 1% of viewport’s larger dimension                           |
///
/// ## Others
/// | unit | represents                                                                      |
/// |------|---------------------------------------------------------------------------------|
/// | dpi  | Dots per inch                                                                   |
/// | dpcm | Dots per centmeter                                                              |
/// | dppx | Dots per px unit                                                                |
/// | fr   | This unit represents one fraction of the available space in the grid container. |
///
/// [1]: https://drafts.csswg.org/css-values-3
/// [2]: https://drafts.csswg.org
#[derive(Clone, Copy, Debug)]
pub enum Unit {
    /// auto size
    Auto,
    /// ch
    Ch(f64),
    /// cm
    Cm(f64),
    /// dpi
    Dpi(f64),
    /// dpcm
    Dpcm(f64),
    /// dppx
    Dppx(f64),
    /// em
    Em(f64),
    /// fr
    Fr(f64),
    /// in
    In(f64),
    /// mm
    Mm(f64),
    /// pc
    Pc(f64),
    /// pt
    Pt(f64),
    /// px
    Px(f64),
    /// q
    Q(f64),
    /// rem
    Rem(f64),
    /// vh
    Vh(f64),
    /// vmax
    Vmax(f64),
    /// vmin
    Vmin(f64),
    /// vw
    Vw(f64),
    /// v%
    Percent(f64),
    /// no unit
    None(f64),
}

impl Eq for Unit {}

impl PartialEq for Unit {
    fn eq(&self, o: &Self) -> bool {
        self.to_string().eq(&o.to_string())
    }
}

impl Ord for Unit {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        None
    }
}

impl Default for Unit {
    fn default() -> Unit {
        Unit::Auto
    }
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Unit, Self::Err> {
        let t = s.trim();
        let u = t
            .find(|c: char| !c.is_numeric() && !c.eq(&'.'))
            .unwrap_or(0);

        let v: f64 = t[..u]
            .trim()
            .parse()
            .unwrap_or_else(|_| t[u..].trim().parse().unwrap_or(1.0));

        Ok(match t[u..].trim().to_ascii_lowercase().as_str() {
            "auto" | "inherit" => Unit::Auto,
            "ch" => Unit::Ch(v),
            "cm" => Unit::Cm(v),
            "dpcm" => Unit::Dpcm(v),
            "dpi" => Unit::Dpi(v),
            "dppx" => Unit::Dppx(v),
            "em" => Unit::Em(v),
            "fr" => Unit::Fr(v),
            "in" => Unit::In(v),
            "mm" => Unit::Mm(v),
            "pc" => Unit::Pc(v),
            "pt" => Unit::Pt(v),
            "px" => Unit::Px(v),
            "q" => Unit::Q(v),
            "rem" => Unit::Rem(v),
            "vh" => Unit::Vh(v),
            "vmax" => Unit::Vmax(v),
            "vmin" => Unit::Vmin(v),
            "vw" => Unit::Vw(v),
            "%" => Unit::Percent(t[..u].parse().unwrap_or(100.0)),
            _ => Unit::None(v),
        })
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Auto => "auto".into(),
            Unit::Ch(n) => format!("{:.1}ch", n),
            Unit::Cm(n) => format!("{:.1}cm", n),
            Unit::Dpcm(n) => format!("{:.1}dpcm", n),
            Unit::Dpi(n) => format!("{:.1}dpi", n),
            Unit::Dppx(n) => format!("{:.1}dppx", n),
            Unit::Em(n) => format!("{:.1}em", n),
            Unit::Fr(n) => format!("{:.1}fr", n),
            Unit::In(n) => format!("{:.1}in", n),
            Unit::Mm(n) => format!("{:.1}mm", n),
            Unit::Pc(n) => format!("{:.1}pc", n),
            Unit::Pt(n) => format!("{:.1}pt", n),
            Unit::Px(n) => format!("{:.1}px", n),
            Unit::Q(n) => format!("{:.1}Q", n),
            Unit::Rem(n) => format!("{:.1}rem", n),
            Unit::Vh(n) => format!("{:.1}vh", n),
            Unit::Vmax(n) => format!("{:.1}vmax", n),
            Unit::Vmin(n) => format!("{:.1}vmin", n),
            Unit::Vw(n) => format!("{:.1}vw", n),
            Unit::Percent(n) => format!("{:.1}%", n),
            Unit::None(n) => format!("{:.0}", n),
        }
    }
}

/// Vec Unit
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VecUnit(pub Vec<Unit>);

impl Default for VecUnit {
    fn default() -> VecUnit {
        VecUnit(vec![Unit::None(0.0)])
    }
}

impl ToString for VecUnit {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl From<Vec<Unit>> for VecUnit {
    fn from(vu: Vec<Unit>) -> VecUnit {
        VecUnit(vu)
    }
}
