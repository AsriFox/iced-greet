use iced::{
    Command, Element, Container, Column, Row, Text, Alignment, Length,
    button::{ self, Button },
    text_input::{ self, TextInput },
    keyboard::{ self, KeyCode },
};
use crate::greeter::{ Greeter, LoginResult };

#[derive(Default)]
pub struct GreetWindow {
    greeter: Greeter,
    state: GreetWindowState,
    
    input_username: text_input::State,
    username: Option<String>,
    
    input_password: text_input::State,
    password: String,
    
    input_cmd: text_input::State,
    // cmd: String,
    
    status: String,
    
    button_login: button::State,
    
    button_exit: button::State,
    exit: bool,
}

impl GreetWindow {
    fn refocus(&mut self) {
        if self.input_username.is_focused() {
            self.input_username.unfocus();
            self.input_password.focus();
        } else {
            self.input_password.unfocus();
            self.input_username.focus();
        }
    }
}

#[derive(Default)]
enum GreetWindowState {
    #[default]
    EnterUsername,
    EnterPassword,
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(iced_native::Event),
    InputUsernameChanged(String),
    InputPasswordChanged(String),
    InputCmdChanged(String),
    InputSubmitted,
    ButtonExitPressed,
}

impl iced::Application for GreetWindow {
    type Message = self::Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            // Self {
            //     greeter: Greeter::new("startx".to_string()),
            //     input_username: text_input::State::new(),
            //     username: None,
            //     input_password: text_input::State::new(),
            //     password: "".to_string(),
            //     button_login: button::State::new(),
            // },
            Self::default(),
            Command::none(),
        )
    }
    
    fn title(&self) -> String {
        "Iced greeter".to_owned()
    }
    
    fn should_exit(&self) -> bool {
        self.exit
    }
    
    fn view(&mut self) -> Element<'_, Self::Message> {
        let input_username = TextInput::new(
            &mut self.input_username,
            "Username",
            &match &self.username {
                Some(u) => u.clone(),
                None => "".to_string(),
            },
            Message::InputUsernameChanged,
        )
        .padding(5)
        .size(20)
        .on_submit(Message::InputSubmitted);
        
        let input_password = TextInput::new(
            &mut self.input_password,
            "Password",
            &self.password,
            Message::InputPasswordChanged,
        )
        .padding(5)
        .size(20)
        .password()
        .on_submit(Message::InputSubmitted);
        
        let input_cmd = TextInput::new(
            &mut self.input_cmd,
            "Command",
            &self.greeter.cmd,
            Message::InputCmdChanged,
        )
        .padding(5)
        .size(20);
        
        let button_login = Button::new(
            &mut self.button_login,
            Text::new("Login"),
        )
        .on_press(Message::InputSubmitted);
        
        let button_exit = Button::new(
            &mut self.button_exit,
            Text::new("Exit"),
        )
        .on_press(Message::ButtonExitPressed);
        
        let header = match self.state {
            GreetWindowState::EnterUsername => "Enter username",
            GreetWindowState::EnterPassword => "Enter password",
        };
        
        let content = Column::new()
            .spacing(10)
            .width(Length::Units(200))
            .align_items(Alignment::Center)
            .push(Text::new(header).size(20))
            .push(input_username)
            .push(input_password)
            .push(
                Row::new()
                    .spacing(10)
                    .push(input_cmd)
                    .push(button_login)
            )
            .push(Text::new(self.status.clone()).size(20))
            .push(button_exit);
            
        Container::new(content)
            .padding(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputCmdChanged(value) => {
                self.greeter.cmd = value;
            }
            Message::InputUsernameChanged(value) => {
                self.state = GreetWindowState::EnterUsername;
                self.username = Some(value);
            }
            Message::InputPasswordChanged(value) => {
                if let GreetWindowState::EnterPassword = self.state {
                    self.password = value;
                } else { 
                    self.refocus(); 
                }
            }
            Message::InputSubmitted => {
                match self.state {
                    GreetWindowState::EnterUsername => {
                        match self.greeter
                            .request_login(self.username.clone().unwrap())
                            .expect("An error occured") {
                            LoginResult::PromptVisible(status) => { 
                                self.state = GreetWindowState::EnterPassword;
                                self.status = status;
                                self.refocus();
                            }
                            LoginResult::PromptSecret(status) => { 
                                self.state = GreetWindowState::EnterPassword;
                                self.status = status;
                                self.refocus();
                            }
                            LoginResult::AuthInfo(status) | LoginResult::AuthError(status) => { 
                                self.status = status;
                                self.greeter
                                    .respond_to_auth_message(None)
                                    .expect("Failed to respond???");
                            }
                            LoginResult::Success => {
                                self.status = "Starting session".to_string();
                            }
                            LoginResult::Failure => {
                                self.status = "Login failed: user not found".to_string();
                            }
                        }
                    }
                    GreetWindowState::EnterPassword => {
                        match self.greeter
                            .respond_to_auth_message(Some(self.password.clone()))
                            .expect("Failed to respond") {
                            LoginResult::Failure => {
                                self.status = "Login failed: wrong password".to_string();
                            }
                            LoginResult::Success => {
                                self.status = "Starting session".to_string();
                            }
                            LoginResult::PromptVisible(status) 
                            | LoginResult::PromptSecret(status) 
                            | LoginResult::AuthInfo(status) 
                            | LoginResult::AuthError(status) => {
                                self.status = status;
                            }
                        }
                    }
                }
            }
            Message::EventOccured(event) => {
                if let iced_native::Event::Keyboard(key) = event {
                    if let keyboard::Event::KeyPressed { key_code, modifiers: _ } = key {
                        match key_code {
                            KeyCode::Tab => self.refocus(),
                            _ => {}
                        }
                    }
                }
            }
            Message::ButtonExitPressed => self.exit = true,
        }
        Command::none()
    }
    
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events().map(Message::EventOccured)
    }
}
