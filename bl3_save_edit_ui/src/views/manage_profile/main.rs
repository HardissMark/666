use iced::{button, svg, Column, Container, Length, Row};
use strum::Display;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::resources::svgs::{BANK, GENERAL, KEYS, PROFILE, SETTINGS};
use crate::views;
use crate::views::manage_profile::bank::BankState;
use crate::views::manage_profile::general::GeneralState;
use crate::views::manage_profile::keys::KeysState;
use crate::views::manage_profile::profile::ProfileState;
use crate::views::manage_profile::{
    bank, general, keys, profile, ManageProfileInteractionMessage, ManageProfileState,
};
use crate::views::settings::SettingsState;
use crate::views::{tab_bar_button, ManageTabBarStyle};

#[derive(Debug, Default)]
pub struct ProfileViewState {
    tab_bar_state: ProfileTabBarState,
    pub general_state: GeneralState,
    pub profile_state: ProfileState,
    pub keys_state: KeysState,
    pub bank_state: BankState,
}

#[derive(Debug, Default)]
pub struct ProfileTabBarState {
    general_button_state: button::State,
    profile_button_state: button::State,
    keys_button_state: button::State,
    bank_button_state: button::State,
    settings_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum ProfileTabBarInteractionMessage {
    General,
    Profile,
    Keys,
    Bank,
    Settings,
}

#[derive(Debug, Display, PartialEq)]
#[strum(serialize_all = "title_case")]
pub enum ProfileTabBarView {
    属性,
    配置,
    钥匙,
    银行,
    设置,
}

pub fn view<'a>(
    settings_state: &'a mut SettingsState,
    manage_profile_state: &'a mut ManageProfileState,
    tab_bar_view: &ProfileTabBarView,
) -> Container<'a, Bl3Message> {
    let general_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .general_button_state,
        ProfileTabBarView::属性,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::General,
        )),
        svg::Handle::from_memory(GENERAL),
        100,
    );

    let profile_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .profile_button_state,
        ProfileTabBarView::配置,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::Profile,
        )),
        svg::Handle::from_memory(PROFILE),
        100,
    );

    let keys_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .keys_button_state,
        ProfileTabBarView::钥匙,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::Keys,
        )),
        svg::Handle::from_memory(KEYS),
        75,
    );

    let bank_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .bank_button_state,
        ProfileTabBarView::银行,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::Bank,
        )),
        svg::Handle::from_memory(BANK),
        75,
    );

    let settings_button = tab_bar_button(
        &mut manage_profile_state
            .profile_view_state
            .tab_bar_state
            .settings_button_state,
        ProfileTabBarView::设置,
        tab_bar_view,
        InteractionMessage::ManageProfileInteraction(ManageProfileInteractionMessage::TabBar(
            ProfileTabBarInteractionMessage::Settings,
        )),
        svg::Handle::from_memory(SETTINGS),
        105,
    );

    let tab_bar = Container::new(
        Row::new()
            .push(general_button)
            .push(profile_button)
            .push(keys_button)
            .push(bank_button)
            .push(settings_button),
    )
    .width(Length::Fill)
    .style(ManageTabBarStyle);

    let tab_content = match tab_bar_view {
        ProfileTabBarView::属性 => {
            general::view(&mut manage_profile_state.profile_view_state.general_state)
        }
        ProfileTabBarView::配置 => {
            profile::view(&mut manage_profile_state.profile_view_state.profile_state)
        }
        ProfileTabBarView::钥匙 => {
            keys::view(&mut manage_profile_state.profile_view_state.keys_state)
        }
        ProfileTabBarView::银行 => {
            bank::view(&mut manage_profile_state.profile_view_state.bank_state)
        }
        ProfileTabBarView::设置 => views::settings::view(settings_state),
    };

    let all_contents = Column::new().push(tab_bar).push(tab_content);

    Container::new(all_contents)
        .width(Length::Fill)
        .height(Length::Fill)
}
