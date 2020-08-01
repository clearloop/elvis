use crate::{
    style::Style,
    value::layouts::{Alignment, FlexPosition},
};

impl From<Alignment> for Vec<Style> {
    fn from(align: Alignment) -> Vec<Style> {
        match align {
            Alignment::BottomCenter => vec![
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignment::BottomLeft => vec![
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignment::BottomRight => vec![
                Style::AlignItems(FlexPosition::End),
                Style::JustifyContent(FlexPosition::End),
            ],
            Alignment::Center => vec![
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignment::CenterLeft => vec![
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignment::CenterRight => vec![
                Style::AlignItems(FlexPosition::Center),
                Style::JustifyContent(FlexPosition::End),
            ],
            Alignment::TopCenter => vec![
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::Center),
            ],
            Alignment::TopLeft => vec![
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::Start),
            ],
            Alignment::TopRight => vec![
                Style::AlignItems(FlexPosition::Start),
                Style::JustifyContent(FlexPosition::End),
            ],
        }
    }
}
