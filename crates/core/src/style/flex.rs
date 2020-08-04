//! Flex Style
use crate::{
    style::Style,
    value::{
        layouts::{Alignment, FlexBasis, FlexDirection, FlexWrap},
        Unit,
    },
};
use elvis_core_support::Setter;

/// `Flex` Style
#[derive(Clone, Setter)]
pub struct FlexStyle {
    /// Flex align
    pub align: Alignment,
    /// Flex basis
    pub basis: FlexBasis,
    /// Flex direction
    pub direction: FlexDirection,
    /// Flex grow
    #[skip]
    pub grow: Unit,
    /// Flex order
    #[skip]
    pub order: Unit,
    /// Flex wrap
    pub wrap: FlexWrap,
}

impl Default for FlexStyle {
    fn default() -> FlexStyle {
        FlexStyle {
            align: Alignment::Center,
            basis: FlexBasis::Inherit,
            direction: FlexDirection::Row,
            grow: Unit::None(0.0),
            order: Unit::None(0.0),
            wrap: FlexWrap::Wrap,
        }
    }
}

impl FlexStyle {
    /// Set FlexGrow
    pub fn grow(mut self, grow: i64) -> FlexStyle {
        self.grow = Unit::None(grow as f64);
        self
    }

    /// Set FlexOrder
    pub fn order(mut self, order: i64) -> FlexStyle {
        self.order = Unit::None(order as f64);
        self
    }
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
            Style::Order(self.order),
            Style::FlexWrap(self.wrap),
        ]
    }
}
