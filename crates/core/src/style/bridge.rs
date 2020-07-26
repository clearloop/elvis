use crate::{
    style::Style,
    value::layouts::{Alignments, FlexPosition},
};

impl From<Alignments> for [Style; 2] {
    fn from(align: Alignments) -> [Style; 2] {
        match align {
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
