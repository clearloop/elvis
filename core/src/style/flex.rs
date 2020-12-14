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
#[derive(Clone, Default, Setter)]
pub struct FlexStyle {
    /// Flex align
    pub align: Option<Alignment>,
    /// Flex basis
    pub basis: Option<FlexBasis>,
    /// Flex direction
    pub direction: Option<FlexDirection>,
    /// Flex grow
    #[skip]
    pub grow: Option<Unit>,
    /// Flex order
    #[skip]
    pub order: Option<Unit>,
    /// Flex wrap
    pub wrap: Option<FlexWrap>,
}

impl FlexStyle {
    /// Set FlexGrow
    pub fn grow(mut self, grow: i64) -> FlexStyle {
        self.grow = Some(Unit::None(grow as f64));
        self
    }

    /// Set FlexOrder
    pub fn order(mut self, order: i64) -> FlexStyle {
        self.order = Some(Unit::None(order as f64));
        self
    }
}

impl Into<Vec<Style>> for FlexStyle {
    fn into(self) -> Vec<Style> {
        let mut styles: Vec<Style> = vec![];

        // let align_style: Vec<Style> = self.align.into();
        // vec![
        //     align_style[0].clone(),
        //     align_style[1].clone(),
        //     self.basis.into(),
        //     self.direction.into(),
        //     Style::FlexGrow(self.grow),
        //     Style::Order(self.order),
        //     Style::FlexWrap(self.wrap),
        // ]
        if let Some(v) = self.align {
            styles.append(&mut v.into());
        }

        styles
    }
}
