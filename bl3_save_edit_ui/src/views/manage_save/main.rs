use iced::{button, svg, Column, Container, Length, Row};
use strum::Display;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::resources::svgs::{CHARACTER, CURRENCY, GENERAL, INVENTORY, SETTINGS, VEHICLE};
use crate::views;
use crate::views::manage_save::character::CharacterState;
use crate::views::manage_save::currency::CurrencyState;
use crate::views::manage_save::general::GeneralState;
use crate::views::manage_save::inventory::InventoryState;
use crate::views::manage_save::vehicle::VehicleState;
use crate::views::manage_save::{
    character, currency, general, inventory, vehicle, ManageSaveInteractionMessage, ManageSaveState,
};
use crate::views::settings::SettingsState;
use crate::views::{tab_bar_button, ManageTabBarStyle};

#[derive(Debug, Default)]
pub struct SaveViewState {
    tab_bar_state: SaveTabBarState,
    pub general_state: GeneralState,
    pub character_state: CharacterState,
    pub inventory_state: InventoryState,
    pub currency_state: CurrencyState,
    pub vehicle_state: VehicleState,
}

#[derive(Debug, Default)]
pub struct SaveTabBarState {
    general_button_state: button::State,
    character_button_state: button::State,
    inventory_button_state: button::State,
    currency_button_state: button::State,
    vehicle_button_state: button::State,
    settings_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum SaveTabBarInteractionMessage {
    General,
    Character,
    Inventory,
    Currency,
    Vehicle,
    Settings,
}

#[derive(Debug, Display, PartialEq)]
#[strum(serialize_all = "title_case")]
pub enum SaveTabBarView {
    属性,
    角色,
    背包,
    资产,
    载具,
    设置,
}

pub fn view<'a>(
    settings_state: &'a mut SettingsState,
    manage_save_state: &'a mut ManageSaveState,
    tab_bar_view: &SaveTabBarView,
) -> Container<'a, Bl3Message> {
    let general_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .general_button_state,
        SaveTabBarView::属性,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::General,
        )),
        svg::Handle::from_memory(GENERAL),
        100,
    );

    let character_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .character_button_state,
        SaveTabBarView::角色,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Character,
        )),
        svg::Handle::from_memory(CHARACTER),
        115,
    );

    let inventory_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .inventory_button_state,
        SaveTabBarView::背包,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Inventory,
        )),
        svg::Handle::from_memory(INVENTORY),
        115,
    );

    let currency_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .currency_button_state,
        SaveTabBarView::资产,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Currency,
        )),
        svg::Handle::from_memory(CURRENCY),
        105,
    );

    let vehicle_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .vehicle_button_state,
        SaveTabBarView::载具,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Vehicle,
        )),
        svg::Handle::from_memory(VEHICLE),
        100,
    );

    let settings_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .settings_button_state,
        SaveTabBarView::设置,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Settings,
        )),
        svg::Handle::from_memory(SETTINGS),
        105,
    );

    let tab_bar = Container::new(
        Row::new()
            .push(general_button)
            .push(character_button)
            .push(inventory_button)
            .push(currency_button)
            .push(vehicle_button)
            .push(settings_button),
    )
    .width(Length::Fill)
    .style(ManageTabBarStyle);

    let tab_content = match tab_bar_view {
        SaveTabBarView::属性 => {
            general::view(&mut manage_save_state.save_view_state.general_state)
        }
        SaveTabBarView::角色 => {
            character::view(&mut manage_save_state.save_view_state.character_state)
        }
        SaveTabBarView::背包 => {
            inventory::view(&mut manage_save_state.save_view_state.inventory_state)
        }
        SaveTabBarView::资产 => {
            currency::view(&mut manage_save_state.save_view_state.currency_state)
        }
        SaveTabBarView::载具 => {
            vehicle::view(&mut manage_save_state.save_view_state.vehicle_state)
        }
        SaveTabBarView::设置 => views::settings::view(settings_state),
    };

    let all_contents = Column::new().push(tab_bar).push(tab_content);

    Container::new(all_contents)
        .width(Length::Fill)
        .height(Length::Fill)
}
