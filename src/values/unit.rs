/// Follows [CSS Values 3][1] drafted in csswg.org.
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
pub enum Unit {
    Ch(f64),
    Cm(f64),
    Dpi(f64),
    Dpcm(f64),
    Dppx(f64),
    Em(f64),
    Fr(f64),
    In(f64),
    Mm(f64),
    Pc(f64),
    Pt(f64),
    Px(f64),
    Q(f64),
    Rem(f64),
    Vh(f64),
    Vmax(f64),
    Vmin(f64),
    Vw(f64),
}

impl std::default::Default for Unit {
    fn default() -> Unit {
        Unit::Em(1.0)
    }
}
