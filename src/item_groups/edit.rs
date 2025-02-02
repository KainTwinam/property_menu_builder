use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length};
use crate::data_types::{EntityId, ValidationError};
use rangemap::RangeInclusiveSet;
use std::iter::empty;
use crate::HotKey;
use super::ItemGroup;

#[derive(Debug, Clone)]
pub struct EditState {
    pub name: String,
    pub range_start: String,
    pub range_end: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(item_group: &ItemGroup) -> Self {
        Self {
            name: item_group.name.clone(),
            range_start: item_group.id_range.start.to_string(),
            range_end: item_group.id_range.end.to_string(),
            validation_error: None,
        }
    }
}

impl EditState {
    pub fn validate(&self, other_groups: &[&ItemGroup]) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Item group name cannot be empty".to_string()
            ));
        }

        let start: EntityId = self.range_start.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid range start value".to_string())
        })?;

        let end: EntityId = self.range_end.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid range end value".to_string())
        })?;

        if start >= end {
            return Err(ValidationError::InvalidRange(
                "Start ID must be less than end ID".to_string()
            ));
        }

        // Check for range overlap with other groups
        for other in other_groups {
            if ranges_overlap(&(start..=end), &(other.id_range.start..=other.id_range.end)) {
                return Err(ValidationError::RangeOverlap(
                    format!("Range overlaps with group '{}'", other.name)
                ));
            }
        }

        Ok(())
    }
}

fn ranges_overlap<T: Ord>(range1: &std::ops::RangeInclusive<T>, range2: &std::ops::RangeInclusive<T>) -> bool {
    range1.start() <= range2.end() && range2.start() <= range1.end()
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateRangeStart(String),
    UpdateRangeEnd(String),
    ValidateRange,
    Save,
    Cancel,
}

pub fn view<'a>(
    item_group: &'a ItemGroup,
    state: &'a EditState,
    other_groups: &'a [&'a ItemGroup]
) -> Element<'a, Message> {
    let content = container(
        column![
            // Name input
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Group Name", &state.name)
                    .on_input(Message::UpdateName)
                    .on_submit(Message::ValidateRange)
                    .padding(5)
            ],
            // Range inputs
            row![
                text("ID Range Start").width(Length::Fixed(150.0)),
                text_input("Start ID", &state.range_start)
                    .on_input(Message::UpdateRangeStart)
                    .on_submit(Message::ValidateRange)
                    .padding(5)
            ],
            row![
                text("ID Range End").width(Length::Fixed(150.0)),
                text_input("End ID", &state.range_end)
                    .on_input(Message::UpdateRangeEnd)
                    .on_submit(Message::ValidateRange)
                    .padding(5)
            ],
            // Validation error message (if any)
            if let Some(error) = &state.validation_error {
                container(
                    text(error)
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            } else {
                container(
                    text("")
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            }
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    let controls = row![
        horizontal_space(),
        button("Cancel")
            .on_press(Message::Cancel)
            .style(button::danger),
        button("Save")
            .on_press(Message::Save)
            .style(button::success),
    ]
    .spacing(10)
    .padding(20);

    container(
        column![
            content,
            controls,
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}