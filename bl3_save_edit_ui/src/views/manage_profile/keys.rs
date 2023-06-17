use iced::{button, text_input, Alignment, Button, Column, Container, Length, Row, Text};

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;

#[derive(Debug, Default)]
pub struct KeysState {
    pub golden_keys_input: i32,
    pub golden_keys_input_state: text_input::State,
    pub diamond_keys_input: i32,
    pub diamond_keys_input_state: text_input::State,
    pub vault_card_1_keys_input: i32,
    pub vault_card_1_keys_input_state: text_input::State,
    pub vault_card_1_chests_input: i32,
    pub vault_card_1_chests_input_state: text_input::State,
    pub vault_card_2_keys_input: i32,
    pub vault_card_2_keys_input_state: text_input::State,
    pub vault_card_2_chests_input: i32,
    pub vault_card_2_chests_input_state: text_input::State,
    pub vault_card_3_keys_input: i32,
    pub vault_card_3_keys_input_state: text_input::State,
    pub vault_card_3_chests_input: i32,
    pub vault_card_3_chests_input_state: text_input::State,
    pub max_golden_keys_button_state: button::State,
    pub max_diamond_keys_button_state: button::State,
    pub max_vault_card_1_keys_button_state: button::State,
    pub max_vault_card_1_chests_button_state: button::State,
    pub max_vault_card_2_keys_button_state: button::State,
    pub max_vault_card_2_chests_button_state: button::State,
    pub max_vault_card_3_keys_button_state: button::State,
    pub max_vault_card_3_chests_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum ProfileKeysInteractionMessage {
    GoldenKeys(i32),
    DiamondKeys(i32),
    VaultCard1Keys(i32),
    VaultCard1Chests(i32),
    VaultCard2Keys(i32),
    VaultCard2Chests(i32),
    VaultCard3Keys(i32),
    VaultCard3Chests(i32),
    MaxGoldenKeysPressed,
    MaxDiamondKeysPressed,
    MaxVaultCard1KeysPressed,
    MaxVaultCard1ChestsPressed,
    MaxVaultCard2KeysPressed,
    MaxVaultCard2ChestsPressed,
    MaxVaultCard3KeysPressed,
    MaxVaultCard3ChestsPressed,
}

pub fn view(keys_state: &mut KeysState) -> Container<Bl3Message> {
    let golden_keys = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "金钥匙",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.golden_keys_input_state,
                        keys_state.golden_keys_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::GoldenKeys(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_golden_keys_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxGoldenKeysPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let diamond_keys = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "钻石钥匙",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.diamond_keys_input_state,
                        keys_state.diamond_keys_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::DiamondKeys(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_diamond_keys_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxDiamondKeysPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let vault_card_1_keys = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "密藏卡片 1 钥匙",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.vault_card_1_keys_input_state,
                        keys_state.vault_card_1_keys_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::VaultCard1Keys(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_vault_card_1_keys_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxVaultCard1KeysPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let vault_card_1_chests = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "密藏卡片 1 宝箱",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.vault_card_1_chests_input_state,
                        keys_state.vault_card_1_chests_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::VaultCard1Chests(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_vault_card_1_chests_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxVaultCard1ChestsPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let vault_card_2_keys = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "密藏卡片 2 钥匙",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.vault_card_2_keys_input_state,
                        keys_state.vault_card_2_keys_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::VaultCard2Keys(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_vault_card_2_keys_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxVaultCard2KeysPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let vault_card_2_chests = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "密藏卡片 2 宝箱",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.vault_card_2_chests_input_state,
                        keys_state.vault_card_2_chests_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::VaultCard2Chests(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_vault_card_2_chests_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxVaultCard2ChestsPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let vault_card_3_keys = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "密藏卡片 3 钥匙",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.vault_card_3_keys_input_state,
                        keys_state.vault_card_3_keys_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::VaultCard3Keys(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_vault_card_3_keys_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxVaultCard3KeysPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let vault_card_3_chests = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "密藏卡片 3 宝箱",
                    Length::Units(170),
                    NumberInput::new(
                        &mut keys_state.vault_card_3_chests_input_state,
                        keys_state.vault_card_3_chests_input,
                        0,
                        None,
                        |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Keys(
                                    ProfileKeysInteractionMessage::VaultCard3Chests(v),
                                ),
                            )
                        },
                    )
                    .0
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .push(
                Button::new(
                    &mut keys_state.max_vault_card_3_chests_button_state,
                    Text::new("上限").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageProfileInteraction(
                    ManageProfileInteractionMessage::Keys(
                        ProfileKeysInteractionMessage::MaxVaultCard3ChestsPressed,
                    ),
                ))
                .padding(10)
                .style(Bl3UiStyle)
                .into_element(),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let all_contents = Column::new()
        .push(golden_keys)
        .push(diamond_keys)
        .push(vault_card_1_keys)
        .push(vault_card_1_chests)
        .push(vault_card_2_keys)
        .push(vault_card_2_chests)
        .push(vault_card_3_keys)
        .push(vault_card_3_chests)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
