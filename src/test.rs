use std::hash::Hash;

use iced::{Alignment, Length};
use iced::widget::{ container, column as iced_col, row as iced_row };
use crate::query::{sessions::*, users::*};
use crate::ui::widgets::*;
use crate::ui::widgets::pick_input::custom_pick_input;

#[derive(Default)]
pub struct TestWindow {
    editing_username: bool,
    editing_cmd: bool,
    exit: bool,
    username: Option<String>,
    users: Vec<String>,
    password: String,
    session: String,
    sessions: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputUsernameChanged(String),
    InputPasswordChanged(String),
    InputCmdChanged(String),

    ToggleEditingUsername,
    ToggleEditingCmd,

    ButtonExitPressed,
    None,
}

impl iced::Sandbox for TestWindow {
    type Message = Message;

    fn new() -> Self {
        let mut sessions = Vec::<String>::new();
        if let Ok(sessions_xorg) = query_sessions_xorg() {
            sessions = [sessions, sessions_xorg].concat();
        }
        if let Ok(sessions_wayland) = query_sessions_wayland() {
            sessions = [sessions, sessions_wayland].concat();
        }
        let session = 
            if sessions.len() > 0 { 
                sessions[0].clone() 
            } else { String::new() };

        let users = 
            match query_usernames() {
                Ok(users) => users,
                Err(_) => Vec::<String>::new(),
            };
        let username = 
            if users.len() > 0 {
                Some(users[0].clone())
            } else { None };

        Self {
            editing_username: users.is_empty(),
            editing_cmd: sessions.is_empty(),
            username,
            users,
            session,
            sessions,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Iced greeter - test")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark    
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let input_username = custom_pick_input(
            self.users.clone(),
            self.username.clone(),
            Message::InputUsernameChanged,
            self.editing_username,
            Message::ToggleEditingUsername
        );

        let input_password = custom_text_input(
            "Password",
            &self.password,
            Message::InputPasswordChanged,
        )
        .password();

        let input_cmd = custom_pick_input(
            self.sessions.clone(),
            Some(self.session.clone()),
            Message::InputCmdChanged,
            self.editing_cmd,
            Message::ToggleEditingCmd,
        );

        container(
            iced_col![
                container(
                    iced::widget::image(
                        format!("{}/images/blackjack.png", env!("CARGO_MANIFEST_DIR"))
                    ),
                )
                .width(Length::Units(100))
                .style(
                    ImageContainerStyle {}
                ),
                custom_text("Welcome to Iced"),
                input_username,
                input_password,
                input_cmd,

                iced_row![
                    custom_text_button(
                        "SD",
                        Message::None,
                    ).width(Length::Units(32)),
                    custom_text_button(
                        "RE",
                        Message::None,
                    ).width(Length::Units(32)),
                    custom_text_button(
                        "Login",
                        Message::None,
                    ).width(Length::Fill),
                ].spacing(10),

                custom_text(
                    "Status goes here"
                ),

                custom_text_button(
                    "Exit",
                    Message::ButtonExitPressed,
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

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonExitPressed => self.exit = true,
            Message::InputUsernameChanged(value) => {
                self.username = Some(value);
            },
            Message::InputPasswordChanged(value) => {
                self.password = value;
            },
            Message::InputCmdChanged(value) => {
                self.session = value;
            },
            Message::ToggleEditingUsername => {
                self.editing_username = !self.editing_username;
            },
            Message::ToggleEditingCmd => {
                self.editing_cmd = !self.editing_cmd;
            },
            Message::None => {},
        }
    }
}

struct ImageContainerStyle {}

impl container::StyleSheet for ImageContainerStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_width: 10.0,
            border_color: iced::Color::WHITE,
            border_radius: 100.0,
            ..Default::default()
        }
    }
}

impl Into<iced::theme::Container> for ImageContainerStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(self))
    }
}