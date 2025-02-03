use iced::widget::{
    button, column, container, row, text, text_input,
    horizontal_space,
};
use iced::{Element, Length, Color};
use crate::data_types::{EntityId, ValidationError};
use std::iter::empty;
use crate::HotKey;
use super::ChoiceGroup;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    Save,
    Cancel,
}

pub struct EditState {
    pub name: String,
    pub id: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(group: &ChoiceGroup) -> Self {
        Self {
            name: group.name.clone(),
            id: group.id.to_string(),
            validation_error: None,
        }
    }
}

impl EditState {
    pub fn validate(&self, other_groups: &[&ChoiceGroup]) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Choice group name cannot be empty".to_string()
            ));
        }

        let id: EntityId = self.id.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid ID format".to_string())
        })?;

        if !(1..=9999).contains(&id) {
            return Err(ValidationError::InvalidId(
                "Choice Group ID must be between 1 and 9999".to_string()
            ));
        }

        for other in other_groups {
            if id == other.id {
                return Err(ValidationError::DuplicateId(
                    format!("Choice Group with ID {} already exists", id)
                ));
            }
        }

        Ok(())
    }
}


pub fn view<'a>(
    group: &'a ChoiceGroup,
    state: EditState,
    other_groups: &'a [&'a ChoiceGroup],
) -> Element<'a, Message> {

    let name = state.name.clone();
    let id = state.id.clone();
    let error_message = state.validation_error.clone();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Choice Group Name", &name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-9999)", &id)
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
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

    let mut col = column![content, controls].spacing(20);

    if let Some(error) = error_message {
        col = col.push(
            container(
                text(error)
                    .style(text::danger)
            )
            .padding(10)
        );
    }

    container(col)
        .padding(20)
        .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Cancel),
        _ => crate::Action::none(),
    }
}