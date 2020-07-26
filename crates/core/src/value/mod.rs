//! Elvis values
mod color;
pub mod layouts;
mod unit;

use crate::style::Style;
use layouts::{Alignments, FlexPosition};

impl Into<[Style; 2]> for Alignments {
    fn into(self) -> [Style; 2] {
        match self {
            Alignments::BottomCenter => [
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignments::BottomLeft => [
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignments::BottomRight => [
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::End),
            ],
            Alignments::Center => [
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignments::CenterLeft => [
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignments::CenterRight => [
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::End),
            ],
            Alignments::TopCenter => [
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignments::TopLeft => [
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignments::TopRight => [
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::End),
            ],
        }
    }
}

pub use {color::Colors, unit::Unit};
