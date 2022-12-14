pub mod widgets;
mod view;
mod update;

use iced::{
    Command,
    event::{ Event, Status },
    keyboard,
};
use iced::widget::{ image, text_input };
use once_cell::sync::Lazy;

use crate::greeter::Greeter;
use crate::query::{
    query_all_cmds,
    users::query_usernames,
};

// static INPUT_ID_USERNAME: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static INPUT_ID_PASSWORD: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub struct GreetWindow {
    greeter: Greeter,
    state: GreetWindowState,
    user_image: Option<image::Handle>,
    
    editing_username: bool,
    username: Option<String>,
    users: Vec<String>,

    editing_cmd: bool,
    cmds: Vec<String>,

    password: String,
    status: String,
    exit: bool,
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
    // ButtonExitPressed,
    ButtonShutdownPressed,
    ButtonRestartPressed,

    ToggleEditingUsername,
    ToggleEditingCmd,
    TabPressed { shift: bool },
}

pub fn get_user_image(username: String) -> image::Handle {
    image::Handle::from_path(
        format!("/etc/greetd/faces/{username}.png")
    )
}

impl iced::Application for GreetWindow {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let users = 
            match query_usernames() {
                Ok(users) => users,
                _ => vec![],
            };

        let username = 
            if users.len() > 0 {
                Some(users[0].clone())
            } else { None };

        let user_image = None;
            // if let Some(username) = &username {
            //     Some(get_user_image(username.clone()))
            // } else { None };

        let cmds = query_all_cmds();

        (
            Self {
                greeter: if cmds.len() > 0 {
                    let cmd = cmds.last().unwrap().clone();
                    Greeter::new(cmd)
                } else {
                    Greeter::default()
                },
                state:      Default::default(),
                password:   Default::default(),
                status:     Default::default(),
                editing_username: false,
                editing_cmd: false,
                exit: false,
                user_image,
                username,
                users,
                cmds,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced greeter")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    // fn should_exit(&self) -> bool {
    //     self.exit
    // }

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