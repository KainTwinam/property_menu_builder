pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
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
    Save(SecurityLevel),
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
pub struct SecurityLevel {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl SecurityLevel {
    fn validate(&self, other_levels: &[&SecurityLevel]) -> Result<(), ValidationError> {
        // Validate ID range (0-9 based on your screenshot)
        if !(0..=9).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Security Level ID must be between 0 and 9".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_levels {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Security Level with ID {} already exists", self.id)
                ));
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Security Level name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    level: &mut SecurityLevel,
    message: Message,
    other_levels: &[&SecurityLevel],
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::Save => {
                match level.validate(other_levels) {
                    Ok(_) => Action::operation(Operation::Save(level.clone())),
                    Err(e) => Action::none(), // Error will be shown in UI
                }
            },
            edit::Message::Cancel => Action::operation(Operation::Cancel),
            // Other edit messages handled by edit::update
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(level.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
}

pub fn view(level: &SecurityLevel, mode: Mode) -> Element<Message> {
    match mode {
        Mode::View => view::view(level).map(Message::View),
        Mode::Edit => edit::view(level).map(Message::Edit),
    }
}