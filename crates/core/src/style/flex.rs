//! Flex Style
use crate::{
    style::Style,
    value::{
        layouts::{Alignments, FlexBasis, FlexDirection},
        Unit,
    },
};
use elvis_core_support::Setter;

/// `Flex` Style
#[derive(Clone, Default, Setter)]
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

impl Into<Vec<Style>> for FlexStyle {
    fn into(self) -> Vec<Style> {
        let align_style: Vec<Style> = self.align.into();
        vec![
            align_style[0].clone(),
            align_style[1].clone(),
            self.basis.into(),
            self.direction.into(),
            Style::FlexGrow(self.grow),
            Style::FlexOrder(self.order),
            Style::Wrap(self.wrap),
        ]
    }
}
