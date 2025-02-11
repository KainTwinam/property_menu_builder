use iced::event;
use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{
    focus_next, focus_previous,
    button, column, container, row, scrollable, text, vertical_space
};
use iced::{Element, Length, Size, Subscription, Task};
use persistence::FileManager;
use std::collections::HashMap;

mod action;
mod settings;
mod items;
mod item_groups;
mod price_levels;
mod product_classes;
mod tax_groups;
mod security_levels;
mod revenue_categories;
mod report_categories;
mod choice_groups;
mod printer_logicals;
mod data_types;
mod persistence;

use crate::{
    items::{Item, ViewContext},
    item_groups::ItemGroup,
    price_levels::PriceLevel,
    product_classes::ProductClass,
    tax_groups::TaxGroup,
    security_levels::SecurityLevel,
    revenue_categories::RevenueCategory,
    report_categories::ReportCategory,
    choice_groups::ChoiceGroup,
    printer_logicals::PrinterLogical,
};

use data_types::EntityId;
pub use action::Action;

fn main() -> iced::Result {
    iced::application(MenuBuilder::title, MenuBuilder::update, MenuBuilder::view)
        .window_size(Size::new(900.0, 700.0))
        .theme(MenuBuilder::theme)
        .antialiasing(true)
        .centered()
        .subscription(MenuBuilder::subscription)
        .run_with(MenuBuilder::new)
}

#[derive(Debug, Clone)]
pub enum Screen {
    Settings(settings::Settings),
    Items(items::Mode),
    ItemGroups(item_groups::Mode),
    PriceLevels(price_levels::Mode),
    ProductClasses(product_classes::Mode),
    TaxGroups(tax_groups::Mode),
    SecurityLevels(security_levels::Mode),
    RevenueCategories(revenue_categories::Mode),
    ReportCategories(report_categories::Mode),
    ChoiceGroups(choice_groups::Mode),
    PrinterLogicals(printer_logicals::Mode),
}

#[derive(Debug, Clone)]
pub enum Message {
    Settings(settings::Message),
    PrinterLogicals(EntityId, printer_logicals::Message),
    Items(EntityId, items::Message),
    ItemGroups(EntityId, item_groups::Message), 
    PriceLevels(EntityId, price_levels::Message),
    ProductClasses(EntityId, product_classes::Message),
    TaxGroups(EntityId, tax_groups::Message),
    SecurityLevels(EntityId, security_levels::Message),
    RevenueCategories(EntityId, revenue_categories::Message),
    ReportCategories(EntityId, report_categories::Message),
    ChoiceGroups(EntityId, choice_groups::Message),
    Navigate(Screen),
    HotKey(HotKey)
}

#[derive(Debug)]
pub enum Operation {
    Settings(settings::Operation),
    Items(EntityId, items::Operation),
    ItemGroups(EntityId, item_groups::Operation),
    PriceLevels(EntityId, price_levels::Operation),
    ProductClasses(EntityId, product_classes::Operation),
    TaxGroups(EntityId, tax_groups::Operation),
    SecurityLevels(EntityId, security_levels::Operation),
    RevenueCategories(EntityId, revenue_categories::Operation),
    ReportCategories(EntityId, report_categories::Operation),
    ChoiceGroups(EntityId, choice_groups::Operation),
    PrinterLogicals(EntityId, printer_logicals::Operation),
}

pub struct MenuBuilder {
    screen: Screen,
    settings: settings::Settings,
    file_manager: persistence::FileManager,
    error_message: Option<String>,
    // Items
    items: HashMap<EntityId, Item>,
    draft_item: Item,
    draft_item_id: Option<EntityId>,
    selected_item_id: Option<EntityId>,
    item_edit_state: items::EditState,
 
    // Item Groups 
    item_groups: HashMap<EntityId, ItemGroup>,
    draft_item_group: ItemGroup,
    draft_item_group_id: Option<EntityId>,
    selected_item_group_id: Option<EntityId>,
    item_group_edit_state: item_groups::EditState,
 
    // Price Levels
    price_levels: HashMap<EntityId, PriceLevel>,
    draft_price_level: PriceLevel,
    draft_price_level_id: Option<EntityId>,
    selected_price_level_id: Option<EntityId>,
    price_level_edit_state: price_levels::EditState,
 
    // Product Classes
    product_classes: HashMap<EntityId, ProductClass>,
    draft_product_class: ProductClass,
    draft_product_class_id: Option<EntityId>,
    selected_product_class_id: Option<EntityId>,
    product_class_edit_state: product_classes::EditState,
 
    // Tax Groups
    tax_groups: HashMap<EntityId, TaxGroup>,
    draft_tax_group: TaxGroup,
    draft_tax_group_id: Option<EntityId>,
    selected_tax_group_id: Option<EntityId>,
    tax_group_edit_state: tax_groups::EditState,
 
    // Security Levels
    security_levels: HashMap<EntityId, SecurityLevel>,
    draft_security_level: SecurityLevel,
    draft_security_level_id: Option<EntityId>,
    selected_security_level_id: Option<EntityId>,
    security_level_edit_state: security_levels::EditState,
 
    // Revenue Categories
    revenue_categories: HashMap<EntityId, RevenueCategory>,
    draft_revenue_category: RevenueCategory,
    draft_revenue_category_id: Option<EntityId>,
    selected_revenue_category_id: Option<EntityId>,
    revenue_category_edit_state: revenue_categories::EditState,
 
    // Report Categories
    report_categories: HashMap<EntityId, ReportCategory>,
    draft_report_category: ReportCategory,
    draft_report_category_id: Option<EntityId>,
    selected_report_category_id: Option<EntityId>,
    report_category_edit_state: report_categories::EditState,
 
    // Choice Groups
    choice_groups: HashMap<EntityId, ChoiceGroup>,
    draft_choice_group: ChoiceGroup,
    draft_choice_group_id: Option<EntityId>,
    selected_choice_group_id: Option<EntityId>,
    choice_group_edit_state: choice_groups::EditState,
 
    // Printer Logicals
    printer_logicals: HashMap<EntityId, PrinterLogical>,
    draft_printer: PrinterLogical,
    draft_printer_id: Option<EntityId>,
    selected_printer_id: Option<EntityId>,
    printer_edit_state: printer_logicals::EditState,

 }
 
 impl Default for MenuBuilder {
    fn default() -> Self {
        // Initialize file manager first
        let file_manager = FileManager::new()
            .expect("Failed to initialize file manager");
        
        // Ensure data directory exists
        file_manager.ensure_data_dir()
            .expect("Failed to create data directory");

        Self {
            screen: Screen::Items(items::Mode::View),
            settings: settings::Settings::default(),
            error_message: None,
            file_manager: file_manager,

            // Items
            items: HashMap::new(),
            draft_item: Item::default(),
            draft_item_id: None,
            selected_item_id: None,
            item_edit_state: items::EditState::default(),
 
            // Item Groups
            item_groups: HashMap::new(),
            draft_item_group: ItemGroup::default(),
            draft_item_group_id: None,
            selected_item_group_id: None,
            item_group_edit_state: item_groups::EditState::default(),
 
            // Price Levels 
            price_levels: HashMap::new(),
            draft_price_level: PriceLevel::default(),
            draft_price_level_id: None,
            selected_price_level_id: None,
            price_level_edit_state: price_levels::EditState::default(),
 
            // Product Classes
            product_classes: HashMap::new(),
            draft_product_class: ProductClass::default(),
            draft_product_class_id: None,
            selected_product_class_id: None,
            product_class_edit_state: product_classes::EditState::default(),
 
            // Tax Groups
            tax_groups: HashMap::new(),
            draft_tax_group: TaxGroup::default(),
            draft_tax_group_id: None,
            selected_tax_group_id: None,
            tax_group_edit_state: tax_groups::EditState::default(),
 
            // Security Levels
            security_levels: HashMap::new(),
            draft_security_level: SecurityLevel::default(),
            draft_security_level_id: None,
            selected_security_level_id: None,
            security_level_edit_state: security_levels::EditState::default(),
 
            // Revenue Categories
            revenue_categories: HashMap::new(),
            draft_revenue_category: RevenueCategory::default(),
            draft_revenue_category_id: None,
            selected_revenue_category_id: None,
            revenue_category_edit_state: revenue_categories::EditState::default(),
 
            // Report Categories
            report_categories: HashMap::new(),
            draft_report_category: ReportCategory::default(),
            draft_report_category_id: None,
            selected_report_category_id: None,
            report_category_edit_state: report_categories::EditState::default(),
 
            // Choice Groups
            choice_groups: HashMap::new(),
            draft_choice_group: ChoiceGroup::default(),
            draft_choice_group_id: None,
            selected_choice_group_id: None,
            choice_group_edit_state: choice_groups::EditState::default(),
 
            // Printer Logicals
            printer_logicals: HashMap::new(),
            draft_printer: PrinterLogical::default(),
            draft_printer_id: None,
            selected_printer_id: None,
            printer_edit_state: printer_logicals::EditState::default(),
        }
    }
 }

impl MenuBuilder {

    fn theme(&self) -> iced::Theme {
        iced::Theme::SolarizedDark
    }

    fn title(&self) -> String {
        String::from("Menu Builder :D")
    }

    fn new() -> (Self, Task<Message>) {
        
        let mut menu_builder = MenuBuilder::default();

        // Try to load state from file
        match menu_builder.load_state() {
            Ok(()) => {
                println!("Successfully loaded saved data");
                menu_builder.error_message = None;
            }
            Err(e) => {
                eprintln!("Failed to load state: {}", e);
                menu_builder.error_message = Some(format!("Failed to load saved data: {}", e));
            }
        }

        (menu_builder, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Settings(msg) => {
                Message::Settings(msg);
                Task::none()
            }
            Message::Items(id, msg) => {
                let cloned_items = self.items.clone();

                if id < 0 {  // New item case

                    let context = ViewContext {
                        available_items: cloned_items,
                        available_choice_groups: self.choice_groups.clone(),
                        available_item_groups: self.item_groups.clone(),
                        available_price_levels: self.price_levels.clone(),
                        available_printer_logicals: self.printer_logicals.clone(),
                        available_product_classes: self.product_classes.clone(),
                        available_report_categories: self.report_categories.clone(),
                        available_revenue_categories: self.revenue_categories.clone(),
                        available_security_levels: self.security_levels.clone(),
                        available_tax_groups: self.tax_groups.clone(),
                    };

                    let action = items::update(
                        &mut self.draft_item,
                        msg,
                        &mut self.item_edit_state,
                        &context
                    )
                    .map_operation(move |o| Operation::Items(id, o))
                    .map(move |m| Message::Items(id, m));

                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                } else {
                    let item = if let Some(draft_id) = self.draft_item_id {
                        if draft_id == id {
                            &mut self.draft_item
                        } else {
                            self.items.get_mut(&id).expect("Item should exist")
                        }
                    } else {
                        self.items.get_mut(&id).expect("Item should exist")
                    };

                    let other_items: HashMap<EntityId, &Item> = cloned_items
                        .iter()
                        .filter(|(&item_id, _)| item_id != id)
                        .map(|(&k, v)| (k, v))
                        .collect();

                let context = ViewContext {
                    available_items: cloned_items,
                    available_choice_groups: self.choice_groups.clone(),
                    available_item_groups: self.item_groups.clone(),
                    available_price_levels: self.price_levels.clone(),
                    available_printer_logicals: self.printer_logicals.clone(),
                    available_product_classes: self.product_classes.clone(),
                    available_report_categories: self.report_categories.clone(),
                    available_revenue_categories: self.revenue_categories.clone(),
                    available_security_levels: self.security_levels.clone(),
                    available_tax_groups: self.tax_groups.clone(),
                };

                let action = items::update(
                    item, 
                    msg, 
                    &mut self.item_edit_state,
                    &context
                )
                    .map_operation(move |o| Operation::Items(id, o))
                    .map(move |m| Message::Items(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)
                }



                
            },
            Message::ItemGroups(id, msg) => {
                let cloned_item_groups = self.item_groups.clone();

                if id < 0 {  // New item group case
                    let other_item_groups: Vec<&ItemGroup> = cloned_item_groups
                    .values()
                    .filter(|ig| ig.id != id)
                    .collect();
    
                    let action = item_groups::update(
                        &mut self.draft_item_group, 
                        msg, 
                        &mut self.item_group_edit_state,
                        &other_item_groups
                    )
                        .map_operation(move |o| Operation::ItemGroups(id, o))
                        .map(move |m| Message::ItemGroups(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)

                } else {
                    let group = if let Some(draft_id) = self.draft_item_group_id {
                        if draft_id == id {
                            &mut self.draft_item_group
                        } else {
                            self.item_groups.get_mut(&id).expect("Item Group should exist")
                        }
                    } else {
                        self.item_groups.get_mut(&id).expect("Item Group should exist")
                    };
    
                    let other_item_groups: Vec<&ItemGroup> = cloned_item_groups
                    .values()
                    .filter(|ig| ig.id != id)
                    .collect();
    
                    let action = item_groups::update(
                        group, 
                        msg, 
                        &mut self.item_group_edit_state,
                        &other_item_groups
                    )
                        .map_operation(move |o| Operation::ItemGroups(id, o))
                        .map(move |m| Message::ItemGroups(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }

                
            },
            Message::PriceLevels(id, msg) => {
                let cloned_price_levels = self.price_levels.clone();

                if id < 0 {  // New price level case
                    let other_price_levels: Vec<&PriceLevel> = cloned_price_levels
                        .values()
                        .filter(|pl| pl.id != id)
                        .collect();
    
                    let action = price_levels::update(
                        &mut self.draft_price_level, 
                        msg, 
                        &mut self.price_level_edit_state,
                        &other_price_levels
                    )
                        .map_operation(move |o| Operation::PriceLevels(id, o))
                        .map(move |m| Message::PriceLevels(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                } else {
                    let price_level = if let Some(draft_id) = self.draft_price_level_id {
                        if draft_id == id {
                            &mut self.draft_price_level
                        } else {
                            self.price_levels.get_mut(&id).expect("Price Level should exist")
                        }
                    } else {
                        self.price_levels.get_mut(&id).expect("Price Level should exist")
                    };
    
                    let other_price_levels: Vec<&PriceLevel> = cloned_price_levels
                        .values()
                        .filter(|pl| pl.id != id)
                        .collect();
    
                    let action = price_levels::update(
                        price_level, 
                        msg, 
                        &mut self.price_level_edit_state,
                        &other_price_levels
                    )
                        .map_operation(move |o| Operation::PriceLevels(id, o))
                        .map(move |m| Message::PriceLevels(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }
            },
            Message::ProductClasses(id, msg) => {
                let cloned_product_classes = self.product_classes.clone();

                if id < 0 {  // New Product Class case
                    let other_product_classes: Vec<&ProductClass> = cloned_product_classes
                    .values()
                    .filter(|pc| pc.id != id)
                    .collect();

                let action = product_classes::update(
                    &mut self.draft_product_class, 
                    msg, 
                    &mut self.product_class_edit_state,
                    &other_product_classes
                )
                    .map_operation(move |o| Operation::ProductClasses(id, o))
                    .map(move |m| Message::ProductClasses(id, m));

                let operation_task = if let Some(operation) = action.operation {
                    self.perform(operation)
                } else {
                    Task::none()
                };

                operation_task.chain(action.task)

                } else {
                    let product_class = if let Some(draft_id) = self.draft_product_class_id {
                        if draft_id == id {
                            &mut self.draft_product_class
                        } else {
                            self.product_classes.get_mut(&id).expect("Product Class should exist")
                        }
                    } else {
                        self.product_classes.get_mut(&id).expect("Product Class should exist")
                    };
    
                    let other_product_classes: Vec<&ProductClass> = cloned_product_classes
                        .values()
                        .filter(|pc| pc.id != id)
                        .collect();
    
                    let action = product_classes::update(
                        product_class, 
                        msg, 
                        &mut self.product_class_edit_state,
                        &other_product_classes
                    )
                        .map_operation(move |o| Operation::ProductClasses(id, o))
                        .map(move |m| Message::ProductClasses(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }

                
            },
            Message::TaxGroups(id, msg) => {
                let cloned_tax_groups = self.tax_groups.clone();

                if id < 0 {  // New Tax Group case
                    let other_tax_groups: Vec<&TaxGroup> = cloned_tax_groups
                        .values()
                        .filter(|tg| tg.id != id)
                        .collect();
    
                    let action = tax_groups::update(
                        &mut self.draft_tax_group, 
                        msg, 
                        &mut self.tax_group_edit_state,
                        &other_tax_groups
                    )
                        .map_operation(move |o| Operation::TaxGroups(id, o))
                        .map(move |m| Message::TaxGroups(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                } else {
                    let tax_group = if let Some(draft_id) = self.draft_tax_group_id {
                        if draft_id == id {
                            &mut self.draft_tax_group
                        } else {
                            self.tax_groups.get_mut(&id).expect("Tax Group should exist")
                        }
                    } else {
                        self.tax_groups.get_mut(&id).expect("Tax Group should exist")
                    };
    
                    let other_tax_groups: Vec<&TaxGroup> = cloned_tax_groups
                        .values()
                        .filter(|tg| tg.id != id)
                        .collect();
    
                    let action = tax_groups::update(
                        tax_group, 
                        msg, 
                        &mut self.tax_group_edit_state,
                        &other_tax_groups
                    )
                        .map_operation(move |o| Operation::TaxGroups(id, o))
                        .map(move |m| Message::TaxGroups(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }
            },
            Message::SecurityLevels(id, msg) => {
                let cloned_security_levels = self.security_levels.clone();

                if id < 0 {  // New Security Level case
                    let other_security_levels: Vec<&SecurityLevel> = cloned_security_levels
                        .values()
                        .filter(|sl| sl.id != id)
                        .collect();
    
                    let action = security_levels::update(
                        &mut self.draft_security_level, 
                        msg, 
                        &mut self.security_level_edit_state,
                        &other_security_levels,
                    )
                        .map_operation(move |o| Operation::SecurityLevels(id, o))
                        .map(move |m| Message::SecurityLevels(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                } else {
                    let security_level = if let Some(draft_id) = self.draft_security_level_id {
                        if draft_id == id {
                            &mut self.draft_security_level
                        } else {
                            self.security_levels.get_mut(&id).expect("Security Level should exist")
                        }
                    } else {
                        self.security_levels.get_mut(&id).expect("Security Level should exist")
                    };
    
                    let other_security_levels: Vec<&SecurityLevel> = cloned_security_levels
                        .values()
                        .filter(|sl| sl.id != id)
                        .collect();
    
                    let action = security_levels::update(
                        security_level, 
                        msg, 
                        &mut self.security_level_edit_state,
                        &other_security_levels,
                    )
                        .map_operation(move |o| Operation::SecurityLevels(id, o))
                        .map(move |m| Message::SecurityLevels(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }
            },
            Message::RevenueCategories(id, msg) => {
                let cloned_revenue_categories = self.revenue_categories.clone();

                if id < 0 {  // New Revenue Category case
                    let other_revenue_categories: Vec<&RevenueCategory> = cloned_revenue_categories
                        .values()
                        .filter(|rc| rc.id != id)
                        .collect();
    
                    let action = revenue_categories::update(
                        &mut self.draft_revenue_category, 
                        msg, 
                        &mut self.revenue_category_edit_state,
                        &other_revenue_categories
                    )
                    .map_operation(move |o| Operation::RevenueCategories(id, o))
                    .map(move |m| Message::RevenueCategories(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                } else {
                    let revenue_category = if let Some(draft_id) = self.draft_revenue_category_id {
                        if draft_id == id {
                            &mut self.draft_revenue_category
                        } else {
                            self.revenue_categories.get_mut(&id).expect("Revenue Category should exist")
                        }
                    } else {
                        self.revenue_categories.get_mut(&id).expect("Revenue Category should exist")
                    };
    
                    let other_revenue_categories: Vec<&RevenueCategory> = cloned_revenue_categories
                        .values()
                        .filter(|rc| rc.id != id)
                        .collect();
    
                    let action = revenue_categories::update(
                        revenue_category, 
                        msg, 
                        &mut self.revenue_category_edit_state,
                        &other_revenue_categories
                    )
                    .map_operation(move |o| Operation::RevenueCategories(id, o))
                    .map(move |m| Message::RevenueCategories(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }
            },
            Message::ReportCategories(id, msg) => {
                let cloned_report_categories = self.report_categories.clone();

                if id < 0 {  // New Report Category case
                    let other_report_categories : Vec<&ReportCategory> = cloned_report_categories
                        .values()
                        .filter(|rc| rc.id != id)
                        .collect();
    
    
                    let action = report_categories::update(
                        &mut self.draft_report_category, 
                        msg, 
                        &mut self.report_category_edit_state,
                        &other_report_categories
                    )
                    .map_operation(move |o| Operation::ReportCategories(id, o))
                    .map(move |m| Message::ReportCategories(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                } else {
                    let report_category = if let Some(draft_id) = self.draft_report_category_id {
                        if draft_id == id {
                            &mut self.draft_report_category
                        } else {
                            self.report_categories.get_mut(&id).expect("Report Category should exist")
                        }
                    } else {
                        self.report_categories.get_mut(&id).expect("Report Category should exist")
                    };
    
                    let other_report_categories : Vec<&ReportCategory> = cloned_report_categories
                        .values()
                        .filter(|rc| rc.id != id)
                        .collect();
    
    
                    let action = report_categories::update(
                        report_category, 
                        msg, 
                        &mut self.report_category_edit_state,
                        &other_report_categories
                    )
                    .map_operation(move |o| Operation::ReportCategories(id, o))
                    .map(move |m| Message::ReportCategories(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }
            },
            Message::ChoiceGroups(id, msg) => {
                let cloned_choice_groups = self.choice_groups.clone();

                if id < 0 {  // New Choice Group case
                    let other_choice_groups: Vec<&ChoiceGroup> = cloned_choice_groups
                    .values()
                    .filter(|c| c.id != id)
                    .collect();

                    let action = choice_groups::update(
                        &mut self.draft_choice_group, 
                        msg, 
                        &mut self.choice_group_edit_state, 
                        &other_choice_groups
                    )
                    .map_operation(move |o| Operation::ChoiceGroups(id, o))
                    .map(move |m| Message::ChoiceGroups(id, m));

                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };

                operation_task.chain(action.task)
                } else {
                    let choice_group = if let Some(draft_id) = self.draft_choice_group_id {
                        if draft_id == id {
                            &mut self.draft_choice_group
                        } else {
                            self.choice_groups.get_mut(&id).expect("Choice Group should exist")
                        }
                    } else {
                        self.choice_groups.get_mut(&id).expect("Choice Group should exist")
                    };
    
                    let other_choice_groups: Vec<&ChoiceGroup> = cloned_choice_groups
                        .values()
                        .filter(|c| c.id != id)
                        .collect();
    
                    let action = choice_groups::update(
                        choice_group, 
                        msg, 
                        &mut self.choice_group_edit_state, 
                        &other_choice_groups
                    )
                    .map_operation(move |o| Operation::ChoiceGroups(id, o))
                    .map(move |m| Message::ChoiceGroups(id, m));
    
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
    
                    operation_task.chain(action.task)
                }
            },
            Message::PrinterLogicals(id, msg) => {
                let cloned_printers = self.printer_logicals.clone();

                if id < 0 {  // New Choice Group case
                    // Get other printers for validation
                    let other_printers: Vec<&PrinterLogical> = cloned_printers
                        .values()
                        .filter(|p| p.id != id)
                        .collect();
                
                    let action = printer_logicals::update(
                        &mut self.draft_printer, 
                        msg, 
                        &mut self.printer_edit_state,
                        &other_printers
                    )
                    .map_operation(move |o| Operation::PrinterLogicals(id, o))
                    .map(move |m| Message::PrinterLogicals(id, m));
                
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
                
                    operation_task.chain(action.task)
                } else {
                    let printer = if let Some(draft_id) = self.draft_printer_id {
                        if draft_id == id {
                            &mut self.draft_printer
                        } else {
                            self.printer_logicals.get_mut(&id).expect("Printer should exist")
                        }
                    } else {
                        self.printer_logicals.get_mut(&id).expect("Printer should exist")
                    };
                
                    // Get other printers for validation
                    let other_printers: Vec<&PrinterLogical> = cloned_printers
                        .values()
                        .filter(|p| p.id != id)
                        .collect();
                
                    let action = printer_logicals::update(
                        printer, 
                        msg, 
                        &mut self.printer_edit_state,
                        &other_printers
                    )
                    .map_operation(move |o| Operation::PrinterLogicals(id, o))
                    .map(move |m| Message::PrinterLogicals(id, m));
                
                    let operation_task = if let Some(operation) = action.operation {
                        self.perform(operation)
                    } else {
                        Task::none()
                    };
                
                    operation_task.chain(action.task)
                }
            }
            Message::Navigate(screen) => {
                self.screen = screen;
                Task::none()
            },   
            Message::HotKey(hotkey) => {
                match hotkey {
                    HotKey::Tab(modifiers) => {
                        if modifiers.shift() {
                            focus_previous()
                        } else {
                            focus_next()
                        }
                    }
                    HotKey::Escape => Task::none(),
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {

        let sidebar = container(
            column![
                button("Items")
                    .on_press(Message::Navigate(Screen::Items(items::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Item Groups")
                    .on_press(Message::Navigate(Screen::ItemGroups(item_groups::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Price Levels")
                    .on_press(Message::Navigate(Screen::PriceLevels(price_levels::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Product Classes")
                    .on_press(Message::Navigate(Screen::ProductClasses(product_classes::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Tax Groups")
                    .on_press(Message::Navigate(Screen::TaxGroups(tax_groups::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Security Levels")
                    .on_press(Message::Navigate(Screen::SecurityLevels(security_levels::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Revenue Categories")
                    .on_press(Message::Navigate(Screen::RevenueCategories(revenue_categories::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Report Categories")
                    .on_press(Message::Navigate(Screen::ReportCategories(report_categories::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Choice Groups")
                    .on_press(Message::Navigate(Screen::ChoiceGroups(choice_groups::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Printer Logicals")
                    .on_press(Message::Navigate(Screen::PrinterLogicals(printer_logicals::Mode::View)))
                    .width(Length::Fill)
                    .style(button::secondary),

                vertical_space(),
                row![
                    iced::widget::horizontal_space(),
                    button(text("\u{2699}"))  // Cog icon using Unicode
                        .on_press(Message::Navigate(Screen::Settings(self.settings.clone())))
                        .width(Length::Fixed(40.0))
                        .style(button::secondary),
                ]
            ]
            .spacing(5)
            .padding(10)
        )
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .style(container::rounded_box);

        let content = match &self.screen {
            Screen::Settings(settings) => {
                settings::view(settings, self.error_message.as_deref()).map(Message::Settings)
            },
            Screen::Items(mode) => {
                if let Some(id) = self.selected_item_id {
                    let item = if id < 0 {  // Negative ID indicates new item
                        &self.draft_item
                    } else if let Some(draft_id) = self.draft_item_id {
                        if draft_id == id {
                            &self.draft_item
                        } else {
                            &self.items[&id]
                        }
                    } else {
                        &self.items[&id]
                    };
     
                    items::view(
                        item,
                        mode,
                        &self.items,
                        &self.item_groups,
                        &self.tax_groups,
                        &self.security_levels,
                        &self.revenue_categories,
                        &self.report_categories,
                        &self.product_classes,
                        &self.choice_groups,
                        &self.printer_logicals,
                        &self.price_levels,
                    )
                    .map(move |msg| Message::Items(id, msg))
                } else {
                    // Welcome screen with Create New button
                    container(
                        column![
                            text("Item Management")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.items.is_empty() {
                                column![
                                    text("No items have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Item")
                                        .on_press(Message::Items(-1, items::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Item")
                                        .on_press(Message::Items(-1, items::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::ItemGroups(mode) => {
                if let Some(id) = self.selected_item_group_id {
                    let item_group = if let Some(draft_id) = self.draft_item_group_id {
                        if draft_id == id {
                            &self.draft_item_group
                        } else {
                            &self.item_groups[&id]
                        }
                    } else {
                        &self.item_groups[&id]
                    };
     
                    item_groups::view(item_group, mode, &self.item_groups)
                        .map(move |msg| Message::ItemGroups(id, msg))
                } else {
                    container(
                        column![
                            text("Item Groups")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.item_groups.is_empty() {
                                column![
                                    text("No item groups have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Item Group")
                                        .on_press(Message::ItemGroups(-1, item_groups::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Item Group")
                                        .on_press(Message::ItemGroups(-1, item_groups::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::PriceLevels(mode) => {
                if let Some(id) = self.selected_price_level_id {
                    let price_level = if let Some(draft_id) = self.draft_price_level_id {
                        if draft_id == id {
                            &self.draft_price_level
                        } else {
                            &self.price_levels[&id]
                        }
                    } else {
                        &self.price_levels[&id]
                    };
     
                    price_levels::view(price_level, mode, &self.price_levels)
                        .map(move |msg| Message::PriceLevels(id, msg))
                } else {
                    container(
                        column![
                            text("Price Levels")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.price_levels.is_empty() {
                                column![
                                    text("No price levels have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Price Level")
                                        .on_press(Message::PriceLevels(-1, price_levels::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Price Level")
                                        .on_press(Message::PriceLevels(-1, price_levels::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::ProductClasses(mode) => {
                if let Some(id) = self.selected_product_class_id {
                    let product_class = if let Some(draft_id) = self.draft_product_class_id {
                        if draft_id == id {
                            &self.draft_product_class
                        } else {
                            &self.product_classes[&id]
                        }
                    } else {
                        &self.product_classes[&id]
                    };
     
                    product_classes::view(product_class, mode, &self.product_classes)
                        .map(move |msg| Message::ProductClasses(id, msg))
                } else {
                    container(
                        column![
                            text("Product Classes")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.product_classes.is_empty() {
                                column![
                                    text("No product classes have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Product Class")
                                        .on_press(Message::ProductClasses(-1, product_classes::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Product Class")
                                        .on_press(Message::ProductClasses(-1, product_classes::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::TaxGroups(mode) => {
                if let Some(id) = self.selected_tax_group_id {
                    let tax_group = if let Some(draft_id) = self.draft_tax_group_id {
                        if draft_id == id {
                            &self.draft_tax_group
                        } else {
                            &self.tax_groups[&id]
                        }
                    } else {
                        &self.tax_groups[&id]
                    };
     
                    tax_groups::view(tax_group, mode, &self.tax_groups)
                        .map(move |msg| Message::TaxGroups(id, msg))
                } else {
                    container(
                        column![
                            text("Tax Groups")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.tax_groups.is_empty() {
                                column![
                                    text("No tax groups have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Tax Group")
                                        .on_press(Message::TaxGroups(-1, tax_groups::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Tax Group")
                                        .on_press(Message::TaxGroups(-1, tax_groups::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::SecurityLevels(mode) => {
                if let Some(id) = self.selected_security_level_id {
                    let security_level = if let Some(draft_id) = self.draft_security_level_id {
                        if draft_id == id {
                            &self.draft_security_level
                        } else {
                            &self.security_levels[&id]
                        }
                    } else {
                        &self.security_levels[&id]
                    };
     
                    security_levels::view(security_level, mode, &self.security_levels)
                        .map(move |msg| Message::SecurityLevels(id, msg))
                } else {
                    container(
                        column![
                            text("Security Levels")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.security_levels.is_empty() {
                                column![
                                    text("No security levels have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Security Level")
                                        .on_press(Message::SecurityLevels(-1, security_levels::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Security Level")
                                        .on_press(Message::SecurityLevels(-1, security_levels::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::RevenueCategories(mode) => {
                if let Some(id) = self.selected_revenue_category_id {
                    let revenue_category = if let Some(draft_id) = self.draft_revenue_category_id {
                        if draft_id == id {
                            &self.draft_revenue_category
                        } else {
                            &self.revenue_categories[&id]
                        }
                    } else {
                        &self.revenue_categories[&id]
                    };
     
                    revenue_categories::view(revenue_category, mode, &self.revenue_categories)
                        .map(move |msg| Message::RevenueCategories(id, msg))
                } else {
                    container(
                        column![
                            text("Revenue Categories")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.revenue_categories.is_empty() {
                                column![
                                    text("No revenue categories have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Revenue Category")
                                        .on_press(Message::RevenueCategories(-1, revenue_categories::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Revenue Category")
                                        .on_press(Message::RevenueCategories(-1, revenue_categories::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::ReportCategories(mode) => {
                if let Some(id) = self.selected_report_category_id {
                    let report_category = if let Some(draft_id) = self.draft_report_category_id {
                        if draft_id == id {
                            &self.draft_report_category
                        } else {
                            &self.report_categories[&id]
                        }
                    } else {
                        &self.report_categories[&id]
                    };
     
                    report_categories::view(report_category, mode, &self.report_categories)
                        .map(move |msg| Message::ReportCategories(id, msg))
                } else {
                    container(
                        column![
                            text("Report Categories")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.report_categories.is_empty() {
                                column![
                                    text("No report categories have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Report Category")
                                        .on_press(Message::ReportCategories(-1, report_categories::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Report Category")
                                        .on_press(Message::ReportCategories(-1, report_categories::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::ChoiceGroups(mode) => {
                if let Some(id) = self.selected_choice_group_id {
                    let choice_group = if let Some(draft_id) = self.draft_choice_group_id {
                        if draft_id == id {
                            &self.draft_choice_group
                        } else {
                            &self.choice_groups[&id]
                        }
                    } else {
                        &self.choice_groups[&id]
                    };
     
                    choice_groups::view(choice_group, mode, &self.choice_groups)
                        .map(move |msg| Message::ChoiceGroups(id, msg))
                } else {
                    container(
                        column![
                            text("Choice Groups")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.choice_groups.is_empty() {
                                column![
                                    text("No choice groups have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Choice Group")
                                        .on_press(Message::ChoiceGroups(-1, choice_groups::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Choice Group")
                                        .on_press(Message::ChoiceGroups(-1, choice_groups::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space()
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
            Screen::PrinterLogicals(mode) => {
                if let Some(id) = self.selected_printer_id {
                    let printer = if let Some(draft_id) = self.draft_printer_id {
                        if draft_id == id {
                            &self.draft_printer
                        } else {
                            &self.printer_logicals[&id]
                        }
                    } else {
                        &self.printer_logicals[&id]
                    };
     
                    printer_logicals::view(printer, mode, &self.printer_logicals)
                        .map(move |msg| Message::PrinterLogicals(id, msg))
                } else {
                    container(
                        column![
                            text("Printer Logicals")
                                .size(24)
                                .width(Length::Fill),
                            vertical_space(),
                            if self.printer_logicals.is_empty() {
                                column![
                                    text("No printer logicals have been created yet.")
                                        .width(Length::Fill),
                                    vertical_space(),
                                    button("Create New Printer Logical")
                                        .on_press(Message::PrinterLogicals(-1, printer_logicals::Message::CreateNew))
                                        .style(button::primary)
                                ]
                            } else {
                                column![
                                    button("Create New Printer Logical")
                                        .on_press(Message::PrinterLogicals(-1, printer_logicals::Message::CreateNew))
                                        .style(button::primary),
                                    vertical_space(),
                                ]
                            }
                        ]
                        .spacing(10)
                        .max_width(500)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .padding(30)
                    .into()
                }
            }
        };

        row![
            sidebar,
            container(content)
                .width(Length::Fill)
                .padding(20)
        ]
        .into()
     }


    fn perform(&mut self, operation: Operation) -> Task<Message> {
        match operation {
            Operation::Settings(op) => {
                match op {
                    settings::Operation::Save(new_settings) => {
                        self.settings = new_settings;

                        if let Err(e) = self.save_state() {
                            self.error_message = Some(e);
                        } else {
                            self.error_message = None;
                        }

                        self.screen = Screen::Settings(self.settings.clone());
                        Task::none()
                    }
                    settings::Operation::Back => {
                        self.screen = Screen::Items(items::Mode::View);
                        self.error_message = None;
                        Task::none()
                    }
                    settings::Operation::ShowError(error) => {
                        self.error_message = Some(error);
                        Task::none()
                    }
                }
            }
            Operation::Items(id, op) => {
                match op {
                    items::Operation::Save(mut item) => {

                        if item.id < 0 {
                            let next_id = self.items
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            item.id = next_id;

                            self.items.insert(next_id, item.clone());
                            self.draft_item_id = None;
                            self.draft_item = Item::default();
                            self.selected_item_id = Some(next_id);
                        } else {
                            self.items.insert(item.id, item.clone());
                            self.selected_item_id = Some(item.id);
                        }
                        self.screen = Screen::Items(items::Mode::View);

                        if self.settings.auto_save {
                            if let Err(e) = self.save_state() {
                                self.handle_save_error(e);
                            }
                        }

                        Task::none()
                    }
                    items::Operation::StartEdit(id) => {
                        // Start editing an existing Item
                        self.draft_item_id = Some(id);
                        self.draft_item = self.items[&id].clone();
                        self.screen = Screen::Items(items::Mode::Edit);
                        Task::none()
                    }
                    items::Operation::Cancel => {
                        if self.draft_item_id.is_some() {
                            self.draft_item_id = None;
                            self.draft_item = Item::default();
                        }
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    }
                    items::Operation::Back => {
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    }
                    items::Operation::ExportToCsv => {
                        todo!();
                        Task::none()
                    }
                    items::Operation::CreateNew(mut item) => {
                        let next_id = self.items
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        item.id = next_id;

                        self.draft_item = item;
                        self.draft_item_id = Some(-1);
                        self.selected_item_id = Some(-1);
                        self.screen = Screen::Items(items::Mode::Edit);
                        Task::none()
                    },
                    items::Operation::Select(id) => {
                        self.selected_item_id = Some(id);
                        self.screen = Screen::Items(items::Mode::View);
                        Task::none()
                    },
                }
            }
    
            Operation::ItemGroups(id, op) => {
                match op {
                    item_groups::Operation::Save(mut group) => {
                        if group.id < 0 {
                            //Generate new ID only for new items
                            let next_id = self.item_groups
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            group.id = next_id;

                            self.item_groups.insert(next_id, group.clone());
                            self.draft_item_group_id = None;
                            self.draft_item_group = ItemGroup::default();
                            self.selected_item_group_id = Some(group.id);
                        } else {
                            self.item_groups.insert(group.id, group.clone());
                            self.selected_item_group_id = Some(group.id);
                        }
                        self.screen = Screen::ItemGroups(item_groups::Mode::View);
                        Task::none()
                    }
                    item_groups::Operation::StartEdit(id) => {
                        // Start editing an existing Item group
                        self.draft_item_group_id = Some(id);
                        self.draft_item_group = self.item_groups[&id].clone();
                        self.screen = Screen::ItemGroups(item_groups::Mode::Edit);
                        Task::none()
                    }
                    item_groups::Operation::Cancel => {
                        if self.draft_item_group_id.is_some() {
                            self.draft_item_group_id = None;
                            self.draft_item_group = ItemGroup::default();
                        }
                        self.screen = Screen::ItemGroups(item_groups::Mode::View);
                        Task::none()
                    }
                    item_groups::Operation::Back => {
                        self.screen = Screen::ItemGroups(item_groups::Mode::View);
                        Task::none()
                    }
                    item_groups::Operation::CreateNew(mut group) => {

                        let next_id = self.item_groups
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        group.id = next_id;

                        self.draft_item_group = group;
                        self.draft_item_group_id = Some(-1);
                        self.selected_item_group_id = Some(-1);
                        self.screen = Screen::ItemGroups(item_groups::Mode::Edit);
                        Task::none()
                    },
                    item_groups::Operation::Select(item_group_id) => {
                        self.selected_item_group_id = Some(item_group_id);
                        self.screen = Screen::ItemGroups(item_groups::Mode::View);
                        Task::none()
                    },
                }
            }
    
            Operation::TaxGroups(id, op) => {
                match op {
                    tax_groups::Operation::Save(mut group) => {

                        if group.id < 0 {
                            let next_id = self.tax_groups
                                .keys()
                                .max()
                                .map_or(1, |max_id|  max_id + 1);
                            group.id = next_id;

                            self.tax_groups.insert(next_id, group.clone());
                            self.draft_tax_group_id = None;
                            self.draft_tax_group = TaxGroup::default();
                            self.selected_tax_group_id = Some(next_id);
                        } else {
                            self.tax_groups.insert(group.id, group.clone());
                            self.selected_tax_group_id = Some(group.id);
                        }
                        self.screen = Screen::TaxGroups(tax_groups::Mode::View);
                        Task::none()
                    }
                    tax_groups::Operation::StartEdit(id) => {
                        // Start editing an existing Security Level
                        self.draft_tax_group_id = Some(id);
                        self.draft_tax_group = self.tax_groups[&id].clone();
                        self.screen = Screen::TaxGroups(tax_groups::Mode::Edit);
                        Task::none()
                    }
                    tax_groups::Operation::Cancel => {
                        if self.draft_tax_group_id.is_some() {
                            self.draft_tax_group_id = None;
                            self.draft_tax_group = TaxGroup::default();
                        }
                        self.screen = Screen::TaxGroups(tax_groups::Mode::View);
                        Task::none()
                    }
                    tax_groups::Operation::Back => {
                        self.screen = Screen::TaxGroups(tax_groups::Mode::View);
                        Task::none()
                    }
                    tax_groups::Operation::CreateNew(mut tax_group) => {
                        let next_id = self.tax_groups
                                .keys()
                                .max()
                                .map_or(1, |max_id|  max_id + 1);
                            tax_group.id = next_id;
                        
                        self.draft_tax_group = tax_group;
                        self.draft_tax_group_id = Some(-1);
                        self.selected_tax_group_id = Some(-1);
                        self.screen = Screen::TaxGroups(tax_groups::Mode::Edit);
                        Task::none()
                    },
                    tax_groups::Operation::Select(id) => {
                        self.selected_tax_group_id = Some(id);
                        self.screen = Screen::TaxGroups(tax_groups::Mode::View);
                        Task::none()
                    },
                }
            }
    
            Operation::SecurityLevels(id, op) => {
                match op {
                    security_levels::Operation::Save(mut level) => {

                        if level.id < 0 {
                            let next_id = self.security_levels
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            level.id = next_id;

                            self.security_levels.insert(next_id, level.clone());
                            self.draft_security_level_id = None;
                            self.draft_security_level = SecurityLevel::default();
                            self.selected_security_level_id = Some(next_id);
                        } else {
                            self.security_levels.insert(level.id, level.clone());
                            self.selected_security_level_id = Some(level.id);
                        }
                        self.screen = Screen::SecurityLevels(security_levels::Mode::View);
                        Task::none()
                    }
                    security_levels::Operation::StartEdit(id) => {
                        // Start editing an existing Security Level
                        self.draft_security_level_id = Some(id);
                        self.draft_security_level = self.security_levels[&id].clone();
                        self.screen = Screen::SecurityLevels(security_levels::Mode::Edit);
                        Task::none()
                    }
                    security_levels::Operation::Cancel => {
                        if self.draft_security_level_id.is_some() {
                            self.draft_security_level_id = None;
                            self.draft_security_level = SecurityLevel::default();
                        }
                        self.screen = Screen::SecurityLevels(security_levels::Mode::View);
                        Task::none()
                    }
                    security_levels::Operation::Back => {
                        self.screen = Screen::SecurityLevels(security_levels::Mode::View);
                        Task::none()
                    }
                    security_levels::Operation::CreateNew(mut level) => {
                        println!("CreateNew operation received in main");
                        let next_id = self.security_levels
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        level.id = next_id;

                        self.draft_security_level = level;
                        self.draft_security_level_id = Some(-1);
                        self.selected_security_level_id = Some(-1);
                        self.screen = Screen::SecurityLevels(security_levels::Mode::Edit);
                        Task::none()
                    },
                    security_levels::Operation::Select(id) => {
                        self.selected_security_level_id = Some(id);
                        self.screen = Screen::SecurityLevels(security_levels::Mode::View);
                        Task::none()
                    },
                }
            }
    
            Operation::RevenueCategories(id, op) => {
                match op {
                    revenue_categories::Operation::Save(mut category) => {
                        if category.id < 0 {
                            let next_id = self.revenue_categories
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            category.id = next_id;

                            self.revenue_categories.insert(next_id, category.clone());
                            self.draft_revenue_category_id = None;
                            self.draft_revenue_category = RevenueCategory::default();
                            self.selected_revenue_category_id = Some(next_id);
                        } else {
                            self.revenue_categories.insert(category.id, category.clone());
                            self.selected_revenue_category_id = Some(category.id);
                        }
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::View);
                        Task::none()
                    }
                    revenue_categories::Operation::StartEdit(id) => {
                        // Start editing an existing revenue category
                        self.draft_revenue_category_id = Some(id);
                        self.draft_revenue_category = self.revenue_categories[&id].clone();
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::Edit);
                        Task::none()
                    }
                    revenue_categories::Operation::Cancel => {
                        if self.draft_revenue_category_id.is_some() {
                            self.draft_revenue_category_id = None;
                            self.draft_revenue_category = RevenueCategory::default();
                        }
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::View);
                        Task::none()
                    }
                    revenue_categories::Operation::Back => {
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::View);
                        Task::none()
                    }
                    revenue_categories::Operation::CreateNew(mut revenue_category) => {
                        let next_id = self.revenue_categories
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        revenue_category.id = next_id;

                        self.draft_revenue_category = revenue_category;
                        self.draft_revenue_category_id = Some(-1);
                        self.selected_revenue_category_id = Some(-1);
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::Edit);
                        Task::none()
                    },
                    revenue_categories::Operation::Select(id) => {
                        self.selected_revenue_category_id = Some(id);
                        self.screen = Screen::RevenueCategories(revenue_categories::Mode::View);
                        Task::none()
                    },
                }
            }
    
            Operation::ReportCategories(id, op) => {
                match op {
                    report_categories::Operation::Save(mut category) => {

                        if category.id < 0 {
                            let next_id = self.report_categories
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            category.id = next_id;

                            self.report_categories.insert(next_id, category.clone());
                            self.draft_report_category_id = None;
                            self.draft_report_category = ReportCategory::default();
                            self.selected_report_category_id = Some(next_id);
                        } else {
                            self.report_categories.insert(category.id, category.clone());
                            self.selected_report_category_id = Some(category.id);
                        }
                        self.screen = Screen::ReportCategories(report_categories::Mode::View);
                        Task::none()
                    }
                    report_categories::Operation::StartEdit(id) => {
                        // Start editing an existing report category
                        self.draft_report_category_id = Some(id);
                        self.draft_report_category = self.report_categories[&id].clone();
                        self.screen = Screen::ReportCategories(report_categories::Mode::Edit);
                        Task::none()
                    }
                    report_categories::Operation::Cancel => {
                        if self.draft_report_category_id.is_some() {
                            self.draft_report_category_id = None;
                            self.draft_report_category = ReportCategory::default();
                        }
                        self.screen = Screen::ReportCategories(report_categories::Mode::View);
                        Task::none()
                    }
                    report_categories::Operation::Back => {
                        self.screen = Screen::ReportCategories(report_categories::Mode::View);
                        Task::none()
                    }
                    report_categories::Operation::CreateNew(mut report_category) => {
                        let next_id = self.report_categories
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        report_category.id = next_id;

                        self.draft_report_category = report_category;
                        self.draft_report_category_id = Some(-1);
                        self.selected_report_category_id = Some(-1);
                        self.screen = Screen::ReportCategories(report_categories::Mode::Edit);
                        Task::none()
                    },
                    report_categories::Operation::Select(id) => {
                        self.selected_report_category_id = Some(id);
                        self.screen = Screen::ReportCategories(report_categories::Mode::View);
                        Task::none()
                    },
                }
            }
    
            Operation::ProductClasses(id, op) => {
                match op {
                    product_classes::Operation::Save(mut class) => {
                        if class.id < 0 {
                            let next_id = self.product_classes
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            class.id = next_id;

                            self.product_classes.insert(next_id, class.clone());
                            self.draft_product_class_id = None;
                            self.draft_product_class = ProductClass::default();
                            self.selected_product_class_id = Some(next_id);
                        } else {
                            self.product_classes.insert(class.id, class.clone());
                            self.selected_product_class_id = Some(class.id);
                        }
                        self.screen = Screen::ProductClasses(product_classes::Mode::View);
                        Task::none()
                    }
                    product_classes::Operation::StartEdit(id) => {
                        // Start editing an existing product class
                        self.draft_product_class_id = Some(id);
                        self.draft_product_class = self.product_classes[&id].clone();
                        self.screen = Screen::ProductClasses(product_classes::Mode::Edit);
                        Task::none()
                    }
                    product_classes::Operation::Cancel => {
                        if self.draft_product_class_id.is_some() {
                            self.draft_product_class_id = None;
                            self.draft_product_class = ProductClass::default();
                        }
                        self.screen = Screen::ProductClasses(product_classes::Mode::View);
                        Task::none()
                    }
                    product_classes::Operation::Back => {
                        self.screen = Screen::ProductClasses(product_classes::Mode::View);
                        Task::none()
                    }
                    product_classes::Operation::CreateNew(mut product_class) => {
                        let next_id = self.product_classes
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        product_class.id = next_id;

                        self.draft_product_class = product_class;
                        self.draft_product_class_id = Some(-1);
                        self.selected_product_class_id = Some(-1);
                        self.screen = Screen::ProductClasses(product_classes::Mode::Edit);
                        Task::none()
                    },
                    product_classes::Operation::Select(id) => {
                        self.selected_product_class_id = Some(id);
                        self.screen = Screen::ProductClasses(product_classes::Mode::View);
                        Task::none()
                    },
                }
            }
    
            Operation::ChoiceGroups(id, op) => match op {
                choice_groups::Operation::Save(mut choice_group) => {
                    if choice_group.id < 0 {
                        // Only generate new ID for new items
                        let next_id = self.choice_groups
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        choice_group.id = next_id;
                        
                        // Insert the new choice group
                        self.choice_groups.insert(next_id, choice_group.clone());
                        self.draft_choice_group_id = None;
                        self.draft_choice_group = ChoiceGroup::default();
                        self.selected_choice_group_id = Some(next_id); // Update selection
                    } else {
                        // Updating existing choice group - keep same ID
                        self.choice_groups.insert(choice_group.id, choice_group.clone());
                        self.selected_choice_group_id = Some(choice_group.id); // Keep selection
                    }
                    self.screen = Screen::ChoiceGroups(choice_groups::Mode::View);
                    Task::none()
                }
                choice_groups::Operation::StartEdit(choice_group_id) => {
                    // Start editing existing choice group
                    self.draft_choice_group_id = Some(choice_group_id);
                    self.draft_choice_group = self.choice_groups[&choice_group_id].clone();
                    self.screen = Screen::ChoiceGroups(choice_groups::Mode::Edit);
                    Task::none()
                }
                choice_groups::Operation::Cancel => {
                    if self.draft_choice_group_id.is_some() {
                        self.draft_choice_group_id = None;
                        self.draft_choice_group = ChoiceGroup::default();
                    }
                    self.screen = Screen::ChoiceGroups(choice_groups::Mode::View);
                    Task::none()
                }
                choice_groups::Operation::Back => {
                    self.screen = Screen::Items(items::Mode::View);
                    Task::none()
                }
                choice_groups::Operation::CreateNew(mut choice_group) => {
                    let next_id = self.choice_groups
                                .keys()
                                .max()
                                .map_or(1, |max_id| max_id + 1);
                            choice_group.id = next_id;
                    //let new_id = ChoiceGroup::new_draft();
                    self.draft_choice_group = choice_group;
                    self.draft_choice_group_id = Some(-1);
                    self.selected_choice_group_id = Some(-1);
                    //self.draft_choice_group_id = Some(next_id);
                    //self.selected_choice_group_id = Some(next_id);
                    self.screen = Screen::ChoiceGroups(choice_groups::Mode::Edit);
                    Task::none()
                },
                choice_groups::Operation::Select(choice_group_id) => {
                    self.selected_choice_group_id = Some(choice_group_id);
                    self.screen = Screen::ChoiceGroups(choice_groups::Mode::View);
                    Task::none()
                },
            },
    
            Operation::PrinterLogicals(id, op) => match op {
                printer_logicals::Operation::Save(mut printer) => {

                    if printer.id < 0 {
                        let next_id = self.printer_logicals
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        printer.id = next_id;

                        self.printer_logicals.insert(next_id, printer.clone());
                        self.draft_printer_id = None;
                        self.draft_printer = PrinterLogical::default();
                        self.selected_printer_id = Some(next_id);
                    } else {
                        self.printer_logicals.insert(printer.id, printer.clone());
                        self.selected_printer_id = Some(printer.id);
                    }
                    self.screen = Screen::PrinterLogicals(printer_logicals::Mode::View);
                    Task::none()
                }
                printer_logicals::Operation::StartEdit(printer_id) => {
                    // Start editing existing printer
                    self.draft_printer_id = Some(printer_id);
                    self.draft_printer = self.printer_logicals[&printer_id].clone();
                    self.screen = Screen::PrinterLogicals(printer_logicals::Mode::Edit);
                    Task::none()
                }
                printer_logicals::Operation::Cancel => {
                    if self.draft_printer_id.is_some() {
                        self.draft_printer_id = None;
                        self.draft_printer = PrinterLogical::default();
                    }
                    self.screen = Screen::PrinterLogicals(printer_logicals::Mode::View);
                    Task::none()
                }
                printer_logicals::Operation::Back => {
                    self.screen = Screen::PrinterLogicals(printer_logicals::Mode::View);
                    Task::none()
                }
                printer_logicals::Operation::CreateNew(mut printer_logical) => {
                    let next_id = self.printer_logicals
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                    printer_logical.id = next_id;
                    
                    self.draft_printer = printer_logical;
                    self.draft_printer_id = Some(-1);
                    self.selected_printer_id = Some(-1);
                    self.screen = Screen::PrinterLogicals(printer_logicals::Mode::Edit);
                    Task::none()
                },
                printer_logicals::Operation::Select(printer_logical_id) => {
                    self.selected_printer_id = Some(printer_logical_id);
                    self.screen = Screen::PrinterLogicals(printer_logicals::Mode::View);
                    Task::none()
                },
            },

            Operation::PriceLevels(id, op) => match op {
                price_levels::Operation::Save(mut level) => {
                    if level.id < 0 {
                        let next_id = self.price_levels
                            .keys()
                            .max()
                            .map_or(1, |max_id| max_id + 1);
                        level.id = next_id;

                        self.price_levels.insert(id, level);
                        self.draft_price_level_id = None;
                        self.draft_price_level = PriceLevel::default();
                        self.selected_price_level_id = Some(next_id);
                    } else {
                        self.price_levels.insert(level.id, level.clone());
                        self.selected_price_level_id = Some(level.id);
                    }
                    self.screen = Screen::PriceLevels(price_levels::Mode::View);
                    Task::none()
                }
                price_levels::Operation::StartEdit(id) => {
                    //start editing existing price level
                    self.draft_price_level_id = Some(id);
                    self.draft_price_level = self.price_levels[&id].clone();
                    self.screen = Screen::PriceLevels(price_levels::Mode::Edit);
                    Task::none()
                }
                price_levels::Operation::Cancel => {
                    if self.draft_price_level_id.is_some() {
                        self.draft_price_level_id = None;
                        self.draft_price_level = PriceLevel::default();
                    }
                    self.screen = Screen::PriceLevels(price_levels::Mode::View);
                    Task::none()
                }
                price_levels::Operation::Back => {
                    self.screen = Screen::PriceLevels(price_levels::Mode::View);
                    Task::none()
                }
                price_levels::Operation::CreateNew(mut price_level) => {

                    let next_id = self.price_levels
                        .keys()
                        .max()
                        .map_or(1, |max_id| max_id + 1);
                    price_level.id = next_id;

                    self.draft_price_level = price_level;
                    self.draft_price_level_id = Some(-1);
                    self.selected_price_level_id = Some(-1);
                    self.screen = Screen::PriceLevels(price_levels::Mode::Edit);
                    Task::none()
                },
                price_levels::Operation::Select(price_level_id) => {
                    self.selected_price_level_id = Some(price_level_id);
                    self.screen = Screen::PriceLevels(price_levels::Mode::View);
                    Task::none()
                },
            },

        }
    }

    pub fn save_state(&self) -> Result<(), String> {
        let state = persistence::AppState {
            items: self.items.values().cloned().collect(),
            item_groups: self.item_groups.values().cloned().collect(),
            price_levels: self.price_levels.values().cloned().collect(),
            product_classes: self.product_classes.values().cloned().collect(),
            tax_groups: self.tax_groups.values().cloned().collect(),
            security_levels: self.security_levels.values().cloned().collect(),
            revenue_categories: self.revenue_categories.values().cloned().collect(),
            report_categories: self.report_categories.values().cloned().collect(),
            choice_groups: self.choice_groups.values().cloned().collect(),
            printer_logicals: self.printer_logicals.values().cloned().collect(),
            settings: self.settings.clone(),
        };

        if self.settings.create_backups {
            self.file_manager.create_backup(std::path::Path::new(&self.settings.file_path))?;
        }

        persistence::save_to_file(&state, &self.settings.file_path)
    }

    fn handle_save_error(&mut self, error: String) {
        self.error_message = Some(error);
        // Optionally switch to settings screen to show error
        self.screen = Screen::Settings(self.settings.clone());
    }

    pub fn load_state(&mut self) -> Result<(), String> {
        // Check if file exists
        let path = std::path::Path::new(&self.settings.file_path);
        if !path.exists() {
            println!("No saved data file found at: {}", self.settings.file_path);
            return Ok(());  // Not an error if file doesn't exist yet
        }

        let state = persistence::load_from_file(&self.settings.file_path)?;

        // Convert Vec to HashMap using id as key
        self.items = state.items.into_iter().map(|i| (i.id, i)).collect();
        self.item_groups = state.item_groups.into_iter().map(|i| (i.id, i)).collect();
        self.price_levels = state.price_levels.into_iter().map(|i| (i.id, i)).collect();
        self.product_classes = state.product_classes.into_iter().map(|i| (i.id, i)).collect();
        self.tax_groups = state.tax_groups.into_iter().map(|i| (i.id, i)).collect();
        self.security_levels = state.security_levels.into_iter().map(|i| (i.id, i)).collect();
        self.revenue_categories = state.revenue_categories.into_iter().map(|i| (i.id, i)).collect();
        self.report_categories = state.report_categories.into_iter().map(|i| (i.id, i)).collect();
        self.choice_groups = state.choice_groups.into_iter().map(|i| (i.id, i)).collect();
        self.printer_logicals = state.printer_logicals.into_iter().map(|i| (i.id, i)).collect();
        self.settings = state.settings.clone();

        // Only update settings if they exist in the loaded state
        if state.settings.file_path.is_empty() {
            // Keep current settings if none in file
            println!("No settings found in save file, keeping current settings");
        } else {
            self.settings = state.settings;
        }

        Ok(())
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(handle_event)
    }
}

#[derive(Debug, Clone)]
pub enum HotKey {
    Escape,
    Tab(Modifiers),
}

fn handle_event(event: event::Event, _: event::Status, _: iced::window::Id) -> Option<Message> {
    match event {
        event::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            match key {
                Key::Named(keyboard::key::Named::Escape) => Some(Message::HotKey(HotKey::Escape)),
                Key::Named(keyboard::key::Named::Tab) => Some(Message::HotKey(HotKey::Tab(modifiers))),
                _ => None,
            }
        }
        _ => None,
    }
}