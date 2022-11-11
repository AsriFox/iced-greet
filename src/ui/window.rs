use iced::{ 
    Alignment, Command, Length, 
    event::{ Event, Status }, 
    keyboard, widget, 
};
use iced::widget::{ button, container, text, text_input };
use once_cell::sync::Lazy;

use crate::greeter::{Greeter, LoginResult};

static INPUT_ID_USERNAME: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static INPUT_ID_PASSWORD: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Default)]
pub struct GreetWindow {
    greeter: Greeter,
    state: GreetWindowState,
    exit: bool,
    
    username: Option<String>,
    password: String,
    status: String,
}

#[derive(Default)]
enum GreetWindowState {
    #[default]
    EnterUsername,
    EnterPassword,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputUsernameChanged(String),
    InputPasswordChanged(String),
    InputCmdChanged(String),
    InputSubmitted,
    ButtonExitPressed,
    TabPressed { shift: bool },
}

impl iced::Application for GreetWindow {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced greeter")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let header = text(
            match self.state {
                GreetWindowState::EnterUsername => "Enter username",
                GreetWindowState::EnterPassword => "Enter password",
            }
        );

        let input_username = text_input(
            "Username", 
            &match &self.username {
                Some(u) => u.clone(),
                None => String::new()
            },
            Message::InputUsernameChanged,
        )
        .padding(5)
        .size(20)
        .id(INPUT_ID_USERNAME.clone())
        .on_submit(Message::InputSubmitted);

        let input_password = text_input(
            "Password", 
            &self.password,
            Message::InputPasswordChanged,
        )
        .padding(5)
        .size(20)
        .password()
        .id(INPUT_ID_PASSWORD.clone())
        .on_submit(Message::InputSubmitted);

        let input_cmd = text_input(
            "Command",
            &self.greeter.cmd,
            Message::InputCmdChanged,
        )
        .padding(5)
        .size(20);

        let button_login = button(
            text("Login"),
        )
        .on_press(Message::InputSubmitted);

        let status_text = text(
            self.status.clone(),
        )
        .size(20);

        let button_exit = button(
            text("Exit"),
        )
        .on_press(Message::ButtonExitPressed);

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
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ButtonExitPressed => self.exit = true,
            Message::InputCmdChanged(value) => {
                self.greeter.cmd = value;
            },
            Message::InputUsernameChanged(value) => {
                self.state = GreetWindowState::EnterUsername;
                self.username = Some(value);
            },
            Message::InputPasswordChanged(value) => {
                if let GreetWindowState::EnterPassword = self.state {
                    self.password = value;
                } else {
                    return text_input::focus(INPUT_ID_USERNAME.clone());
                }
            },
            Message::InputSubmitted => {
                match self.state {
                    GreetWindowState::EnterUsername => {
                        match self.greeter
                            .request_login(self.username.clone().unwrap())
                            .expect("An error occured") {
                            LoginResult::PromptVisible(status) => {
                                self.state = GreetWindowState::EnterPassword;
                                self.status = status;
                                return text_input::focus(INPUT_ID_PASSWORD.clone());
                            },
                            LoginResult::PromptSecret(status) => {
                                self.state = GreetWindowState::EnterPassword;
                                self.status = status;
                                return text_input::focus(INPUT_ID_PASSWORD.clone());
                            },
                            LoginResult::AuthInfo(status) | LoginResult::AuthError(status) => {
                                self.status = status;
                                self.greeter
                                    .respond_to_auth_message(None)
                                    .expect("Failed to respond???");
                            },
                            LoginResult::Success => {
                                self.status = String::from("Starting session?");
                            },
                            LoginResult::Failure => {
                                self.status = String::from("Login failed: user not found");
                            },
                        }
                    },
                    GreetWindowState::EnterPassword => {
                        match self.greeter
                            .respond_to_auth_message(Some(self.password.clone()))
                            .expect("Failed to respond") {
                            LoginResult::Failure => {
                                self.status = String::from("Login failed: wrong password");
                            },
                            LoginResult::Success => {
                                self.status = String::from("Starting session");
                            },
                            LoginResult::PromptVisible(status) 
                            | LoginResult::PromptSecret(status) 
                            | LoginResult::AuthInfo(status) 
                            | LoginResult::AuthError(status) => {
                                self.status = status;
                            },
                        }
                    }
                }
            },
            Message::TabPressed { shift } => {
                return if shift {
                    widget::focus_previous()
                } else {
                    widget::focus_next()
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events_with(
            |event, status| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key_code: keyboard::KeyCode::Tab,
                        modifiers,
                        ..
                    }),
                    Status::Ignored,
                ) => Some(Message::TabPressed { 
                    shift: modifiers.shift(),
                }),
                _ => None,
            }
        )
    }
}