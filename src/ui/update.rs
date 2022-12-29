use iced::Command;
use iced::widget::{ self, text_input };

use crate::greeter::LoginResult;
use super::{
    // INPUT_ID_USERNAME, 
    INPUT_ID_PASSWORD,
    GreetWindow, GreetWindowState, Message,
};

pub fn update(state: &mut GreetWindow, message: Message) -> Command<Message> {
    match message {
        // Message::ButtonExitPressed => state.exit = true,
        Message::ButtonShutdownPressed => {
            if state.exit {
                match system_shutdown::shutdown() {
                    Ok(_) => state.status = "Shutting down...".to_string(),
                    Err(e) => state.status = format!("Error on shutdown: {e}"),
                }
            } else {
                state.status = String::from("Press again to confirm");
                state.exit = true;
            }
        },
        Message::ButtonRestartPressed => {
            if state.exit {
                match system_shutdown::reboot() {
                    Ok(_) => state.status = "Restarting...".to_string(),
                    Err(e) => state.status = format!("Error on reboot: {e}"),
                }
            } else {
                state.status = String::from("Press again to confirm");
                state.exit = true;
            }
        },
        Message::ToggleEditingUsername => state.editing_username = !state.editing_username,
        Message::ToggleEditingCmd => state.editing_cmd = !state.editing_cmd,
        Message::InputCmdChanged(value) => {
            state.greeter.cmd = value;
        },
        Message::InputUsernameChanged(value) => {
            state.username = Some(value);
            if state.editing_username {
                state.state = GreetWindowState::EnterUsername;
            } else {
                return submit_username(state);
            }
        },
        Message::InputPasswordChanged(value) => {
            if let GreetWindowState::EnterPassword = state.state {
                state.password = value;
            } // else {
            //     return text_input::focus(INPUT_ID_USERNAME.clone());
            // }
        },
        Message::InputSubmitted => {
            return match state.state {
                GreetWindowState::EnterUsername => submit_username(state),
                GreetWindowState::EnterPassword => submit_password(state),
            }
        },
        Message::TabPressed { shift } => {
            return if shift {
                widget::focus_previous()
            } else {
                widget::focus_next()
            }
        },
    }
    Command::none()
}

fn submit_username(state: &mut GreetWindow) -> Command<Message> {
    match state.greeter
    .request_login(state.username.clone().unwrap())
    .expect("An error occured") {
        LoginResult::PromptVisible(_) 
        | LoginResult::PromptSecret(_) => {
            state.state = GreetWindowState::EnterPassword;
            state.status = "Enter password".to_string();
            state.user_image = 
                if let Some(username) = &state.username {
                    Some(crate::ui::get_user_image(username.clone()))
                } else { None };
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
    Command::none()
}

fn submit_password(state: &mut GreetWindow) -> Command<Message> {
    match state.greeter
    .respond_to_auth_message(
        Some(state.password.clone())
    ).expect("Failed to respond") {
        LoginResult::Failure => {
            match state.greeter.cancel_login()
            .expect("Failed to cancel login") {
                LoginResult::AuthInfo(status) 
                | LoginResult::AuthError(status) => {
                    state.status = status;
                },
                _ => {}
            }
            state.password = String::new();
            state.state = GreetWindowState::EnterUsername;
            if state.editing_username {
                state.status = String::from("Login failed: wrong username or password");
                state.user_image = None;
            } else {
                state.status = String::from("Login failed: wrong password");
                return submit_username(state);
            }
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
    Command::none()
}