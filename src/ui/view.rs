use iced::{ Alignment, Length };
use iced::widget::container;

use crate::ui::widgets::*;
use super::{
    INPUT_ID_USERNAME, INPUT_ID_PASSWORD,
    GreetWindow, GreetWindowState, Message,
};

pub fn view(state: &GreetWindow) -> iced::Element<'_, Message, iced::Renderer<iced::Theme>> {
    let header = custom_text(
        match state.state {
            GreetWindowState::EnterUsername => "Enter username",
            GreetWindowState::EnterPassword => "Enter password",
        }
    );

    let input_username = custom_text_input(
        "Username", 
        &match &state.username {
            Some(u) => u.clone(),
            None => String::new(),
        },
        Message::InputUsernameChanged,
    )
    .id(INPUT_ID_USERNAME.clone())
    .on_submit(Message::InputSubmitted);

    let input_password = custom_text_input(
        "Password", 
        &state.password,
        Message::InputPasswordChanged,
    )
    .password()
    .id(INPUT_ID_PASSWORD.clone())
    .on_submit(Message::InputSubmitted);

    let input_cmd = custom_text_input(
        "Command",
        &state.greeter.cmd,
        Message::InputCmdChanged,
    );

    let button_login = custom_text_button(
        "Login", 
        Message::InputSubmitted,
    );

    let status_text = custom_text(
        state.status.clone(),
    );

    let button_exit = custom_text_button(
        "Exit",
        Message::ButtonExitPressed,
    );

    container(
    iced::widget::column![
            header,
            input_username,
            input_password,

            iced::widget::row![
                input_cmd,
                button_login,
            ]
            .spacing(10),

            status_text,
            button_exit,
        ]
        .spacing(10)
        .width(Length::Units(200))
        .align_items(Alignment::Center)
    )
    .padding(5)
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}
