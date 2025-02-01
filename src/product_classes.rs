pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
    IdRange,
};
use crate::item_groups::ItemGroup;
use crate::revenue_categories::RevenueCategory;
use crate::Action;
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ProductClass),
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
    MissingItemGroup(String),
    MissingRevenueCategory(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductClass {
    pub id: EntityId,
    pub name: String,
    pub item_group: Option<EntityId>,        // Reference to ItemGroup
    pub revenue_category: Option<EntityId>,   // Reference to RevenueCategory
}

impl std::fmt::Display for ProductClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ProductClass {
    fn validate(
        &self,
        other_classes: &[&ProductClass],
        available_item_groups: &[&ItemGroup],
        available_revenue_categories: &[&RevenueCategory],
    ) -> Result<(), ValidationError> {
        // Validate ID range (1-999 based on your screenshot)
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Product Class ID must be between 1 and 999".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_classes {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Product Class with ID {} already exists", self.id)
                ));
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Product Class name cannot be empty".to_string()
            ));
        }

        // Validate ItemGroup reference exists
        if let Some(group_id) = self.item_group {
            if !available_item_groups.iter().any(|g| g.id == group_id) {
                return Err(ValidationError::MissingItemGroup(
                    format!("Referenced Item Group {} does not exist", group_id)
                ));
            }
        }

        // Validate RevenueCategory reference exists
        if let Some(category_id) = self.revenue_category {
            if !available_revenue_categories.iter().any(|c| c.id == category_id) {
                return Err(ValidationError::MissingRevenueCategory(
                    format!("Referenced Revenue Category {} does not exist", category_id)
                ));
            }
        }

        Ok(())
    }
}

pub fn update(
    class: &mut ProductClass,
    message: Message,
    other_classes: &[&ProductClass],
    available_item_groups: &[&ItemGroup],
    available_revenue_categories: &[&RevenueCategory],
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::Save => {
                match class.validate(other_classes, available_item_groups, available_revenue_categories) {
                    Ok(_) => Action::operation(Operation::Save(class.clone())),
                    Err(e) => Action::none(), // Error will be shown in UI
                }
            },
            edit::Message::Cancel => Action::operation(Operation::Cancel),
            // Other edit messages handled by edit::update
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(class.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
}

pub fn view(
    class: &ProductClass,
    mode: Mode,
    available_item_groups: &[&ItemGroup],
    available_revenue_categories: &[&RevenueCategory],
) -> Element<Message> {
    match mode {
        Mode::View => view::view(class, available_item_groups, available_revenue_categories).map(Message::View),
        Mode::Edit => edit::view(class, available_item_groups, available_revenue_categories).map(Message::Edit),
    }
}