use crate::{
    style::Style,
    value::layouts::{Alignments, FlexPosition},
};

impl From<Alignments> for Vec<Style> {
    fn from(align: Alignments) -> Vec<Style> {
        match align {
            Alignments::BottomCenter => vec![
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignments::BottomLeft => vec![
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignments::BottomRight => vec![
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::End),
            ],
            Alignments::Center => vec![
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignments::CenterLeft => vec![
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignments::CenterRight => vec![
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::End),
            ],
            Alignments::TopCenter => vec![
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignments::TopLeft => vec![
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignments::TopRight => vec![
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::End),
            ],
        }
    }
}
