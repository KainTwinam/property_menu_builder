pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
    IdRange,
};
use crate::Action;
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ChoiceGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidId(String),
    DuplicateId(String),
    EmptyName(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceGroup {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for ChoiceGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ChoiceGroup {
    fn validate(&self, other_groups: &[&ChoiceGroup]) -> Result<(), ValidationError> {
        // Validate ID range (1-9999 based on your screenshot)
        if !(1..=9999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Choice Group ID must be between 1 and 9999".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_groups {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Choice Group with ID {} already exists", self.id)
                ));
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Choice Group name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    group: &mut ChoiceGroup,
    message: Message,
    state: &mut edit::EditState,
    other_groups: &[&ChoiceGroup],
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                state.name = name;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                state.id = id;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::Save => {
                match state.validate(other_groups) {
                    Ok(_) => Action::operation(Operation::Save(group.clone())),
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(group.id)),
            view::Message::Back => Action::operation(Operation::Back),
        },
    }
}

pub fn view<'a>(
    group: &'a ChoiceGroup, 
    mode: &'a Mode,
    other_groups: &'a [&'a ChoiceGroup]
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(group).map(Message::View),
        Mode::Edit => {
            edit::view(
                group,
                edit::EditState::new(group),
                other_groups
            ).map(Message::Edit)
        }
    }
}