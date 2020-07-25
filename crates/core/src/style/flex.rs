//! Flex Style
use crate::value::{
    layouts::{Alignments, FlexBasis, FlexDirection},
    Unit,
};

/// `Align` style
pub struct AlignStyle {
    /// Align value
    pub align: Alignments,
}

impl ToString for AlignStyle {
    fn to_string(&self) -> String {
        self.align.to_string()
    }
}

/// `Flex` Style
pub struct FlexStyle {
    /// Flex align
    pub align: Alignments,
    /// Flex basis
    pub basis: FlexBasis,
    /// Flex direction
    pub direction: FlexDirection,
    /// Flex grow
    pub grow: Unit,
    /// Flex order
    pub order: Unit,
    /// Flex wrap
    pub wrap: bool,
}

impl ToString for FlexStyle {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        s += &self.align.to_string();
        s += &self.basis.to_string();
        s += &self.direction.to_string();
        s += &format!("flex-grow: {};", self.grow.to_string());
        s += &format!("flex-order: {};", self.order.to_string());
        s += &format!("wrap: {};", if self.wrap { "wrap" } else { "no-wrap" });
        s
    }
}
