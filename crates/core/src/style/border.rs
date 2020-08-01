use crate::value::{BorderStyle, Colors, Unit};

/// Border Style
pub struct Border {
    color: Colors,
    radius: Unit,
    width: Unit,
    style: Vec<BorderStyle>,
}
