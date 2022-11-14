use std::clone::Clone;
use iced::widget::{ row, Row };
use super::*;

const SIDE_BUTTON_WIDTH: iced::Length = iced::Length::Units(24);

pub fn custom_pick_input<'a, Message: Clone + 'a>(
    options: Vec<String>,
    selected: Option<String>,
    on_selected: impl Fn(String) -> Message + 'a,
    is_editing: bool,
    on_toggle_editing: Message,
) -> Row<'a, Message, iced::Renderer<iced::Theme>> {
    if is_editing {
        row![
            custom_text_input(
                "",
                &selected.unwrap(),
                on_selected
            ),
            custom_text_button(
                "V",
                on_toggle_editing,
            ).width(SIDE_BUTTON_WIDTH),
        ]
    } else {
        row![
            custom_pick_list(
                options,
                selected,
                on_selected
            ),
            custom_text_button(
                "...",
                on_toggle_editing,
            ).width(SIDE_BUTTON_WIDTH),
        ]
    }
}