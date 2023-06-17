use iced::{
    button, pick_list, text_input, tooltip, Alignment, Button, Column, Container, Length, PickList,
    Row, Text, TextInput, Tooltip,
};

use bl3_save_edit_core::parser::HeaderType;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::labelled_element::LabelledElement;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_input_limited::TextInputLimited;

#[derive(Debug, Default)]
pub struct GeneralState {
    pub filename_input: String,
    pub filename_input_state: text_input::State,
    pub guid_input: String,
    pub guid_input_state: text_input::State,
    pub slot_input: u32,
    pub slot_input_state: text_input::State,
    pub generate_guid_button_state: button::State,
    pub save_type_selector: pick_list::State<HeaderType>,
    pub save_type_selected: HeaderType,
}

#[derive(Debug, Clone)]
pub enum SaveGeneralInteractionMessage {
    Guid(String),
    Slot(u32),
    GenerateGuidPressed,
    SaveTypeSelected(HeaderType),
}

pub fn view(general_state: &mut GeneralState) -> Container<Bl3Message> {
    let file = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "文件",
                    Length::Units(90),
                    Tooltip::new(
                        TextInput::new(
                            &mut general_state.filename_input_state,
                            "1.sav",
                            &general_state.filename_input,
                            |_| InteractionMessage::Ignore,
                        )
                        .font(JETBRAINS_MONO)
                        .padding(10)
                        .size(17)
                        .style(Bl3UiStyle)
                        .into_element(),
                        "不可编辑，可通过更改下面的保存槽进行修改",
                        tooltip::Position::Top,
                    )
                    .gap(10)
                    .padding(10)
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .style(Bl3UiTooltipStyle),
                )
                .spacing(15)
                .width(Length::FillPortion(9))
                .align_items(Alignment::Center),
            )
            .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let save_guid = Container::new(
        Row::new()
            .push(
                LabelledElement::create(
                    "存档 GUI",
                    Length::Units(90),
                    TextInputLimited::new(
                        &mut general_state.guid_input_state,
                        "00000000000000000000000000000000",
                        &general_state.guid_input,
                        500,
                        |s| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::General(
                                    SaveGeneralInteractionMessage::Guid(s),
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
                    &mut general_state.generate_guid_button_state,
                    Text::new("随机").font(JETBRAINS_MONO_BOLD).size(17),
                )
                .on_press(InteractionMessage::ManageSaveInteraction(
                    ManageSaveInteractionMessage::General(
                        SaveGeneralInteractionMessage::GenerateGuidPressed,
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

    let save_slot = Container::new(
        LabelledElement::create(
            "存档槽位",
            Length::Units(90),
            Tooltip::new(
                NumberInput::new(
                    &mut general_state.slot_input_state,
                    general_state.slot_input,
                    1,
                    None,
                    |v| {
                        InteractionMessage::ManageSaveInteraction(
                            ManageSaveInteractionMessage::General(
                                SaveGeneralInteractionMessage::Slot(v),
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
                "插槽必须为1或更大",
                tooltip::Position::Top,
            )
            .gap(10)
            .padding(10)
            .font(JETBRAINS_MONO)
            .size(17)
            .style(Bl3UiTooltipStyle),
        )
        .spacing(15)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let save_type = Container::new(
        LabelledElement::create(
            "存档类型",
            Length::Units(90),
            PickList::new(
                &mut general_state.save_type_selector,
                &HeaderType::SAVE_TYPES[..],
                Some(general_state.save_type_selected),
                |h| {
                    InteractionMessage::ManageSaveInteraction(
                        ManageSaveInteractionMessage::General(
                            SaveGeneralInteractionMessage::SaveTypeSelected(h),
                        ),
                    )
                },
            )
            .font(JETBRAINS_MONO)
            .text_size(17)
            .width(Length::Fill)
            .padding(10)
            .style(Bl3UiStyle)
            .into_element(),
        )
        .spacing(15)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let all_contents = Column::new()
        .push(file)
        .push(save_guid)
        .push(save_slot)
        .push(save_type)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
