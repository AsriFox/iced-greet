use std::clone::Clone;
use iced::{ Renderer, Theme };
use iced::widget::{
    button, Button, 
    pick_list, PickList,
    text, Text,
    text_input, TextInput,
};
pub mod pick_input;

const CUSTOM_PADDING: u16 = 5;
const CUSTOM_FONT_SIZE: u16 = 20;

pub fn custom_text_input<'a, Message: Clone>(
    placeholder: &str,
    value: &str,
    on_change: impl Fn(String) -> Message + 'a,
) -> TextInput<'a, Message, Renderer<Theme>> {
    text_input(placeholder, value, on_change)
        .padding(CUSTOM_PADDING)
        .size(CUSTOM_FONT_SIZE)
}

pub fn custom_text<'a>(
    value: impl ToString,
) -> Text<'a, Renderer<Theme>> {
    text(value)
        .size(CUSTOM_FONT_SIZE)
}

pub fn custom_text_button<'a, Message: Clone>(
    caption: &str,
    msg: Message,
) -> Button<'a, Message, Renderer<Theme>> {
    button(
        text(caption)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
    )
    .on_press(msg)
}

pub fn custom_pick_list<'a, Message: Clone>(
    options: Vec<String>,
    selected: Option<String>,
    on_selected: impl Fn(String) -> Message + 'a,
) -> PickList<'a, String, Message, Renderer<Theme>> {
    pick_list(options, selected, on_selected)
        .placeholder("Session")
        .width(iced::Length::Fill)
}
