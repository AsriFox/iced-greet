pub mod widgets;
pub mod window;
mod view;
mod update;

use iced::{
    Command,
    event::{ Event, Status },
    keyboard,
};
use iced::widget::text_input;
use once_cell::sync::Lazy;

use crate::greeter::Greeter;

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
pub enum GreetWindowState {
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
        view::view(self)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        update::update(self, message)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events_with(
            |event, status| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key_code: keyboard::KeyCode::Tab,
                        modifiers,
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