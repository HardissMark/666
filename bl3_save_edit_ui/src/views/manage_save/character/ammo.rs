use std::rc::Rc;

use derivative::Derivative;
use iced::alignment::Horizontal;
use iced::{
    button, text_input, tooltip, Alignment, Button, Color, Column, Container, Length, Row, Text,
    Tooltip,
};

use bl3_save_edit_core::bl3_save::ammo::AmmoPool;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::character::{CharacterAmmoMessage, SaveCharacterInteractionMessage};
use crate::views::manage_save::ManageSaveInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct AmmoSetterField {
    name: String,
    text_margin: usize,
    pub ammo_pool: AmmoPool,
    pub input: i32,
    input_state: text_input::State,
    #[derivative(
        Debug = "ignore",
        Default(value = "Rc::new(CharacterAmmoMessage::Grenade)")
    )]
    on_changed: Rc<dyn Fn(i32) -> CharacterAmmoMessage>,
}

impl AmmoSetterField {
    pub fn new<F>(text_margin: usize, ammo_pool: AmmoPool, on_changed: F) -> Self
    where
        F: 'static + Fn(i32) -> CharacterAmmoMessage,
    {
        AmmoSetterField {
            name: ammo_pool.to_string(),
            text_margin,
            ammo_pool,
            on_changed: Rc::new(on_changed),
            ..Default::default()
        }
    }

    pub fn view(&mut self) -> Row<Bl3Message> {
        let on_changed = self.on_changed.clone();
        let minimum = 0;
        let maximum = self.ammo_pool.maximum();

        Row::new()
            .push(
                TextMargin::new(&self.name, self.text_margin)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(220, 220, 220))
                    .width(Length::FillPortion(8)),
            )
            .push(
                Tooltip::new(
                    NumberInput::new(
                        &mut self.input_state,
                        self.input,
                        minimum,
                        Some(maximum),
                        move |v| {
                            InteractionMessage::ManageSaveInteraction(
                                ManageSaveInteractionMessage::Character(
                                    SaveCharacterInteractionMessage::AmmoMessage(on_changed(v)),
                                ),
                            )
                        },
                    )
                    .0
                    .width(Length::FillPortion(3))
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                    format!("数量必须介于 {} 和 {}", minimum, maximum),
                    tooltip::Position::Top,
                )
                .gap(10)
                .padding(10)
                .font(JETBRAINS_MONO)
                .size(17)
                .style(Bl3UiTooltipStyle),
            )
            .width(Length::Fill)
            .align_items(Alignment::Center)
    }
}

#[derive(Debug)]
pub struct AmmoSetter {
    pub sniper: AmmoSetterField,
    pub heavy: AmmoSetterField,
    pub shotgun: AmmoSetterField,
    pub grenade: AmmoSetterField,
    pub smg: AmmoSetterField,
    pub assault_rifle: AmmoSetterField,
    pub pistol: AmmoSetterField,
    max_all_button_state: button::State,
}

impl std::default::Default for AmmoSetter {
    fn default() -> Self {
        Self {
            sniper: AmmoSetterField::new(0, AmmoPool::Sniper, CharacterAmmoMessage::Sniper),
            heavy: AmmoSetterField::new(4, AmmoPool::Heavy, CharacterAmmoMessage::Heavy),
            shotgun: AmmoSetterField::new(0, AmmoPool::Shotgun, CharacterAmmoMessage::Shotgun),
            grenade: AmmoSetterField::new(4, AmmoPool::Grenade, CharacterAmmoMessage::Grenade),
            smg: AmmoSetterField::new(0, AmmoPool::Smg, CharacterAmmoMessage::Smg),
            assault_rifle: AmmoSetterField::new(
                4,
                AmmoPool::Ar,
                CharacterAmmoMessage::AssaultRifle,
            ),
            pistol: AmmoSetterField::new(0, AmmoPool::Pistol, CharacterAmmoMessage::Pistol),
            max_all_button_state: button::State::default(),
        }
    }
}

impl AmmoSetter {
    pub fn view(&mut self) -> Container<Bl3Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Text::new("弹药数量")
                            .font(JETBRAINS_MONO_BOLD)
                            .size(17)
                            .color(Color::from_rgb8(242, 203, 5)),
                    )
                    .padding(10)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
                    .style(Bl3UiStyle),
                )
                .push(
                    Container::new(
                        Column::new()
                            .push(Row::new().push(self.sniper.view()).push(self.heavy.view()))
                            .push(
                                Row::new()
                                    .push(self.shotgun.view())
                                    .push(self.grenade.view()),
                            )
                            .push(
                                Row::new()
                                    .push(self.smg.view())
                                    .push(self.assault_rifle.view()),
                            )
                            .push(
                                Row::new()
                                    .push(self.pistol.view())
                                    .push(Row::new().width(Length::Fill)),
                            )
                            .push(
                                Container::new(
                                    Button::new(
                                        &mut self.max_all_button_state,
                                        Text::new("所有弹药最大数量")
                                            .font(JETBRAINS_MONO_BOLD)
                                            .size(17),
                                    )
                                    .on_press(InteractionMessage::ManageSaveInteraction(
                                        ManageSaveInteractionMessage::Character(
                                            SaveCharacterInteractionMessage::MaxAmmoAmountsPressed,
                                        ),
                                    ))
                                    .padding(10)
                                    .style(Bl3UiStyle)
                                    .into_element(),
                                )
                                .padding(5),
                            )
                            .align_items(Alignment::Center)
                            .spacing(15),
                    )
                    .padding(20)
                    .style(Bl3UiStyle),
                ),
        )
    }
}
