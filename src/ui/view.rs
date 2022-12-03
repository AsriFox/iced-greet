use iced::{ Alignment, Length };
use iced::widget::container;

use crate::ui::widgets::{ *, pick_input::* };
use super::{
    // INPUT_ID_USERNAME, 
    INPUT_ID_PASSWORD,
    GreetWindow, GreetWindowState, Message,
};

pub fn view(state: &GreetWindow) -> iced::Element<'_, Message, iced::Renderer<iced::Theme>> {
    let user_image: iced::Element<Message> = 
        if let Some(image) = &state.user_image {
            iced::widget::image(image.clone()).into()
        } else {
            custom_text("").into()
        };

    let header = custom_text(
        match state.state {
            GreetWindowState::EnterUsername => "Enter username",
            GreetWindowState::EnterPassword => "Enter password",
        }
    );

    let input_username = custom_pick_input(
        state.users.clone(),
        state.username.clone(), 
        Message::InputUsernameChanged,
        state.editing_username, 
        Message::ToggleEditingUsername,
    );

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
    )
    .on_submit(Message::InputSubmitted);

    container(
    iced::widget::column![
            container(user_image).width(Length::Units(100)),
            header,
            input_username,
            input_password,
            input_cmd,

            iced::widget::row![
                custom_text_button(
                    "SD",
                    Message::ButtonShutdownPressed,
                ).width(Length::Units(32)),
                custom_text_button(
                    "RE",
                    Message::ButtonRestartPressed,
                ).width(Length::Units(32)),
                custom_text_button(
                    "Login", 
                    Message::InputSubmitted,
                ).width(Length::Fill),
            ]
            .spacing(10),

            custom_text(
                state.status.clone()
            ),
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
