//! Flex Style
use crate::{
    style::Style,
    value::{
        layouts::{Alignments, FlexBasis, FlexDirection},
        Unit,
    },
};

/// `Flex` Style
#[derive(Clone)]
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

impl Into<[Style; 7]> for FlexStyle {
    fn into(self) -> [Style; 7] {
        let [items, content]: [Style; 2] = self.align.into();
        [
            items,
            content,
            self.basis.into(),
            self.direction.into(),
            Style::FlexGrow(self.grow),
            Style::FlexOrder(self.order),
            Style::Wrap(self.wrap),
        ]
    }
}
