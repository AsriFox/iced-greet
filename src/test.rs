use iced::{Alignment, Length};
use iced::widget::container;
use crate::query::{sessions::*, users::*};
use crate::ui::widgets::*;

#[derive(Default)]
pub struct TestWindow {
    exit: bool,
    username: Option<String>,
    password: String,
    session: String,
    sessions: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputUsernameChanged(String),
    InputPasswordChanged(String),
    SessionSelected(String),
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
        Self {
            session: sessions[0].clone(),
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
        let header = custom_text(
            "Welcome to Iced"
        );

        let input_username = custom_text_input(
            "Username",
            &match &self.username {
                Some(u) => u.clone(),
                None => String::new(),
            }, 
            Message::InputUsernameChanged,
        );

        let input_password = custom_text_input(
            "Password",
            &self.password,
            Message::InputPasswordChanged,
        )
        .password();

        let pick_cmd = custom_pick_list(
            self.sessions.clone(),
            self.session.clone(),
            Message::SessionSelected
        );

        let button_login = custom_text_button(
            "Login",
            Message::None,
        );

        let status_text = custom_text(
            "Status goes here"
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
                    pick_cmd,
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

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonExitPressed => self.exit = true,
            Message::InputUsernameChanged(value) => {
                self.username = Some(value);
            },
            Message::InputPasswordChanged(value) => {
                self.password = value;
            },
            Message::SessionSelected(value) => {
                self.session = value;
            },
            Message::None => {},
        }
    }
}