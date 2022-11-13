use iced::Command;
use iced::widget::{ self, text_input };

use crate::greeter::LoginResult;
use super::{
    INPUT_ID_USERNAME, INPUT_ID_PASSWORD,
    GreetWindow, GreetWindowState, Message,
};

pub fn update(state: &mut GreetWindow, message: Message) -> Command<Message> {
    match message {
        Message::ButtonExitPressed => state.exit = true,
        Message::InputCmdChanged(value) => {
            state.greeter.cmd = value;
        },
        Message::InputUsernameChanged(value) => {
            state.state = GreetWindowState::EnterUsername;
            state.username = Some(value);
        },
        Message::InputPasswordChanged(value) => {
            if let GreetWindowState::EnterPassword = state.state {
                state.password = value;
            } else {
                return text_input::focus(INPUT_ID_USERNAME.clone());
            }
        },
        Message::InputSubmitted => {
            match state.state {
                GreetWindowState::EnterUsername => {
                    match state.greeter
                        .request_login(state.username.clone().unwrap())
                        .expect("An error occured") {
                        LoginResult::PromptVisible(status) => {
                            state.state = GreetWindowState::EnterPassword;
                            state.status = status;
                            return text_input::focus(INPUT_ID_PASSWORD.clone());
                        },
                        LoginResult::PromptSecret(status) => {
                            state.state = GreetWindowState::EnterPassword;
                            state.status = status;
                            return text_input::focus(INPUT_ID_PASSWORD.clone());
                        },
                        LoginResult::AuthInfo(status) | LoginResult::AuthError(status) => {
                            state.status = status;
                            state.greeter
                                .respond_to_auth_message(None)
                                .expect("Failed to respond???");
                        },
                        LoginResult::Success => {
                            state.status = String::from("Starting session?");
                        },
                        LoginResult::Failure => {
                            state.status = String::from("Login failed: user not found");
                        },
                    }
                },
                GreetWindowState::EnterPassword => {
                    match state.greeter
                        .respond_to_auth_message(Some(state.password.clone()))
                        .expect("Failed to respond") {
                        LoginResult::Failure => {
                            state.status = String::from("Login failed: wrong password");
                        },
                        LoginResult::Success => {
                            state.status = String::from("Starting session");
                        },
                        LoginResult::PromptVisible(status) 
                        | LoginResult::PromptSecret(status) 
                        | LoginResult::AuthInfo(status) 
                        | LoginResult::AuthError(status) => {
                            state.status = status;
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